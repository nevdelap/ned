extern crate ansi_term;
extern crate getopts;
extern crate glob;
extern crate libc;
extern crate regex;
extern crate time;
extern crate walkdir;

mod colors;
mod files;
mod ned_error;
mod options_with_defaults;
mod opts;
mod parameters;
mod source;
#[cfg(test)]
mod tests;

#[cfg(target_os = "windows")]
use ansi_term::enable_ansi_support;
use ansi_term::Colour::{Purple, Red};
use files::Files;
use ned_error::{stderr_write_file_err, NedError, NedResult};
use options_with_defaults::OptionsWithDefaults;
use opts::{make_opts, usage_brief, usage_full, usage_version};
use parameters::{get_parameters, Parameters};
use regex::{Captures, Match, Regex};
use source::Source;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{stderr, stdin, stdout, Read, Seek, SeekFrom, Write};
use std::iter::Iterator;
use std::string::String;
use std::{env, process};

fn main() {
    // Output is passed here so that tests can
    // call ned() directly to read the output
    // that would go to stdout.
    let mut output = stdout();
    let exit_code = match ned(&mut output, &env::args().skip(1).collect::<Vec<String>>()) {
        Ok(exit_code) => exit_code,
        Err(err) => {
            let _ =
                stderr().write(&format!("{}\n{}\n\n", usage_brief(), err.to_string()).into_bytes());
            1
        }
    };
    let _ = output.flush();
    process::exit(exit_code)
}

fn ned(output: &mut Write, args: &[String]) -> NedResult<i32> {
    let options_with_defaults = OptionsWithDefaults::new(make_opts(), args)?;
    let parameters = get_parameters(&options_with_defaults)?;

    if parameters.version {
        let _ = output.write(&format!("\n{}\n", usage_version()).into_bytes());
        process::exit(0);
    }

    if parameters.help {
        let _ = output
            .write(&format!("\n{}\n", usage_full(&options_with_defaults.get_opts())).into_bytes());
        process::exit(0);
    }

    if parameters.regex.is_none() {
        let _ = stderr().write(&format!("\n{}\n\n", usage_brief()).into_bytes());
        process::exit(1);
    }

    if parameters.colors {
        #[cfg(target_os = "windows")]
        match enable_ansi_support() {
            Ok(_) => {}
            Err(_) => {
                let _ = stderr().write(
                    &"Sadly, colors are not supported in this terminal. ansi_term colors are not supported in Git Bash or Cygwin Terminal. Colors are supported in cmd.exe, PowerShell, the OS X terminal, and all Linux terminals.\n\n"
                        .to_string()
                        .into_bytes(),
                );
                process::exit(1);
            }
        }
    }

    let found_matches = process_files(output, &parameters)?;
    Ok(if found_matches { 0 } else { 1 })
}

fn process_files(output: &mut Write, parameters: &Parameters) -> NedResult<bool> {
    let mut found_matches = false;
    if parameters.stdin {
        let mut source = Source::Stdin(Box::new(stdin()));
        found_matches = process_file(output, parameters, &None, &mut source)?;
    } else {
        for glob in &parameters.globs {
            for path_buf in &mut Files::new(parameters, &glob) {
                match OpenOptions::new()
                    .read(true)
                    .write(parameters.replace.is_some())
                    .open(path_buf.as_path())
                {
                    Ok(file) => {
                        let mut source = Source::File(Box::new(file));
                        let file_name = &Some(path_buf.as_path().to_string_lossy().to_string());
                        found_matches |=
                            match process_file(output, parameters, &file_name, &mut source) {
                                Ok(found_matches) => found_matches,
                                Err(err) => {
                                    stderr_write_file_err(&path_buf, &err);
                                    false
                                }
                            }
                    }
                    Err(err) => stderr_write_file_err(&path_buf, &err),
                }
            }
            if parameters.quiet && found_matches {
                break;
            }
            output.flush()?;
            stderr().flush()?;
        }
    }
    Ok(found_matches)
}

