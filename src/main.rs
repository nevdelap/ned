//
// ned, https://github.com/nevdelap/ned, main.rs
//
// Copyright 2016-2026 Nev Delap (nevdelap at gmail)
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA
// 02110-1301, USA.
//

mod colors;
mod files;
mod ned_error;
mod options_with_defaults;
mod opts;
mod parameters;
mod source;
#[cfg(test)]
mod tests;

use crate::files::Files;
use crate::ned_error::{NedError, NedResult, stderr_write_file_err};
use crate::options_with_defaults::OptionsWithDefaults;
use crate::opts::{make_opts, usage_brief, usage_full, usage_version};
use crate::parameters::{Parameters, get_parameters};
use crate::source::Source;
use nu_ansi_term::Color;
use regex::{Captures, Match, Regex};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write, stderr, stdin, stdout};
use std::iter::Iterator;
use std::string::String;
use std::{env, process};
use tempfile::NamedTempFile;

fn write_in_place(file: &mut std::fs::File, bytes: &[u8]) -> NedResult<()> {
    file.seek(SeekFrom::Start(0))?;
    file.write_all(bytes)?;
    file.set_len(bytes.len() as u64)?;
    Ok(())
}

fn main() {
    // Output is passed here so that tests can
    // call ned() directly to read the output
    // that would go to stdout.
    let mut output = stdout();
    match ned(&mut output, &env::args().skip(1).collect::<Vec<String>>()) {
        Ok(exit_code) => {
            let _ = output.flush();
            process::exit(exit_code)
        }
        Err(err) => {
            if err.io_error_kind() == Some(std::io::ErrorKind::BrokenPipe) {
                // Stop immediately on BrokenPipe to avoid extra work/flushes.
                process::exit(0)
            } else {
                let mut e = stderr();
                let _ = writeln!(e, "{}\n{}\n", usage_brief(), err);
                let _ = output.flush();
                process::exit(1)
            }
        }
    }
}

fn ned(output: &mut dyn Write, args: &[String]) -> NedResult<i32> {
    let options_with_defaults = OptionsWithDefaults::new(make_opts(), args)?;
    let parameters = get_parameters(&options_with_defaults)?;

    if parameters.version {
        let _ = writeln!(output, "\n{}", usage_version());
        return Ok(0);
    }

    if parameters.help {
        let _ = writeln!(output, "\n{}", usage_full(options_with_defaults.get_opts()));
        return Ok(0);
    }

    if parameters.regex.is_none() {
        let mut e = stderr();
        let _ = write!(e, "\n{}\n\n", usage_brief());
        return Ok(1);
    }

    let found_matches = process_files(output, &parameters)?;
    Ok(if found_matches { 0 } else { 1 })
}

#[cfg(test)]
fn ned_with_defaults(
    output: &mut dyn Write,
    args: &[String],
    defaults: Option<&str>,
) -> NedResult<i32> {
    let options_with_defaults =
        OptionsWithDefaults::new_with_defaults_string(make_opts(), args, defaults)?;
    let parameters = get_parameters(&options_with_defaults)?;

    if parameters.version {
        let _ = writeln!(output, "\n{}", usage_version());
        return Ok(0);
    }

    if parameters.help {
        let _ = writeln!(output, "\n{}", usage_full(options_with_defaults.get_opts()));
        return Ok(0);
    }

    if parameters.regex.is_none() {
        let mut e = stderr();
        let _ = write!(e, "\n{}\n\n", usage_brief());
        return Ok(1);
    }

    let found_matches = process_files(output, &parameters)?;
    Ok(if found_matches { 0 } else { 1 })
}

