extern crate getopts;
extern crate regex;
extern crate ansi_term;

use ansi_term::Colour::Red;
use getopts::{Matches, Options, ParsingStyle};
use regex::Regex;
use std::{env, path, process};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::string::String;

enum InOut {
    DifferentFiles((Box<Read>, Box<Write>)),
    SameFile(Box<File>),
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
    let only_matches = matches.opt_present("only-matches");
    let quiet = matches.opt_present("quiet");
    let replace = matches.opt_str("replace");
    let single = matches.opt_present("single");
    let stdout = matches.opt_present("stdout");

    let file_names: Vec<&String> = matches.free.iter().skip(1).collect();
    let stdin = file_names.len() == 0;
    let colors = matches.opt_present("colors") && (stdin || stdout);

    println!("TODO: add recursive");
    let mut files = Vec::<InOut>::new();
    if stdin {
        let input = Box::new(io::stdin());
        let output: Box<Write> = if !quiet {
            Box::new(io::stdout())
        } else {
            Box::new(io::sink())
        };
        files.push(InOut::DifferentFiles((input, output)));
    } else {
        for file_name in file_names {
            match OpenOptions::new()
                      .read(true)
                      .write(matches.opt_present("replace"))
                      .open(file_name) {
                Ok(file) => {
                    files.push(if !quiet && !stdout {
                        InOut::SameFile(Box::new(file))
                    } else {
                        let input = Box::new(file);
                        let output: Box<Write> = if quiet {
                            Box::new(io::sink())
                        } else {
                            Box::new(io::stdout())
                        };
                        InOut::DifferentFiles((input, output))
                    })
                }
                Err(err) => {
                    println!("{}: {}", &program, err.to_string());
                    process::exit(1);
                }
            }
        }
    }

    match do_work(re.expect("Bug, already checked for a regex parse error."),
                  colors,
                  group,
                  invert_match,
                  only_matches,
                  quiet,
                  replace,
                  single,
                  &mut files) {
        Ok(status) => process::exit(status),
        Err(err) => {
            println!("{}: {}", &program, err);
            process::exit(1);
        }
    }
}

static OPTS_AND_ARGS: &'static str = "[OPTION]... [-p] <PATTERN> [FILE]...";
static PRE_DESCRIPTION: &'static str = "
ned is a bit like grep and a bit like sed. FILEs are ascii or utf-8 text files.
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
                 "multi-line",
                 "multi-line, ^ and $ match beginning and end of each line");
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

fn do_work(re: Regex,
           colors: bool,
           group: Option<String>,
           invert_match: bool,
           only_matches: bool,
           quiet: bool,
           replace: Option<String>,
           single: bool,
           files: &mut Vec<InOut>)
           -> Result<i32, String> {
    println!("TODO: Change from Result<i32, String> to Result<i32, NedError>.");

    let mut exit_code = 0;
    let color = Red.bold();

    for file in files {
        let mut content;
        {
            let read: &mut Read = match file {
                &mut InOut::DifferentFiles((ref mut read, ref mut _write)) => read,
                &mut InOut::SameFile(ref mut file) => file,
            };
            let mut buffer = Vec::new();
            let _ = try!(read.read_to_end(&mut buffer).map_err(|e| e.to_string()));
            content = try!(String::from_utf8(buffer).map_err(|e| e.to_string()));
        }

        if let Some(mut replace) = replace.clone() {
            if replace.len() > 0 {
                if colors {
                    replace = color.paint(replace.as_str()).to_string();
                }
                content = re.replace_all(&content, replace.as_str());
                // If content changed exit_code = 1
            }
        } else if quiet {
            // Quiet match only can be shortcut by the more
            // performant is_match() and skip the write.
            println!("{}", "TODO: implement all/any for all the files.");
            if re.is_match(&content) {
                exit_code = 1;
                // if any break
            } else {
                exit_code = 0;
                // if all break
            }
        } else if let Some(ref group) = group {
            if let Some(captures) = re.captures(&content) {
                match group.trim().parse::<usize>() {
                    Ok(index) => {
                        // if there are captures exit_code = 1
                        if let Some(matched) = captures.at(index) {
                            println!("{}", matched);
                        }
                    }
                    Err(_) => {
                        if let Some(matched) = captures.name(group) {
                            println!("{}", matched);
                        }
                    }
                }
            }
        } else if only_matches {
            // if it finds anything exit_code = 1
            for (start, end) in re.find_iter(&content) {
                println!("{}", &content[start..end]);
            }
        } else if !single {
            for line in content.lines() {
                if !colors || invert_match {
                    if re.is_match(line) ^ invert_match {
                        println!("{}", line);
                        exit_code = 1;
                    }
                } else {
                    println!("TODO: display line with colored matches.");
                    // If found anything exit_code = 1;
                    for (start, end) in re.find_iter(&line) {
                        println!("{}", &line[start..end]);
                    }
                }
            }
        } else {
            println!("TODO: display entire content with colored matches.");
            // If found anything exit_code = 1;
            for (start, end) in re.find_iter(&content) {
                println!("{}", &content[start..end]);
            }
        }

        {
            if let &mut InOut::SameFile(ref mut seek) = file {
                try!(seek.seek(SeekFrom::Start(0)).map_err(|e| e.to_string()));
            }
            let write: &mut Write = match file {
                &mut InOut::DifferentFiles((ref mut _read, ref mut write)) => write,
                &mut InOut::SameFile(ref mut file) => file,
            };
            try!(write.write(&content.into_bytes()).map_err(|e| e.to_string()));
        }
    }
    Ok(exit_code)
}
