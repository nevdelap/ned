extern crate ansi_term;
extern crate getopts;
extern crate glob;
extern crate libc;
extern crate regex;
extern crate walkdir;

mod files;
mod ned_error;
mod opts;
mod parameters;
mod source;
#[cfg(test)]
mod tests;

use ansi_term::Colour::{Purple, Red};
use files::Files;
use ned_error::{NedError, NedResult, stderr_write_file_err};
use opts::{make_opts, PROGRAM, usage_full, usage_version};
use parameters::{get_parameters, Parameters};
use source::Source;
use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, stderr, stdin, stdout, Write};
use std::iter::Iterator;
use std::string::String;
use std::{env, process};

fn main() {

    let args = get_args();

    // Output is passed here so that tests can
    // call ned() directly to read the output
    // that would go to stdout.
    let mut output = stdout();
    match ned(&args, &mut output) {
        Ok(exit_code) => process::exit(exit_code),
        Err(err) => {
            // Aside from output exsting so that tests can read the stdout, this uses write()
            // rather than println!() because of this issue...
            // https://github.com/rust-lang/rfcs/blob/master/text/1014-stdout-existential-crisis.md
            let _ = output.write(&format!("{}: {}\n", PROGRAM, err.to_string()).into_bytes());
            process::exit(1)
        }
    }
}

fn get_args() -> Vec<String> {
    let mut args = env::args().skip(1).collect();
    if let Ok(mut default_args) = env::var("NED_DEFAULTS") {
        // This replace of ASCII RS character (what the?) is special - it is for
        // if when using fish shell someone has done "set NED_DEFAULTS -u -c" rather
        // than this "set NED_DEFAULTS '-u -c'" they don't get a cryptic complaint.
        default_args = default_args.replace("\u{1e}", " ");
        let old_args = args;
        args = default_args.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        args.extend(old_args);
    }
    args
}

fn ned(args: &[String], mut output: &mut Write) -> NedResult<i32> {

    let opts = make_opts();
    let parameters = try!(get_parameters(&opts, args));

    if parameters.version {
        let _ = output.write(&format!("{}", usage_version()).into_bytes());
        process::exit(0);
    }

    if parameters.regex.is_none() || parameters.help {
        let _ = output.write(&format!("{}", usage_full(&opts)).into_bytes());
        process::exit(0);
    }

    let found_matches = try!(process_files(&parameters, &mut output));
    Ok(if found_matches {
        0
    } else {
        1
    })
}