fn process_files(output: &mut dyn Write, parameters: &Parameters) -> NedResult<bool> {
    let mut found_matches = false;
    if parameters.stdin {
        let mut source = Source::Stdin(Box::new(stdin()));
        found_matches = process_file(output, parameters, &None, &mut source)?;
    } else {
        for glob in &parameters.globs {
            for path_buf in &mut Files::new(parameters, glob) {
                match OpenOptions::new()
                    .read(true)
                    .write(parameters.replace.is_some())
                    .open(path_buf.as_path())
                {
                    Ok(file) => {
                        let mut source = Source::File(file);
                        let file_name = &Some(path_buf.as_path().to_string_lossy().to_string());
                        found_matches |=
                            match process_file(output, parameters, file_name, &mut source) {
                                Ok(found_matches) => found_matches,
                                Err(err) => {
                                    if err.io_error_kind()
                                        == Some(std::io::ErrorKind::BrokenPipe)
                                    {
                                        // Propagate BrokenPipe so top-level can short-circuit.
                                        return Err(err);
                                    } else {
                                        stderr_write_file_err(&path_buf, &err);
                                        false
                                    }
                                }
                            }
                    }
                    Err(err) => stderr_write_file_err(&path_buf, &err),
                }
            }
            if parameters.quiet && found_matches {
                break;
            }
            let _ = output.flush();
            let _ = stderr().flush();
        }
    }
    Ok(found_matches)
}

