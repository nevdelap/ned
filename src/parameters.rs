extern crate regex;

use getopts::{Matches, Options};
use glob::Pattern;
use regex::Regex;
use std::iter::Iterator;
use std::string::String;

#[derive(Clone)]
pub struct Parameters {
    pub all: bool,
    pub colors: bool,
    pub exclude_dirs: Vec<Pattern>,
    pub excludes: Vec<Pattern>,
    pub follow: bool,
    pub globs: Vec<String>,
    pub group: Option<String>,
    pub help: bool,
    pub includes: Vec<Pattern>,
    pub no_match: bool,
    pub only_matches: bool,
    pub quiet: bool,
    pub regex: Option<Regex>,
    pub recursive: bool,
    pub replace: Option<String>,
    pub stdin: bool,
    pub stdout: bool,
    pub version: bool,
    pub whole_files: bool,
}

pub fn get_parameters(opts: &Options, args: &[String]) -> Result<Parameters, String> {

    let matches = try!(opts.parse(args).map_err(|err| err.to_string()));

    let globs;
    let regex;

    if matches.opt_present("pattern") {
        let pattern = add_re_options_to_pattern(&matches,
                                                &matches.opt_str("pattern")
                                                        .expect("Bug, already checked that \
                                                                 pattern is present."));
        regex = Some(try!(Regex::new(&pattern).map_err(|err| err.to_string())));
        globs = matches.free.iter().map(|glob| glob.clone()).collect::<Vec<String>>();
    } else if matches.free.len() > 0 {
        let pattern = add_re_options_to_pattern(&matches, &matches.free[0]);
        regex = Some(try!(Regex::new(&pattern).map_err(|err| err.to_string())));
        globs = matches.free.iter().skip(1).map(|glob| glob.clone()).collect::<Vec<String>>();
    } else {
        regex = None;
        globs = matches.free.iter().map(|glob| glob.clone()).collect::<Vec<String>>();
    }

    let mut includes = Vec::<Pattern>::new();
    for include in matches.opt_strs("include") {
        let pattern = try!(Pattern::new(&include).map_err(|err| err.to_string()));
        includes.push(pattern);
    }

    let mut excludes = Vec::<Pattern>::new();
    for exclude in matches.opt_strs("exclude") {
        let pattern = try!(Pattern::new(&exclude).map_err(|err| err.to_string()));
        excludes.push(pattern);
    }

    let mut exclude_dirs = Vec::<Pattern>::new();
    for exclude in matches.opt_strs("exclude-dir") {
        let pattern = try!(Pattern::new(&exclude).map_err(|err| err.to_string()));
        exclude_dirs.push(pattern);
    }

    let replace = matches.opt_str("replace");
    let stdout = matches.opt_present("stdout");
    let stdin = globs.len() == 0 || stdout;

    Ok(Parameters {
        all: matches.opt_present("all"),
        colors: matches.opt_present("colors") && (stdout || replace.is_none()),
        excludes: excludes,
        exclude_dirs: exclude_dirs,
        follow: matches.opt_present("follow"),
        globs: globs,
        group: matches.opt_str("group"),
        help: matches.opt_present("help"),
        includes: includes,
        no_match: matches.opt_present("no-match"),
        only_matches: matches.opt_present("only-matches"),
        quiet: matches.opt_present("quiet"),
        regex: regex,
        recursive: matches.opt_present("recursive"),
        replace: replace,
        stdin: stdin,
        stdout: stdout,
        version: matches.opt_present("version"),
        whole_files: matches.opt_present("whole-files"),
    })
}

fn add_re_options_to_pattern(matches: &Matches, pattern: &str) -> String {
    let mut options: String = "".to_string();
    for option in vec!["i", "s", "m", "x"] {
        if matches.opt_present(&option) {
            options.push_str(&option);
        }
    }
    if options != "" {
        format!("(?{}){}", &options, &pattern)
    } else {
        pattern.to_string()
    }
}
