extern crate getopts;
extern crate regex;
extern crate ansi_term;

use ansi_term::Colour::Red;
use getopts::{Matches, Options, ParsingStyle};
use regex::Regex;
use std::{env, path, process};
use std::fs::{File, OpenOptions};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::string::String;

#[cfg(test)]
mod tests;

enum Source {
    Stdin(Box<Read>),
    File(Box<File>),
    Cursor(Box<Cursor<Vec<u8>>>),
}

fn main() {
    let (program, args) = get_program_and_args();
    let opts = make_opts();

    let parsed = opts.parse(&args);
    if let Err(err) = parsed {
        println!("{}: {}", &program, err.to_string());
        process::exit(1);
    }

    let matches = parsed.expect("Bug, already checked for a getopts parse error.");
    if matches.free.len() == 0 || matches.opt_present("h") {
        let brief = format!("Usage: {} {}\n{}",
                            program,
                            &OPTS_AND_ARGS,
                            &PRE_DESCRIPTION);
        print!("{}{}\n", opts.usage(&brief), &POST_DESCRIPTION);
        process::exit(1);
    }

    let options = make_options(&matches);

    let mut pattern: String = matches.free[0].clone();
    if options != "" {
        pattern = format!("(?{}){}", &options, &pattern);
    }
    let pattern = pattern;

    let re = Regex::new(&pattern);
    if let Err(err) = re {
        println!("{}: {}", &program, err.to_string());
        process::exit(1);
    }

    let group = matches.opt_str("group");
    let invert_match = matches.opt_present("invert-match");
    let line_oriented = matches.opt_present("line_oriented");
    let only_matches = matches.opt_present("only-matches");
    let quiet = matches.opt_present("quiet");
    let replace = matches.opt_str("replace");
    let stdout = matches.opt_present("stdout");

    let file_names: Vec<&String> = matches.free.iter().skip(1).collect();
    let stdin = file_names.len() == 0;
    let colors = matches.opt_present("colors") && (stdin || stdout);

    println!("TODO: add recursive");
    let mut files = Vec::<Source>::new();
    if stdin {
        files.push(Source::Stdin(Box::new(io::stdin())));
    } else {
        for file_name in file_names {
            match OpenOptions::new()
                      .read(true)
                      .write(matches.opt_present("replace"))
                      .open(file_name) {
                Ok(file) => files.push(Source::File(Box::new(file))),
                Err(err) => {
                    println!("{}: {}", &program, err.to_string());
                    process::exit(1);
                }
            }
        }
    }

    let mut output = io::stdout();
    match process_files(re.expect("Bug, already checked for a regex parse error."),
                        colors,
                        &group,
                        invert_match,
                        line_oriented,
                        only_matches,
                        quiet,
                        &replace,
                        stdout,
                        &mut files,
                        &mut output) {
        Ok(status) => process::exit(status),
        Err(err) => {
            println!("{}: {}", &program, err);
            process::exit(1);
        }
    }
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

fn get_program_and_args() -> (String, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let program = path::Path::new(&args[0])
                      .file_name()
                      .expect("Bug, could't get bin name.")
                      .to_str()
                      .expect("Bug, could't get bin name.");
    let args: Vec<String> = args.iter().skip(1).map(|arg| arg.clone()).collect();
    (program.to_string(), args)
}

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
    opts.optopt("n", "number", "match/replace N occurrences", "N");
    opts.optopt("k",
                "skip",
                "skip N occurrences before matching/replacing",
                "N");
    opts.optflag("b", "backwards", "-n and -k options count backwards");
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
    opts.optflag("v", "invert-match", "show non-matching lines");
    opts.optflag("r", "recursive", "recurse, follow all symbolic links");
    opts.optflag("",
                 "cautious-recursive",
                 "recurse, only follow symbolic links if they are on the command line");
    opts.optflag("c", "colors", "show matches in color");
    opts.optflag("", "stdout", "output to stdout");
    opts.optflag("q", "quiet", "suppress all normal output");
    opts.optflag("a",
                 "all",
                 "exit code is 0 if all files match, default is \
                 exit code is 0 if any file matches");
    opts.optflag("V", "version", "output version information and exit");
    opts.optflag("h", "help", "print this help menu and exit");
    opts
}

