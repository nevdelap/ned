extern crate getopts;
extern crate regex;

use getopts::{Options, ParsingStyle};
use regex::Regex;
use std::{env, path, process};

static OPTS_AND_ARGS: &'static str = "[OPTION]... <PATTERN> [FILE]...";
static DESCRIPTION: &'static str = "
For syntax see: http://rust-lang-nursery.github.io/regex/regex/#syntax";

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = path::Path::new(&args[0])
                      .file_name()
                      .expect("Bug, could't get bin name.")
                      .to_str()
                      .expect("Bug, could't get bin name.");
    let args: Vec<&String> = args.iter().skip(1).collect();

    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::FloatingFrees);
    opts.optflag("r", "replace", "replace matches, may include named groups");
    opts.optflag("",
                 "options",
                 "regex options: i - ignore case, s - single line, . matches newlines, m - \
                  multi-line, ^ and $ match begin and end of each line, x - extended, ignore \
                  whitespace and # comments");
    opts.optflag("i", "in-place", "edit files in place");
    opts.optflag("v", "invert-match", "show non-matching lines");
    opts.optflag("o",
                 "only-matching",
                 "show only the matching part of a line");
    opts.optflag("q", "quiet", "suppress all normal output");
    opts.optflag("V", "version", "output version information and exit");
    opts.optflag("h", "help", "print this help menu and exit");

    let opts = opts;
    let parsed = opts.parse(&args);
    if let Err(err) = parsed {
        println!("{}: {}", &program, err.to_string());
        process::exit(1);
    }

    let matches = parsed.expect("Bug, already checked for a getopts parse error.");
    if matches.free.len() == 0 || matches.opt_present("h") {
        let brief = format!("Usage: {} {}\n{}", program, &OPTS_AND_ARGS, &DESCRIPTION);
        print!("{}", opts.usage(&brief));
        process::exit(1);
    }

    let pattern = &matches.free[0];
    let files: Vec<&String> = matches.free.iter().skip(1).collect();

    let re = Regex::new(&pattern);
    if let Err(err) = re {
        println!("{}: {}", &program, err.to_string());
        process::exit(1);
    }

    // Turn files into a collection of file handles.
    // If there are no files the collection will contain just stdin, stdout.

    let re = re.expect("Bug, already checked for a regex parse error.");
    println!("p: {}", pattern);
    for file in files {
        // Read each file, apply the pattern, write the file.
        println!("f: {}", file);
    }
}
