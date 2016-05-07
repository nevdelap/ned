// TODO:
// exit_codes, put back in tests.
// make it to stdin if there are globs.
// remove the Option from walkdirs.
// stderr in Files.
// write a test for files: check behaviour if Files is passed zero globs.
// move opts funcs, constants to their own files.

extern crate ansi_term;
extern crate getopts;
extern crate glob;
extern crate regex;
extern crate walkdir;

mod files;
mod parameters;
mod source;

use ansi_term::Colour::Red;
use files::Files;
use getopts::{Options, ParsingStyle};
use parameters::{get_parameters, Parameters};
use source::Source;
use std::fs::OpenOptions;
#[cfg(test)]
use std::io::Cursor;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::iter::Iterator;
use std::string::String;
use std::{env, path, process};

#[cfg(test)]
mod test_files;
#[cfg(test)]
mod test_general;
#[cfg(test)]
mod test_matches;

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
        println!("{}{}", &VERSION, &LICENSE);
        process::exit(1);
    }

    if parameters.globs.len() == 0 && !parameters.re.is_none() || parameters.help {
        let brief = format!("Usage: {} {}\n{}",
                            program,
                            &OPTS_AND_ARGS,
                            &PRE_DESCRIPTION);
        println!("\n{}{}{}{}",
                 opts.usage(&brief),
                 &POST_DESCRIPTION,
                 &VERSION,
                 &LICENSE);
        process::exit(1);
    }

    let found_matches = try!(process_files(&parameters, &mut output));
    Ok(if found_matches {
        0
    } else {
        1
    })
}

static OPTS_AND_ARGS: &'static str = "[OPTION]... [-p] <PATTERN> [FILE]...";
static PRE_DESCRIPTION: &'static str = "
ned is a bit like grep and a bit like sed, but not really. FILEs are ascii or
utf-8 text files.

For regex syntax see: http://rust-lang-nursery.github.io/regex/regex/#syntax";
static POST_DESCRIPTION: &'static str = "
Environment:
    NED_DEFAULTS        ned options prepended to the program's arguments

Exit codes:
    0                   matches found/replaced
    1                   no matches

Quiet:
    When -q --quiet is  specified ned tests for matches and returns an exit
    code of 0 if a match is found in ANY file. When -a --all is combined with
    quiet it returns an exit code of 0 if a match is found in ALL files. Quiet
    any matches will only read only as many files as needed to find a match.
    Quiet matches are more performant than non-quiet matches.
";
static VERSION: &'static str = "
ned 0.1.0 Copyright (C) 2016 Nev Delap - https://github.com/nevdelap/ned
";
static LICENSE: &'static str = "
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
";

fn make_opts() -> Options {
    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::FloatingFrees);
    opts.optopt("p",
                "pattern",
                "specify pattern, if the option isn't used the pattern must precede the files, \
                 the option allows the pattern to be put after the files for more convenient \
                 editing",
                "PATTERN");
    opts.optopt("r",
                "replace",
                "replace matches, may include named groups",
                "REPLACEMENT");
    // opts.optopt("n", "number", "match/replace N occurrences", "N");
    // opts.optopt("k",
    // "skip",
    // "skip N occurrences before matching/replacing",
    // "N");
    // opts.optflag("b",
    // "backwards",
    // "-n --number and -k --skip options count backwards");
    opts.optflag("i", "ignore-case", "ignore case");
    opts.optflag("s",
                 "single",
                 ". matches newlines, ^ and $ match beginning and end of each file");
    opts.optflag("m",
                 "multiline",
                 "multiline, ^ and $ match beginning and end of each line");
    opts.optflag("l", "line-oriented", "operate on lines");
    opts.optflag("x", "extended", "ignore whitespace and # comments");
    opts.optflag("o", "only-matches", "show only matches");
    opts.optopt("g",
                "group",
                "show the match group, specified by number or name",
                "GROUP");
    opts.optflag("v", "no-match", "show only non-matching");
    opts.optflag("R", "recursive", "recurse");
    opts.optflag("f", "follow", "follow symlinks");
    opts.optmulti("", "include", "match only files that match GLOB", "GLOB");
    opts.optmulti("", "exclude", "skip files matching GLOB", "GLOB");
    opts.optmulti("", "exclude-dir", "skip directories matching GLOB", "GLOB");
    opts.optflag("c", "colors", "show matches in color");
    opts.optflag("", "stdout", "output to stdout");
    opts.optflag("q", "quiet", "suppress all normal output");
    opts.optflag("a", "all", "do not ignore entries starting with .");
    opts.optflag("V", "version", "output version information and exit");
    opts.optflag("h", "help", "print this help menu and exit");
    opts
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

    let mut content;
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

    let mut found_matches = true; // TODO
    if let Some(mut replace) = parameters.replace.clone() {
        if parameters.colors {
            replace = color.paint(replace.as_str()).to_string();
        }
        content = re.replace_all(&content, replace.as_str());
        if parameters.stdout {
            if !parameters.quiet {
                try!(output.write(&content.into_bytes()).map_err(|err| err.to_string()));
            }
        } else {
            match source {
                &mut Source::File(ref mut file) => {
                    try!(file.seek(SeekFrom::Start(0)).map_err(|err| err.to_string()));
                    try!(file.write(&content.into_bytes()).map_err(|err| err.to_string()));
                }
                #[cfg(test)]
                &mut Source::Cursor(ref mut cursor) => {
                    try!(cursor.seek(SeekFrom::Start(0)).map_err(|err| err.to_string()));
                    try!(cursor.write(&content.into_bytes()).map_err(|err| err.to_string()));
                }
                _ => {}
            }
        }
    } else if parameters.quiet {
        // Quiet match only is shortcut by the more performant is_match() .
        found_matches = re.is_match(&content)
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
                }
                return Ok(true); // TODO
            } else if parameters.no_match {
                if !re.is_match(&text) {
                    try!(output.write(&pre.to_string().into_bytes())
                               .map_err(|err| err.to_string()));
                    try!(output.write(&text.to_string().into_bytes())
                               .map_err(|err| err.to_string()));
                    try!(output.write(&post.to_string().into_bytes())
                               .map_err(|err| err.to_string()));
                }
                return Ok(true); // TODO
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
                return Ok(true); // TODO
            } else {
                return Ok(false); // TODO
            }
        };

        if parameters.line_oriented {
            for (line_number, line) in content.lines().enumerate() {
                let pre = if line_number == 0 && line.starts_with("\n") {
                    "\n"
                } else {
                    ""
                };
                try!(process_text(pre, &line, "\n"));
            }
        } else {
            try!(process_text("", &content, ""));
        }
    }
    Ok(found_matches)
}