fn process_file(
    output: &mut Write,
    parameters: &Parameters,
    file_name: &Option<String>,
    source: &mut Source,
) -> NedResult<bool> {
    let content: String;
    {
        let read: &mut Read = match source {
            &mut Source::Stdin(ref mut read) => read,
            &mut Source::File(ref mut file) => file,
            #[cfg(test)]
            &mut Source::Cursor(ref mut cursor) => cursor,
        };
        let mut buffer = Vec::new();
        let _ = read.read_to_end(&mut buffer)?;
        match String::from_utf8(buffer) {
            Ok(ref parsed) => {
                content = parsed.to_string();
            }
            Err(err) => {
                if parameters.ignore_non_utf8 {
                    return Ok(false);
                } else {
                    return Err(NedError::from(err));
                }
            }
        }
    }

    let re = parameters
        .regex
        .clone()
        .expect("Bug, already checked parameters.");

    if let Some(mut replacement) = parameters.replace.clone() {
        if parameters.colors {
            replacement = Red.bold().paint(replacement.as_str()).to_string();
        }
        if parameters.case_replacements {
            replacement = replace_case_escape_sequences_with_special_strings(&replacement);
        }
        let (content, found_matches) = replace(parameters, &re, &content, &replacement);
        let content = if parameters.case_replacements {
            replace_case_with_special_strings(&content)
        } else {
            content
        };
        if parameters.stdout {
            if !parameters.quiet {
                write_file_name_and_line_number(output, parameters, file_name, None)?;
                output.write(&content.into_bytes())?;
            }
        } else {
            match source {
                // A better way???
                &mut Source::File(ref mut file) => {
                    if found_matches {
                        file.seek(SeekFrom::Start(0))?;
                        let bytes = &content.into_bytes();
                        file.write(bytes)?;
                        file.set_len(bytes.len() as u64)?;
                    }
                }
                #[cfg(test)]
                &mut Source::Cursor(ref mut cursor) => {
                    cursor.seek(SeekFrom::Start(0))?;
                    cursor.write(&content.into_bytes())?;
                }
                _ => {}
            }
        }
        return Ok(found_matches);
    } else if parameters.file_names_only {
        let found_matches = re.is_match(&content);
        if found_matches ^ parameters.no_match {
            write_file_name_and_line_number(output, parameters, file_name, None)?;
        }
        return Ok(found_matches);
    } else {
        if !parameters.whole_files {
            let mut found_matches = false;
            let context_map = make_context_map(&parameters, &re, &content)?;
            for (index, line) in content.lines().enumerate() {
                let line_number = index + 1;
                found_matches |= process_text(
                    output,
                    parameters,
                    &re,
                    file_name,
                    Some(line_number),
                    line,
                    Some(&context_map),
                )?;
                if parameters.quiet && found_matches {
                    break;
                }
            }
            return Ok(found_matches);
        } else {
            let found_matches =
                process_text(output, parameters, &re, file_name, None, &content, None)?;
            return Ok(found_matches);
        }
    }
}

/// Returns a vector whose capacity equals the number of lines in the file, and whose
/// value is a boolean that indicates whether or not that line should be shown given
/// the -C --context, -B --before, and -A --after options specified in the parameters.
fn make_context_map(parameters: &Parameters, re: &Regex, content: &str) -> NedResult<Vec<bool>> {
    let lines = content.lines().map(str::to_string).collect::<Vec<String>>();
    let mut match_map = Vec::<bool>::with_capacity(lines.len());
    for line in lines {
        match_map.push(is_match_with_number_skip_backwards(parameters, re, &line));
    }
    let mut context_map = match_map.clone();
    for line in 0..context_map.len() {
        if match_map[line] {
            // We can't use std::cmp::max() for this test because the indices are unsigned.
            let start = if line >= parameters.context_before {
                line - parameters.context_before
            } else {
                0usize
            };
            let end = std::cmp::min(match_map.len(), line + parameters.context_after + 1);
            for context_line in start..end {
                context_map[context_line] = true;
            }
        }
    }
    Ok(context_map)
}

