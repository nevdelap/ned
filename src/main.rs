extern crate getopts;
extern crate regex;

use getopts::{Matches, Options, ParsingStyle};
use regex::Regex;
use std::{env, path, process};
use std::fs::File;
use std::io::{self, Read};
use std::string::String;

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
        print!("{}{}\n\n", opts.usage(&brief), &POST_DESCRIPTION);
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

    let mut files = Vec::<Box<Read>>::new();
    if stdin {
        files.push(Box::new(io::stdin()));
    } else {
        for file_name in file_names {
            match File::open(file_name) {
                Ok(file) => files.push(Box::new(file)),
                Err(err) => {
                    println!("{}: {}", &program, err.to_string());
                    process::exit(1);
                }
            }
        }
    }

    match do_work(re.expect("Bug, already checked for a regex parse error."),
                  matches.opt_present("colors"),
                  matches.opt_present("group"),
                  matches.opt_present("invert-match"),
                  matches.opt_present("matching-lines"),
                  matches.opt_present("matches"),
                  stdin || matches.opt_present("stdout"),
                  &mut files) {
        Ok(_) => process::exit(0),
        Err(err) => {
            println!("{}: {}", &program, err);
            process::exit(1);
        }
    }
}

static OPTS_AND_ARGS: &'static str = "[OPTION]... <PATTERN> [FILE]...";
static PRE_DESCRIPTION: &'static str = "
ned is a bit like grep and a bit like sed. FILEs are text files. For regex
syntax see: http://rust-lang-nursery.github.io/regex/regex/#syntax";
static POST_DESCRIPTION: &'static str = "
Environment:
    NED_DEFAULTS        ned options prepended to the programs arguments";

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
    opts.optflag("i", "ignore-case", "ignore case");
    opts.optflag("s",
                 "single",
                 ". matches newlines, ^ and $ match beginning and end of each file");
    opts.optflag("m",
                 "multi-line",
                 "multi-line, ^ and $ match beginning and end of each line");
    opts.optflag("x", "extended", "ignore whitespace and # comments");
    opts.optflag("l", "matching-lines", "show only matching lines");
    opts.optflag("m", "matches", "show only matches");
    opts.optopt("g",
                "group",
                "show the match group, specified by number or name",
                "GROUP");
    opts.optflag("v", "invert-match", "show non-matching lines");
    opts.optflag("c", "colors", "show matches in color");
    opts.optflag("", "stdout", "output to stdout");
    opts.optflag("q", "quiet", "suppress all normal output");
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

fn do_work(_re: Regex,
           _colors: bool,
           _group: bool,
           _invert: bool,
           _lines: bool,
           stdout: bool,
           _matches: bool,
           files: &mut Vec<Box<Read>>)
           -> Result<(), String> {
    for file in files {
        let mut data = Vec::with_capacity(10240);
        let size = try!(file.read_to_end(&mut data).map_err(|e| e.to_string()));
        if size > 0 {
            let _content = try!(String::from_utf8(data).map_err(|e| e.to_string()));
            if stdout {
                // print
            } else {
                // rewind the file and write over it
            }
        }
    }
    Ok(())
}
