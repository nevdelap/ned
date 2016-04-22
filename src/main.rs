extern crate getopts;
extern crate regex;
extern crate term;

use getopts::{Matches, Options, ParsingStyle};
use regex::Regex;
use std::{env, path, process};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Stdin, Write};
use std::string::String;

enum Source {
    Stdin(Stdin),
    File(File),
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

    let file_names: Vec<&String> = matches.free.iter().skip(1).collect();
    let stdin = file_names.len() == 0;

    println!("TODO: add recursive");
    let mut files = Vec::<Source>::new();
    if stdin {
        files.push(Source::Stdin(io::stdin()));
    } else {
        for file_name in file_names {
            match OpenOptions::new()
                      .read(true)
                      .write(matches.opt_present("replace"))
                      .open(file_name) {
                Ok(file) => files.push((Source::File(file))),
                Err(err) => {
                    println!("{}: {}", &program, err.to_string());
                    process::exit(1);
                }
            }
        }
    }

    match do_work(re.expect("Bug, already checked for a regex parse error."),
                  matches.opt_str("replace"),
                  matches.opt_present("colors"),
                  matches.opt_present("group"),
                  matches.opt_present("invert-match"),
                  matches.opt_present("only-matches"),
                  matches.opt_present("quiet"),
                  matches.opt_present("single"),
                  stdin || matches.opt_present("stdout"),
                  &mut files) {
        Ok(status) => process::exit(status),
        Err(err) => {
            println!("{}: {}", &program, err);
            process::exit(1);
        }
    }
}

static OPTS_AND_ARGS: &'static str = "[OPTION]... <PATTERN> [FILE]...";
static PRE_DESCRIPTION: &'static str = "
ned is a bit like grep and a bit like sed. FILEs are ascii or utf-8 text files.
For regex syntax see: http://rust-lang-nursery.github.io/regex/regex/#syntax";
static POST_DESCRIPTION: &'static str = "
Environment:
    NED_DEFAULTS        ned options prepended to the programs arguments

Exit codes:
    0                   matches found/replaced
    1                   no match

Quiet:
    When -q --quiet is  specified ned tests for matches and returns an exit
    code of 0 if a match is found in any file. When -a --all is combined with
    quiet it returns an exit code of 0 if a match is found in all files.
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
    opts.optflag("r",
                 "recursive",
                 "recurse, only follow symbolic links if they are on the command line");
    opts.optflag("R",
                 "derefence-recursive",
                 "recurse, follow all symbolic links");
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
           replacement: Option<String>,
           colors: bool,
           group: bool,
           invert: bool,
           only_matches: bool,
           quiet: bool,
           single: bool,
           stdout: bool,
           files: &mut Vec<Source>)
           -> Result<i32, String> {
    println!("TODO: Change from Result<i32, String> to Result<i32, NedError>.");
    for file in files {
        let mut data = Vec::with_capacity(10240);
        let size = try!(match file {
            &mut Source::Stdin(ref mut stdin) => {
                stdin.read_to_end(&mut data).map_err(|e| e.to_string())
            }
            &mut Source::File(ref mut file) => {
                file.read_to_end(&mut data).map_err(|e| e.to_string())
            }
        });
        if size > 0 {
            let content = try!(String::from_utf8(data).map_err(|e| e.to_string()));
            if let Some(ref replacement) = replacement {
                if replacement.len() > 0 {
                    let mut content = content;
                    content = re.replace_all(&content, replacement.as_str());
                    if stdout && !quiet {
                        println!("{}", content);
                    } else {
                        if let &mut Source::File(ref mut file) = file {
                            try!(file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string()));
                            try!(file.write(&content.into_bytes()).map_err(|e| e.to_string()));
                        } else {
                            panic!("Bug, should be a File.");
                        }
                    }
                }
            } else if quiet {
                println!("{}", "TODO: implement all/any for all the files.");
                return Ok(if re.is_match(&content) {
                    0
                } else {
                    1
                });
            } else if group {
                println!("TODO: display the indicated group.");
                // regex.captures.
            } else if single {
                println!("TODO: display entire content with colored matches.");
                for (start, end) in re.find_iter(&content) {
                    println!("{}", &content[start..end]);
                }
            } else if only_matches {
                for (start, end) in re.find_iter(&content) {
                    println!("{}", &content[start..end]);
                }
            } else {
                for line in content.lines() {
                    if !colors || invert {
                        if re.is_match(line) ^ invert {
                            println!("{}", line);
                        }
                    } else {
                        println!("TODO: display line with colored matches.");
                        for (start, end) in re.find_iter(&line) {
                            println!("{}", &line[start..end]);
                        }
                    }
                }
            }
        }
    }
    Ok(0)
}