fn process_file(
    output: &mut dyn Write,
    parameters: &Parameters,
    file_name: &Option<String>,
    source: &mut Source,
) -> NedResult<bool> {
    let re = parameters
        .regex
        .clone()
        .expect("Bug, already checked parameters.");

    if let Some(mut replacement) = parameters.replace.clone() {
        // Replacements operate on whole files; read complete content.
        let read: &mut dyn Read = match source {
            Source::Stdin(read) => read,
            Source::File(file) => file,
            #[cfg(test)]
            Source::Cursor(cursor) => cursor,
        };
        let mut buffer = Vec::new();
        let _ = read.read_to_end(&mut buffer)?;
        let mut content = match String::from_utf8(buffer) {
            Ok(parsed) => parsed,
            Err(err) => {
                if parameters.ignore_non_utf8 {
                    return Ok(false);
                } else {
                    return Err(NedError::from(err));
                }
            }
        };
        if parameters.colors {
            replacement = paint_red_bold(&replacement);
        }
        if parameters.case_replacements {
            replacement = replace_case_escape_sequences_with_special_strings(&replacement);
        }
        let (new_content, found_matches) = replace(parameters, &re, &content, &replacement);
        content = if parameters.case_replacements {
            replace_case_with_special_strings(&new_content)
        } else {
            new_content
        };
        if parameters.stdout {
            if !parameters.quiet {
                write_file_name_and_line_number(output, parameters, file_name, None)?;
                output.write_all(content.as_bytes())?;
            }
        } else {
            // It's not a single match in test.
            #[allow(clippy::single_match)]
            match source {
                // A better way???
                Source::File(file) => {
                    if found_matches {
                        let bytes = content.as_bytes();
                        // Write to a temp file in the same directory and atomically replace.
                        if let Some(file_name) = file_name.as_ref() {
                            let orig_path = std::path::Path::new(file_name);
                            let parent = orig_path.parent().unwrap_or(std::path::Path::new("."));
                            match NamedTempFile::new_in(parent) {
                                Ok(mut tmp) => {
                                    tmp.write_all(bytes)?;
                                    tmp.flush()?;
                                    if let Ok(meta) = std::fs::metadata(orig_path) {
                                        let _ = std::fs::set_permissions(tmp.path(), meta.permissions());
                                    }
                                    match tmp.persist(orig_path) {
                                        Ok(_persisted_file) => {}
                                        Err(_err) => {
                                            // Graceful fallback: modify original file in-place if atomic persist fails.
                                            write_in_place(file, bytes)?;
                                        }
                                    }
                                }
                                Err(_err) => {
                                    // Graceful fallback: if temp file creation fails (e.g., non-writable dir), write in-place.
                                    write_in_place(file, bytes)?;
                                }
                            }
                        } else {
                            // Fallback if path is unavailable (shouldn't happen for regular files).
                            write_in_place(file, bytes)?;
                        }
                    }
                }
                #[cfg(test)]
                Source::Cursor(cursor) => {
                    cursor.seek(SeekFrom::Start(0))?;
                    cursor.write_all(content.as_bytes())?;
                }
                _ => {}
            }
        }
        Ok(found_matches)
    } else if parameters.file_names_only {
        // Prefer streaming for line-mode; read whole file only for whole-files patterns.
        let read: &mut dyn Read = match source {
            Source::Stdin(read) => read,
            Source::File(file) => file,
            #[cfg(test)]
            Source::Cursor(cursor) => cursor,
        };
        if parameters.whole_files {
            let mut buffer = Vec::new();
            let _ = read.read_to_end(&mut buffer)?;
            match String::from_utf8(buffer) {
                Ok(content) => {
                    let found_matches = re.is_match(&content);
                    if found_matches ^ parameters.no_match {
                        write_file_name_and_line_number(output, parameters, file_name, None)?;
                    }
                    Ok(found_matches)
                }
                Err(err) => {
                    if parameters.ignore_non_utf8 {
                        Ok(false)
                    } else {
                        Err(NedError::from(err))
                    }
                }
            }
        } else {
            use std::io::BufRead;
            let mut buf = std::io::BufReader::new(read);
            let mut line = String::new();
            let mut found_matches = false;
            loop {
                line.truncate(0);
                let n = buf.read_line(&mut line)?;
                if n == 0 {
                    break;
                }
                let text = if line.ends_with('\n') {
                    &line[..line.len() - 1]
                } else {
                    &line
                };
                if is_match_with_number_skip_backwards(parameters, &re, text) {
                    found_matches = true;
                    if parameters.quiet {
                        break;
                    }
                }
            }
            if found_matches ^ parameters.no_match {
                write_file_name_and_line_number(output, parameters, file_name, None)?;
            }
            Ok(found_matches)
        }
    } else if !parameters.whole_files {
        // Stream line-mode: build context windows on the fly using before/after counters.
        use std::collections::{HashSet, VecDeque};
        use std::io::BufRead;

        let read: &mut dyn Read = match source {
            Source::Stdin(read) => read,
            Source::File(file) => file,
            #[cfg(test)]
            Source::Cursor(cursor) => cursor,
        };
        let mut buf = std::io::BufReader::new(read);
        let mut line = String::new();
        let mut line_number: usize = 0;
        let mut found_matches = false;
        let mut after_remaining: usize = 0;
        let mut before_buf: VecDeque<(usize, String)> = VecDeque::new();
        let mut printed: HashSet<usize> = HashSet::new();

        loop {
            line.truncate(0);
            let n = buf.read_line(&mut line)?;
            if n == 0 {
                break;
            }
            line_number += 1;
            let text = if line.ends_with('\n') {
                &line[..line.len() - 1]
            } else {
                &line
            };

            if parameters.quiet
                && !parameters.limit_matches()
                && parameters.group.is_none()
                && re.is_match(text)
            {
                return Ok(true);
            }

            let this_line_matches = is_match_with_number_skip_backwards(parameters, &re, text);

            if this_line_matches {
                // Print before-context lines not yet printed.
                if parameters.context_before > 0 {
                    for (ln, ctx) in before_buf.iter() {
                        if !printed.contains(ln) {
                            write_line(output, parameters, file_name, Some(*ln), ctx)?;
                            printed.insert(*ln);
                        }
                    }
                }
                after_remaining = parameters.context_after;
                // Print the matched line via existing logic (colors, groups, matches-only etc.).
                found_matches |= process_text(
                    output,
                    parameters,
                    &re,
                    file_name,
                    Some(line_number),
                    text,
                    None,
                )?;
                printed.insert(line_number);
                if parameters.quiet && found_matches {
                    break;
                }
            } else if after_remaining > 0 {
                write_line(output, parameters, file_name, Some(line_number), text)?;
                printed.insert(line_number);
                after_remaining -= 1;
            } else if parameters.no_match {
                // Show unmatched lines when --no-match is specified.
                write_line(output, parameters, file_name, Some(line_number), text)?;
            }

            // Maintain before buffer window.
            if parameters.context_before > 0 {
                before_buf.push_back((line_number, text.to_string()));
                while before_buf.len() > parameters.context_before {
                    before_buf.pop_front();
                }
            }
        }
        Ok(found_matches)
    } else {
        // Whole-file processing (match/replace operate on complete content).
        let read: &mut dyn Read = match source {
            Source::Stdin(read) => read,
            Source::File(file) => file,
            #[cfg(test)]
            Source::Cursor(cursor) => cursor,
        };
        let mut buffer = Vec::new();
        let _ = read.read_to_end(&mut buffer)?;
        let content = match String::from_utf8(buffer) {
            Ok(parsed) => parsed,
            Err(err) => {
                if parameters.ignore_non_utf8 {
                    return Ok(false);
                } else {
                    return Err(NedError::from(err));
                }
            }
        };
        let found_matches = process_text(output, parameters, &re, file_name, None, &content, None)?;
        Ok(found_matches)
    }
}

