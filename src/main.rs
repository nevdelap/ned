// TODO:
// exit_codes, put back in tests.
// make it to stdin if there are globs.
// remove the Option from walkdirs.
// stderr in Files.
// write a test for files: check behaviour if Files is passed zero globs.

extern crate ansi_term;
extern crate getopts;
extern crate glob;
extern crate regex;
extern crate walkdir;

mod files;
mod opts;
mod parameters;
mod source;

use ansi_term::Colour::Red;
use files::Files;
use opts::{make_opts, usage_version, usage_full};
use parameters::{get_parameters, Parameters};
use source::Source;
use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::iter::Iterator;
use std::string::String;
use std::{env, path, process};

#[cfg(test)]
mod tests;

fn main() {

    let (program, args) = get_program_and_args();

    // Output is passed here so that tests can
    // call ned() directly to read the output
    // that will go to stdout.
    let mut output = io::stdout();
    match ned(&program, &args, &mut output) {
        Ok(exit_code) => process::exit(exit_code),
        Err(err) => {
            println!("{}: {}", &program, err.to_string());
            process::exit(1)
        }
    }
}

fn get_program_and_args() -> (String, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let program = path::Path::new(&args[0])
                      .file_name()
                      .expect("Bug, could't get bin name.")
                      .to_str()
                      .expect("Bug, could't get bin name.");
    let mut args: Vec<String> = args.iter().skip(1).map(|arg| arg.clone()).collect();
    if let Ok(default_args) = env::var("NED_DEFAULTS") {
        let old_args = args;
        args = default_args.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        args.extend(old_args);;
    }
    (program.to_string(), args)
}

fn ned(program: &str, args: &Vec<String>, mut output: &mut Write) -> Result<i32, String> {

    let opts = make_opts();
    let parameters = try!(get_parameters(&opts, args));

    if parameters.version {
        println!("{}", usage_version());
        process::exit(1);
    }

    if parameters.re.is_none() || parameters.help {
        println!("{}", usage_full(program, &opts));
        process::exit(1);
    }

    let found_matches = try!(process_files(&parameters, &mut output));
    Ok(if found_matches {
        0
    } else {
        1
    })
}

fn process_files(parameters: &Parameters, mut output: &mut Write) -> Result<bool, String> {
    let mut found_matches = false;
    for glob in &parameters.globs {
        for path_buf in &mut Files::new(&parameters, &glob) {
            match OpenOptions::new()
                      .read(true)
                      .write(parameters.replace
                                       .is_some())
                      .open(path_buf.as_path()) {
                Ok(file) => {
                    let mut source = Source::File(Box::new(file));
                    found_matches |= try!(process_file(&parameters, &mut source, output));
                }
                Err(err) => {
                    panic!("Ouch! {}", err);
                    // TODO: write err to stdout
                    // continue;
                }
            }
        }
    }
    try!(output.flush().map_err(|err| err.to_string()));
    Ok(found_matches)
}