fn is_match_with_number_skip_backwards(parameters: &Parameters, re: &Regex, text: &str) -> bool {
    let start_end_byte_indices = re.find_iter(&text).collect::<Vec<Match>>();
    let count = start_end_byte_indices.len();
    for index in 0..count {
        if parameters.include_match(index, count) {
            return true;
        }
    }
    false
}

fn process_text(
    output: &mut Write,
    parameters: &Parameters,
    re: &Regex,
    file_name: &Option<String>,
    line_number: Option<usize>,
    text: &str,
    context_map: Option<&Vec<bool>>,
) -> NedResult<bool> {
    if parameters.quiet && !parameters.limit_matches() && parameters.group.is_none() {
        // Quiet match only is shortcut by the more performant is_match() .
        return Ok(re.is_match(&text));
    }
    if let Some(ref group) = parameters.group {
        // TODO 2: make it respect -n, -k, -b TO TEST
        return Ok(write_groups(
            output,
            parameters,
            &re,
            file_name,
            line_number,
            text,
            group,
        )?);
    } else if parameters.no_match {
        let found_matches = re.is_match(&text);
        if !found_matches {
            write_line(output, parameters, file_name, line_number, &text)?;
        }
        return Ok(found_matches);
    } else if re.is_match(text) {
        if parameters.matches_only {
            if write_matches(output, parameters, &re, file_name, line_number, text)? {
                return Ok(true);
            }
        } else {
            // TODO 4: make it respect -n, -k, -b TO TEST
            // Need to get is found_matches out of this...
            let (text, found_matches) =
                color_matches_with_number_skip_backwards(parameters, re, text);
            if found_matches {
                write_line(output, parameters, file_name, line_number, &text)?;
                return Ok(true);
            }
        }
    }

    if let Some(line_number) = line_number {
        if let Some(context_map) = context_map {
            if context_map.len() > 0 {
                if context_map[line_number - 1] {
                    write_line(output, parameters, file_name, Some(line_number), text)?;
                }
            }
        }
    }
    Ok(false)
}

/// Do a replace_all() or a find_iter() taking into account which of --number, --skip, and
/// --backwards have been specified.
fn replace(parameters: &Parameters, re: &Regex, text: &str, replace: &str) -> (String, bool) {
    let mut found_matches = false;
    let mut new_text;
    if !parameters.limit_matches() {
        found_matches = re.is_match(text);
        new_text = re.replace_all(text, replace).into_owned()
    } else {
        new_text = text.to_string();
        let start_end_byte_indices = re.find_iter(&text).collect::<Vec<Match>>();
        let count = start_end_byte_indices.len();
        // Walk it backwards so that replacements don't invalidate indices.
        for (rev_index, &_match) in start_end_byte_indices.iter().rev().enumerate() {
            let index = count - rev_index - 1;
            if parameters.include_match(index, count) {
                found_matches = true;
                let this_replace = re.replace(_match.as_str(), replace).into_owned();
                new_text = format!(
                    "{}{}{}",
                    // find_iter guarantees that start and end
                    // are at a Unicode code point boundary.
                    unsafe { &new_text.get_unchecked(0.._match.start()) },
                    this_replace,
                    unsafe { &new_text.get_unchecked(_match.end()..new_text.len()) }
                );
            }
        }
    };
    return (new_text, found_matches);
}

enum CaseEscape {
    Upper,
    Lower,
    Initial,
    First,
    End,
}