fn is_match_with_number_skip_backwards(parameters: &Parameters, re: &Regex, text: &str) -> bool {
    let start_end_byte_indices = re.find_iter(text);
    let count = start_end_byte_indices.count();
    for index in 0..count {
        if parameters.include_match(index, count) {
            return true;
        }
    }
    false
}

fn process_text(
    output: &mut dyn Write,
    parameters: &Parameters,
    re: &Regex,
    file_name: &Option<String>,
    line_number: Option<usize>,
    text: &str,
    context_map: Option<&Vec<bool>>,
) -> NedResult<bool> {
    if parameters.quiet && !parameters.limit_matches() && parameters.group.is_none() {
        // Quiet match only is shortcut by the more performant is_match() .
        return Ok(re.is_match(text));
    }
    if let Some(ref group) = parameters.group {
        // TODO 2: make it respect -n, -k, -b TO TEST
        return write_groups(output, parameters, re, file_name, line_number, text, group);
    } else if parameters.no_match {
        let found_matches = re.is_match(text);
        if !found_matches {
            write_line(output, parameters, file_name, line_number, text)?;
        }
        return Ok(found_matches);
    } else if re.is_match(text) {
        if parameters.matches_only {
            if write_matches(output, parameters, re, file_name, line_number, text)? {
                return Ok(true);
            }
        } else {
            // TODO 4: make it respect -n, -k, -b TO TEST
            // Need to get is found_matches out of this...
            let (colored, found_matches) =
                color_matches_with_number_skip_backwards(parameters, re, text);
            if found_matches {
                write_line(output, parameters, file_name, line_number, colored.as_ref())?;
                return Ok(true);
            }
        }
    }

    if let Some(line_number) = line_number {
        if let Some(context_map) = context_map {
            if !context_map.is_empty() && context_map[line_number - 1] {
                write_line(output, parameters, file_name, Some(line_number), text)?;
            }
        }
    }
    Ok(false)
}

fn paint_red_bold(text: &str) -> String {
    Color::Red.bold().paint(text).to_string()
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
        let start_end_byte_indices = re.find_iter(text).collect::<Vec<Match>>();
        let count = start_end_byte_indices.len();
        // Walk it backwards so that replacements don't invalidate indices.
        for (rev_index, &_match) in start_end_byte_indices.iter().rev().enumerate() {
            let index = count - rev_index - 1;
            if parameters.include_match(index, count) {
                found_matches = true;
                let this_replace = re.replace(_match.as_str(), replace).into_owned();
                // find_iter guarantees that start and end are at a Unicode code point boundary.
                let prefix = &new_text[0.._match.start()];
                let suffix = &new_text[_match.end()..];
                new_text = format!("{}{}{}", prefix, this_replace, suffix);
            }
        }
    };
    (new_text, found_matches)
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
        let case_escape = &escapes[case_escape];
        // Apply the last escape to the current piece,
        // append it to the result, clear the current
        // piece, and remember the escape we just found.
        let piece = apply_case_escape(last_case_escape, piece);
        result.push_str(&piece);
        last_end = end;
        last_case_escape = case_escape;
    }
    // Apply the last escape to the remaining piece
    // when we've hit the end of the string.
    let piece = &str[last_end..];
    let piece = apply_case_escape(last_case_escape, piece);
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
        CaseEscape::First => title_case(piece),
        CaseEscape::End => piece.to_string(),
    }
}