fn make_options(matches: &Matches) -> String {
    let mut options: String = "".to_string();
    for option in vec!["i", "s", "m", "x"] {
        if matches.opt_present(&option) {
            options.push_str(&option);
        }
    }
    options
}

fn process_files(re: Regex,
                 colors: bool,
                 group: &Option<String>,
                 invert_match: bool,
                 line_oriented: bool,
                 only_matches: bool,
                 quiet: bool,
                 replace: &Option<String>,
                 stdout: bool,
                 files: &mut Vec<Source>,
                 mut output: &mut Write)
                 -> Result<i32, String> {
    println!("TODO: Change from Result<i32, String> to Result<i32, NedError>.");

    let mut exit_code = 0;
    for mut file in files {
        exit_code = try!(process_file(&re,
                                      colors,
                                      &group,
                                      invert_match,
                                      line_oriented,
                                      only_matches,
                                      quiet,
                                      &replace,
                                      stdout,
                                      &mut file,
                                      &mut output));
    }
    Ok(exit_code)
}

fn process_file(re: &Regex,
                colors: bool,
                group: &Option<String>,
                invert_match: bool,
                line_oriented: bool,
                only_matches: bool,
                quiet: bool,
                replace: &Option<String>,
                stdout: bool,
                file: &mut Source,
                mut output: &mut Write)
                -> Result<i32, String> {

    let mut exit_code = 0;
    let color = Red.bold();

    let content;
    {
        let read: &mut Read = match file {
            &mut Source::Stdin(ref mut read) => read,
            &mut Source::File(ref mut file) => file,
            &mut Source::Cursor(ref mut cursor) => cursor,
        };
        let mut buffer = Vec::new();
        let _ = try!(read.read_to_end(&mut buffer).map_err(|e| e.to_string()));
        content = try!(String::from_utf8(buffer).map_err(|e| e.to_string()));
    }

    if let Some(mut replace) = replace.clone() {
        if stdout && colors {
            replace = color.paint(replace.as_str()).to_string();
        }
        let new_content = re.replace_all(&content, replace.as_str());
        if stdout {
            try!(output.write(&new_content.into_bytes()).map_err(|e| e.to_string()));
        } else {
            // Better way to do this?
            match file {
                &mut Source::File(ref mut file) => {
                    try!(file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string()));
                    try!(file.write(&new_content.into_bytes()).map_err(|e| e.to_string()));
                }
                &mut Source::Cursor(ref mut file) => {
                    try!(file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string()));
                    try!(file.write(&new_content.into_bytes()).map_err(|e| e.to_string()));
                }
                _ => {}
            }
        }
        return Ok(exit_code);
    }

    if quiet {
        // Quiet match only is shortcut by the more performant is_match() .
        exit_code = if re.is_match(&content) {
            0
        } else {
            1
        };
    } else {

        let mut process_text = |text: &str| -> Result<i32, String> {
            if let &Some(ref group) = group {
                if let Some(captures) = re.captures(&text) {
                    match group.trim().parse::<usize>() {
                        Ok(index) => {
                            // if there are captures exit_code = 1
                            if let Some(matched) = captures.at(index) {
                                try!(output.write(&matched.to_string().into_bytes()).map_err(|e| e.to_string()));
                            }
                        }
                        Err(_) => {
                            if let Some(matched) = captures.name(group) {
                                try!(output.write(&matched.to_string().into_bytes()).map_err(|e| e.to_string()));
                            }
                        }
                    }
                }
            } else if only_matches {
                for (start, end) in re.find_iter(&text) {
                    try!(output.write(&text[start..end].to_string().into_bytes()).map_err(|e| e.to_string()));
                }
            } else {
                // Print colored matches within matches.
                for (start, end) in re.find_iter(&text) {
                    try!(output.write(&text[start..end].to_string().into_bytes()).map_err(|e| e.to_string()));
                }
            }
            Ok(0)
        };

        if line_oriented {
            for line in content.lines() {
                try!(process_text(&line));
            }
        } else {
            try!(process_text(&content));
        }
    }

    Ok(exit_code)
}