fn replace_case_escape_sequences_with_special_strings(str: &str) -> String {
    // Convert \U etc. into --nedUned--- etc. so that they should
    // never clash with something in a real file, you'd think!
    Regex::new(r"\\(U|L|I|F|E)")
        .unwrap()
        .replace_all(str, "--ned${1}ned--")
        .into_owned()
}

fn replace_case_with_special_strings(str: &str) -> String {
    let mut escapes = HashMap::new();
    escapes.insert("U", CaseEscape::Upper);
    escapes.insert("L", CaseEscape::Lower);
    escapes.insert("I", CaseEscape::Initial);
    escapes.insert("F", CaseEscape::First);
    escapes.insert("E", CaseEscape::End);
    let escapes = escapes;

    let mut result = String::new();
    let mut last_end = 0;
    let mut last_case_escape = &CaseEscape::End;

    for _match in Regex::new(r"--ned(U|L|I|F|E)ned--").unwrap().find_iter(str) {
        let (start, end) = (_match.start(), _match.end());
        let piece = &str[last_end..start];
        let case_escape = &str[start + 5..end - 5];
        // It must be there because the definition of escapes matches the regex, so unwrap.
        let case_escape = escapes.get(case_escape).unwrap();
        // Apply the last escape to the current piece,
        // append it to the result, clear the current
        // piece, and remember the escape we just found.
        let piece = apply_case_escape(last_case_escape, &piece);
        result.push_str(&piece);
        last_end = end;
        last_case_escape = case_escape;
    }
    // Apply the last escape to the remaining piece
    // when we've hit the end of the string.
    let piece = &str[last_end..];
    let piece = apply_case_escape(last_case_escape, &piece);
    result.push_str(&piece);
    result
}

fn apply_case_escape(case_escape: &CaseEscape, piece: &str) -> String {
    match case_escape {
        CaseEscape::Upper => piece.to_uppercase(),
        CaseEscape::Lower => piece.to_lowercase(),
        CaseEscape::Initial => piece
            .split(' ')
            .map(title_case)
            .collect::<Vec<String>>()
            .join(" "),
        CaseEscape::First => title_case(&piece),
        CaseEscape::End => piece.to_string(),
    }
}

fn title_case(str: &str) -> String {
    let mut result = String::new();
    let str = str.to_lowercase();
    let mut chars = str.chars();
    while let Some(char) = chars.next() {
        if char.is_whitespace() {
            result.push(char);
        } else {
            result.push_str(&char.to_string().to_uppercase());
            break;
        }
    }
    result.push_str(chars.as_str());
    result
}

fn write_line(
    output: &mut Write,
    parameters: &Parameters,
    file_name: &Option<String>,
    line_number: Option<usize>,
    text: &str,
) -> NedResult<()> {
    if !parameters.quiet {
        write_file_name_and_line_number(output, parameters, file_name, line_number)?;
        if !parameters.line_numbers_only && !parameters.quiet {
            output.write(&text.to_string().into_bytes())?;
            write_newline_if_replaced_text_ends_with_newline(output, &text)?;
        }
    }
    Ok(())
}

fn write_groups(
    output: &mut Write,
    parameters: &Parameters,
    re: &Regex,
    file_name: &Option<String>,
    line_number: Option<usize>,
    text: &str,
    group: &str,
) -> NedResult<bool> {
    let mut wrote_file_name = false;
    let mut found_matches = false;
    let captures = re.captures_iter(text).collect::<Vec<Captures>>();
    for (index, capture) in captures.iter().enumerate() {
        if parameters.include_match(index, captures.len()) {
            let _match = match group.trim().parse::<usize>() {
                Ok(index) => capture.get(index),
                Err(_) => capture.name(group),
            };
            if let Some(_match) = _match {
                found_matches = true;
                if !parameters.quiet {
                    let text = color_matches_all(parameters, re, _match.as_str());
                    if !wrote_file_name {
                        write_file_name_and_line_number(
                            output,
                            parameters,
                            file_name,
                            line_number,
                        )?;
                        wrote_file_name = true;
                    }
                    output.write(&text.to_string().into_bytes())?;
                } else {
                    break;
                }
            }
        }
    }
    if !parameters.quiet && found_matches {
        output.write(&"\n".to_string().into_bytes())?;
    }
    Ok(found_matches)
}