fn process_file(parameters: &Parameters,
                source: &mut Source,
                mut output: &mut Write)
                -> Result<bool, String> {
    let color = Red.bold();

    let content;
    {
        let read: &mut Read = match source {
            &mut Source::Stdin(ref mut read) => read,
            &mut Source::File(ref mut file) => file,
            #[cfg(test)]
            &mut Source::Cursor(ref mut cursor) => cursor,
        };
        let mut buffer = Vec::new();
        let _ = try!(read.read_to_end(&mut buffer).map_err(|err| err.to_string()));
        content = try!(String::from_utf8(buffer).map_err(|err| err.to_string()));
    }

    let re = parameters.re.clone().expect("Bug, already checked parameters.");
    let mut found_matches = false;

    if let Some(mut replace) = parameters.replace.clone() {
        if parameters.colors {
            replace = color.paint(replace.as_str()).to_string();
        }
        let new_content = re.replace_all(&content, replace.as_str());
        found_matches = new_content != content;
        if parameters.stdout {
            if !parameters.quiet {
                try!(output.write(&new_content.into_bytes()).map_err(|err| err.to_string()));
            }
        } else {
            match source {
                &mut Source::File(ref mut file) => {
                    try!(file.seek(SeekFrom::Start(0)).map_err(|err| err.to_string()));
                    try!(file.write(&new_content.into_bytes()).map_err(|err| err.to_string()));
                }
                #[cfg(test)]
                &mut Source::Cursor(ref mut cursor) => {
                    try!(cursor.seek(SeekFrom::Start(0)).map_err(|err| err.to_string()));
                    try!(cursor.write(&new_content.into_bytes()).map_err(|err| err.to_string()));
                }
                _ => {}
            }
        }
    } else if parameters.quiet {
        // Quiet match only is shortcut by the more performant is_match() .
        found_matches = re.is_match(&content);
    } else {
        let mut process_text = |pre: &str, text: &str, post: &str| -> Result<bool, String> {
            if let Some(ref group) = parameters.group {
                if let Some(captures) = re.captures(&text) {
                    try!(output.write(&pre.to_string().into_bytes())
                               .map_err(|err| err.to_string()));
                    match group.trim().parse::<usize>() {
                        Ok(index) => {
                            // if there are captures exit_code = 1
                            if let Some(matched) = captures.at(index) {
                                let mut matched = matched.to_string();
                                if parameters.colors {
                                    matched = re.replace_all(&matched,
                                                             color.paint("$0")
                                                                  .to_string()
                                                                  .as_str());
                                }
                                try!(output.write(&matched.to_string().into_bytes())
                                           .map_err(|err| err.to_string()));
                            }
                        }
                        Err(_) => {
                            if let Some(matched) = captures.name(group) {
                                let mut matched = matched.to_string();
                                if parameters.colors {
                                    matched = re.replace_all(&matched,
                                                             color.paint("$0")
                                                                  .to_string()
                                                                  .as_str());
                                }
                                try!(output.write(&matched.to_string().into_bytes())
                                           .map_err(|err| err.to_string()));
                            }
                        }
                    }
                    try!(output.write(&post.to_string().into_bytes())
                               .map_err(|err| err.to_string()));
                    return Ok(true);
                }
                return Ok(false);
            } else if parameters.no_match {
                let found_matches = re.is_match(&text);
                if !found_matches {
                    try!(output.write(&pre.to_string().into_bytes())
                               .map_err(|err| err.to_string()));
                    try!(output.write(&text.to_string().into_bytes())
                               .map_err(|err| err.to_string()));
                    try!(output.write(&post.to_string().into_bytes())
                               .map_err(|err| err.to_string()));
                }
                return Ok(found_matches);
            } else if re.is_match(&text) {
                try!(output.write(&pre.to_string().into_bytes())
                           .map_err(|err| err.to_string()));
                if parameters.only_matches {
                    for (start, end) in re.find_iter(&text) {
                        let mut matched = text[start..end].to_string();
                        if parameters.colors {
                            matched = re.replace_all(&matched,
                                                     color.paint("$0").to_string().as_str());
                        }
                        try!(output.write(&matched.to_string().into_bytes())
                                   .map_err(|err| err.to_string()));
                    }
                } else {
                    let mut text = text.to_string();
                    if parameters.colors {
                        text = re.replace_all(&text, color.paint("$0").to_string().as_str());
                    }
                    try!(output.write(&text.to_string().into_bytes())
                               .map_err(|err| err.to_string()));
                }
                try!(output.write(&post.to_string().into_bytes())
                           .map_err(|err| err.to_string()));
                return Ok(true);
            } else {
                return Ok(false);
            }
        };

        if parameters.line_oriented {
            for (line_number, line) in content.lines().enumerate() {
                let pre = if line_number == 0 && line.starts_with("\n") {
                    "\n"
                } else {
                    ""
                };
                found_matches |= try!(process_text(pre, &line, "\n"));
            }
        } else {
            found_matches = try!(process_text("", &content, ""));
        }
    }
    Ok(found_matches)
}