fn title_case(str: &str) -> String {
    let mut result = String::new();
    let str = str.to_lowercase();
    let mut uppercased = false;
    for char in str.chars() {
        if !uppercased && !char.is_whitespace() {
            result.push_str(&char.to_string().to_uppercase());
            uppercased = true;
            continue;
        }
        result.push(char);
    }
    result
}

fn write_line(
    output: &mut dyn Write,
    parameters: &Parameters,
    file_name: &Option<String>,
    line_number: Option<usize>,
    text: &str,
) -> NedResult<()> {
    if !parameters.quiet {
        write_file_name_and_line_number(output, parameters, file_name, line_number)?;
        if !parameters.line_numbers_only && !parameters.quiet {
            output.write_all(text.as_bytes())?;
            write_ensuring_trailing_newline(output, text)?;
        }
    }
    Ok(())
}

fn write_groups(
    output: &mut dyn Write,
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
                    output.write_all(text.as_ref().as_bytes())?;
                } else {
                    break;
                }
            }
        }
    }
    if !parameters.quiet && found_matches {
        output.write_all(b"\n")?;
    }
    Ok(found_matches)
}

/// Write matches taking into account which of --number, --skip, and --backwards have been
/// specified.
fn write_matches(
    output: &mut dyn Write,
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
            let colored = color(parameters, &text[_match.start().._match.end()]);
            if !parameters.quiet {
                output.write_all(colored.as_ref().as_bytes())?;
            } else {
                return Ok(found_matches);
            }
        }
    }
    if file_name_written {
        output.write_all(b"\n")?;
    }
    Ok(found_matches)
}

/// Taking into account parameters specifying to display or not display file names and line numbers,
/// write the filename, and line number if they are given, colored if the parameters specify color,
/// and with a newline, colon and newline, or colon, also depending on the specified parameters.
fn write_file_name_and_line_number(
    output: &mut dyn Write,
    parameters: &Parameters,
    file_name: &Option<String>,
    line_number: Option<usize>,
) -> NedResult<()> {
    if !parameters.quiet {
        let mut location = "".to_string();
        if !parameters.no_file_names && !parameters.line_numbers_only {
            if let Some(file_name) = file_name {
                location.push_str(file_name);
            }
        }
        if !parameters.no_line_numbers && !parameters.file_names_only {
            if let Some(line_number) = line_number {
                if !location.is_empty() {
                    location.push(':');
                }
                location.push_str(&line_number.to_string());
            }
        }
        if !location.is_empty() {
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
                location = Color::Purple.paint(location).to_string();
            }
            output.write_all(location.as_bytes())?;
        }
    }
    Ok(())
}

fn write_ensuring_trailing_newline(output: &mut dyn Write, text: &str) -> NedResult<()> {
    if !text.ends_with('\n') {
        output.write_all(b"\n")?;
    }
    Ok(())
}

// TODO: use Cows to reduce allocations in the color*() functions.

fn color_matches_with_number_skip_backwards<'a>(
    parameters: &Parameters,
    re: &Regex,
    text: &'a str,
) -> (Cow<'a, str>, bool) {
    if parameters.colors {
        let (new_text, found_matches) =
            replace(parameters, re, text, paint_red_bold("$0").as_str());
        (Cow::Owned(new_text), found_matches)
    } else {
        let found_matches = is_match_with_number_skip_backwards(parameters, re, text);
        (Cow::Borrowed(text), found_matches)
    }
}

fn color_matches_all<'a>(parameters: &Parameters, re: &Regex, text: &'a str) -> Cow<'a, str> {
    if parameters.colors {
        re.replace_all(text, |caps: &Captures| {
            // Color the actual matched text via a closure to avoid building a static replacement.
            let m = caps.get(0).expect("match exists").as_str();
            paint_red_bold(m)
        })
    } else {
        Cow::Borrowed(text)
    }
}

/// Color the whole text if --colors has been specified.
fn color<'a>(parameters: &Parameters, text: &'a str) -> Cow<'a, str> {
    if parameters.colors {
        Cow::Owned(paint_red_bold(text))
    } else {
        Cow::Borrowed(text)
    }
}