/// Write matches taking into account which of --number, --skip, and --backwards have been
/// specified.
fn write_matches(
    output: &mut Write,
    parameters: &Parameters,
    re: &Regex,
    file_name: &Option<String>,
    line_number: Option<usize>,
    text: &str,
) -> NedResult<bool> {
    let mut found_matches = false;
    let mut file_name_written = false;
    let start_end_byte_indices = re.find_iter(text).collect::<Vec<Match>>();
    let count = start_end_byte_indices.len();
    for (index, &_match) in start_end_byte_indices.iter().enumerate() {
        if parameters.include_match(index, count) {
            found_matches = true;
            if !file_name_written {
                write_file_name_and_line_number(output, parameters, file_name, line_number)?;
                file_name_written = true;
            }
            let text = color(parameters, &text[_match.start().._match.end()]);
            if !parameters.quiet {
                output.write(&text.to_string().into_bytes())?;
            } else {
                return Ok(found_matches);
            }
        }
    }
    if file_name_written {
        output.write(&"\n".to_string().into_bytes())?;
    }
    Ok(found_matches)
}

/// Taking into account parameters specifying to display or not display file names and line numbers,
/// write the filename, and line number if they are given, colored if the parameters specify color,
/// and with a newline, colon and newline, or colon, also depending on the specified parameters.
fn write_file_name_and_line_number(
    output: &mut Write,
    parameters: &Parameters,
    file_name: &Option<String>,
    line_number: Option<usize>,
) -> NedResult<()> {
    if !parameters.quiet {
        let mut location = "".to_string();
        if !parameters.no_file_names && !parameters.line_numbers_only {
            if let &Some(ref file_name) = file_name {
                location.push_str(&file_name);
            }
        }
        if !parameters.no_line_numbers && !parameters.file_names_only {
            if let Some(line_number) = line_number {
                if location.len() > 0 {
                    location.push(':');
                }
                location.push_str(&line_number.to_string());
            }
        }
        if location.len() > 0 {
            location.push_str(
                if parameters.file_names_only || parameters.line_numbers_only {
                    "\n"
                } else if parameters.replace.is_some() || parameters.whole_files {
                    ":\n"
                } else {
                    ":"
                },
            );
            if parameters.colors {
                location = Purple.paint(location).to_string();
            }
            output.write(&location.into_bytes())?;
        }
    }
    Ok(())
}

fn write_newline_if_replaced_text_ends_with_newline(
    output: &mut Write,
    text: &str,
) -> NedResult<()> {
    if !text.ends_with("\n") {
        output.write(&"\n".to_string().into_bytes())?;
    }
    Ok(())
}

// TODO: use Cows to reduce allocations in the color*() functions.

fn color_matches_with_number_skip_backwards(
    parameters: &Parameters,
    re: &Regex,
    text: &str,
) -> (String, bool) {
    let (new_text, found_matches) = replace(
        parameters,
        &re,
        text,
        Red.bold().paint("$0").to_string().as_str(),
    );
    if parameters.colors {
        (new_text, found_matches)
    } else {
        (text.to_string(), found_matches)
    }
}

fn color_matches_all(parameters: &Parameters, re: &Regex, text: &str) -> String {
    if parameters.colors {
        re.replace_all(&text, Red.bold().paint("$0").to_string().as_str())
            .into_owned()
    } else {
        text.to_string()
    }
}

/// Color the whole text if --colors has been specified.
fn color(parameters: &Parameters, text: &str) -> String {
    if parameters.colors {
        Red.bold().paint(text).to_string()
    } else {
        text.to_string()
    }
}