fn process_files(parameters: &Parameters, output: &mut Write) -> NedResult<bool> {
    let mut found_matches = false;
    if parameters.stdin {
        let mut source = Source::Stdin(Box::new(stdin()));
        found_matches = try!(process_file(&parameters, None, &mut source, output));
    } else {
        for glob in &parameters.globs {
            for path_buf in &mut Files::new(&parameters, &glob) {
                match OpenOptions::new()
                          .read(true)
                          .write(parameters.replace
                                           .is_some())
                          .open(path_buf.as_path()) {
                    Ok(file) => {
                        let mut source = Source::File(Box::new(file));
                        found_matches |= match process_file(&parameters,
                                                            Some(path_buf.as_path()
                                                                         .to_string_lossy()),
                                                            &mut source,
                                                            output) {
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
            try!(output.flush());
            try!(stderr().flush());
        }
    }
    Ok(found_matches)
}

fn process_file(parameters: &Parameters,
                filename: Option<Cow<str>>,
                source: &mut Source,
                mut output: &mut Write)
                -> NedResult<bool> {
    let purple = Purple;
    let red = Red.bold();

    let filename: Option<Cow<str>> = if let Some(filename) = filename {
        let mut filename = filename.to_string();
        if parameters.colors {
            filename = purple.paint(filename).to_string();
        }
        filename = if parameters.filenames {
            format!("{}\n", filename)
        } else if parameters.whole_files {
            format!("{}:\n", filename)
        } else {
            format!("{}: ", filename)
        };
        Some(Cow::Owned(filename))
    } else {
        None
    };

    let content: String;
    {
        let read: &mut Read = match source {
            &mut Source::Stdin(ref mut read) => read,
            &mut Source::File(ref mut file) => file,
            #[cfg(test)]
            &mut Source::Cursor(ref mut cursor) => cursor,
        };
        let mut buffer = Vec::new();
        let _ = try!(read.read_to_end(&mut buffer));
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

    let re = parameters.regex.clone().expect("Bug, already checked parameters.");
    let mut found_matches = false;

    if let Some(mut replace) = parameters.replace.clone() {
        if parameters.colors {
            replace = red.paint(replace.as_str()).to_string();
        }
        let new_content = re.replace_all(&content, replace.as_str());
        // The replace has to do at least one allocation, so keep the old copy
        // to figure out if there where patches, to save an unnecessary regex match.
        found_matches = new_content != content;
        if parameters.stdout {
            if !parameters.quiet {
                if let Some(ref filename) = filename {
                    try!(output.write(&filename.to_string().into_bytes()));
                }
                try!(output.write(&new_content.into_bytes()));
            }
        } else {
            match source {
                // A better way???
                &mut Source::File(ref mut file) => {
                    try!(file.seek(SeekFrom::Start(0)));
                    try!(file.write(&new_content.into_bytes()));
                }
                #[cfg(test)]
                &mut Source::Cursor(ref mut cursor) => {
                    try!(cursor.seek(SeekFrom::Start(0)));
                    try!(cursor.write(&new_content.into_bytes()));
                }
                _ => {}
            }
        }
    } else if parameters.quiet {
        // Quiet match only is shortcut by the more performant is_match() .
        found_matches = re.is_match(&content);
    } else if parameters.filenames {
        found_matches = re.is_match(&content);
        if found_matches ^ parameters.no_match {
            if let Some(ref filename) = filename {
                try!(output.write(&filename.to_string().into_bytes()));
            }
        }
    } else {
        let mut process_text = |text: &str| -> NedResult<bool> {
            if let Some(ref group) = parameters.group {
                if let Some(captures) = re.captures(&text) {
                    let matched = match group.trim().parse::<usize>() {
                        Ok(index) => captures.at(index),
                        Err(_) => captures.name(group),
                    };
                    if let Some(matched) = matched {
                        let mut matched = matched.to_string();
                        if parameters.colors {
                            matched = re.replace_all(&matched,
                                                     red.paint("$0")
                                                        .to_string()
                                                        .as_str());
                        }
                        if let Some(ref filename) = filename {
                            try!(output.write(&filename.to_string().into_bytes()));
                        }
                        try!(output.write(&matched.to_string().into_bytes()));
                        if !matched.ends_with("\n") {
                            try!(output.write(&"\n".to_string().into_bytes()));
                        }
                    }
                    return Ok(true);
                }
                return Ok(false);
            } else if parameters.no_match {
                let found_matches = re.is_match(&text);
                if !found_matches {
                    if let Some(ref filename) = filename {
                        try!(output.write(&filename.to_string().into_bytes()));
                    }
                    try!(output.write(&text.to_string().into_bytes()));
                    if !text.ends_with("\n") {
                        try!(output.write(&"\n".to_string().into_bytes()));
                    }
                }
                return Ok(found_matches);
            } else if re.is_match(&text) {
                if parameters.only_matches {
                    if let Some(ref filename) = filename {
                        try!(output.write(&filename.to_string().into_bytes()));
                    }
                    for (start, end) in re.find_iter(&text) {
                        let mut matched = text[start..end].to_string();
                        if parameters.colors {
                            matched = re.replace_all(&matched,
                                                     red.paint("$0").to_string().as_str());
                        }
                        try!(output.write(&matched.to_string().into_bytes()));
                        if !matched.ends_with("\n") {
                            try!(output.write(&"\n".to_string().into_bytes()));
                        }
                    }
                } else {
                    if let Some(ref filename) = filename {
                        try!(output.write(&filename.to_string().into_bytes()));
                    }
                    let mut text = text.to_string();
                    if parameters.colors {
                        text = re.replace_all(&text, red.paint("$0").to_string().as_str());
                    }
                    try!(output.write(&text.to_string().into_bytes()));
                    if !text.ends_with("\n") {
                        try!(output.write(&"\n".to_string().into_bytes()));
                    }
                }
                return Ok(true);
            } else {
                return Ok(false);
            }
        };

        if !parameters.whole_files {
            for line in content.lines() {
                found_matches |= try!(process_text(&line));
            }
        } else {
            found_matches = try!(process_text(&content));
        }
    }
    Ok(found_matches)
}
