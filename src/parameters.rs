extern crate regex;

use getopts::{Matches, Options};
use glob::Pattern;
use libc;
use ned_error::{NedError, NedResult, StringError};
use regex::Regex;
use std::collections::HashMap;
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Clone)]
pub struct Parameters {
    pub all: bool,
    pub backwards: bool,
    pub colors: bool,
    pub context_after: usize,
    pub context_before: usize,
    pub exclude_dirs: Vec<Pattern>,
    pub excludes: Vec<Pattern>,
    pub file_names_only: bool,
    pub follow: bool,
    pub globs: Vec<String>,
    pub group: Option<String>,
    pub help: bool,
    pub ignore_non_utf8: bool,
    pub includes: Vec<Pattern>,
    pub line_numbers_only: bool,
    pub matches_only: bool,
    pub no_file_names: bool,
    pub no_line_numbers: bool,
    pub no_match: bool,
    pub number: Option<usize>,
    pub quiet: bool,
    pub recursive: bool,
    pub regex: Option<Regex>,
    pub replace: Option<String>,
    pub skip: usize,
    pub stdin: bool,
    pub stdout: bool,
    pub version: bool,
    pub whole_files: bool,
}

impl Parameters {
    pub fn limit_matches(&self) -> bool {
        self.skip > 0 || self.number.is_some()
    }

    pub fn include_match(&self, index: usize, count: usize) -> bool {
        if index >= count || self.skip >= count {
            return false;
        }
        let (skip, number) = if !self.backwards {
            (self.skip, self.number)
        } else {
            if let Some(number) = self.number {
                (
                    if self.skip + number >= count {
                        0
                    } else {
                        count - number - self.skip
                    },
                    Some(number),
                )
            } else {
                (0, Some(count - self.skip))
            }
        };
        index >= skip && if let Some(number) = number {
            index - skip < number
        } else {
            true
        }
    }
}

pub fn get_parameters(opts: &Options, args: &[String]) -> NedResult<Parameters> {
    let matches = try!(opts.parse(args));

    let stdout = matches.opt_present("stdout");
    let replace = convert_escapes(matches.opt_str("replace"));
    // TODO: decide what is the best way to deal with STDOUT_FILENO not
    // being defined in the x86_64-pc-windows-gnu version of libc.
    let isatty = unsafe {
        libc::isatty(/*libc::STDOUT_FILENO as i32*/ 1)
    } != 0;

    // -C --context takes precedence over -B --before and -A --after.
    let mut context_before = try!(parse_opt_str(&matches, "context", 0));
    let context_after;
    if context_before != 0 {
        context_after = context_before;
    } else {
        context_before = try!(parse_opt_str(&matches, "before", 0));
        context_after = try!(parse_opt_str(&matches, "after", 0));
    }

    let mut exclude_dirs = Vec::<Pattern>::new();
    for exclude in matches.opt_strs("exclude-dir") {
        let pattern = try!(Pattern::new(&exclude));
        exclude_dirs.push(pattern);
    }

    let mut excludes = Vec::<Pattern>::new();
    for exclude in matches.opt_strs("exclude") {
        let pattern = try!(Pattern::new(&exclude));
        excludes.push(pattern);
    }

    let mut includes = Vec::<Pattern>::new();
    for include in matches.opt_strs("include") {
        let pattern = try!(Pattern::new(&include));
        includes.push(pattern);
    }

    let whole_files = matches.opt_present("whole-files");

    // TODO: Test combinations of file name and line number options.

    // file_names_only takes precedence over line_numbers_only.
    let file_names_only = matches.opt_present("filenames-only");
    let line_numbers_only =
        !whole_files && !file_names_only && matches.opt_present("line-numbers-only");

    // file_names_only takes precedence over no_file_names.
    let no_file_names = !file_names_only && matches.opt_present("no-filenames");
    let no_line_numbers = !line_numbers_only
        && (file_names_only || !whole_files && matches.opt_present("no-line-numbers"));

    let regex;
    let mut glob_iter: Box<Iterator<Item = _>> = Box::new(matches.free.iter());

    if matches.opt_present("pattern") {
        let pattern = add_re_options_to_pattern(
            &matches,
            &matches.opt_str("pattern").expect(
                "Bug, already checked that pattern \
                 is present.",
            ),
        );
        regex = Some(try!(Regex::new(&pattern)));
    } else if matches.free.len() > 0 {
        let pattern = add_re_options_to_pattern(&matches, &matches.free[0]);
        regex = Some(try!(Regex::new(&pattern)));
        glob_iter = Box::new(glob_iter.skip(1));
    } else {
        regex = None;
    }

    let globs = glob_iter.map(|glob| glob.clone()).collect::<Vec<String>>();

    let number = try!(parse_optional_opt_str(&matches, "number"));
    let skip = try!(parse_opt_str(&matches, "skip", 0));

    let stdin = globs.len() == 0;

    Ok(Parameters {
        all: matches.opt_present("all"),
        backwards: matches.opt_present("backwards"),
        colors: matches.opt_present("colors") && (stdout || replace.is_none()) && isatty,
        context_after: context_after,
        context_before: context_before,
        exclude_dirs: exclude_dirs,
        excludes: excludes,
        file_names_only: file_names_only,
        follow: matches.opt_present("follow"),
        globs: globs,
        group: matches.opt_str("group"),
        help: matches.opt_present("help"),
        ignore_non_utf8: matches.opt_present("ignore-non-utf8"),
        includes: includes,
        line_numbers_only: line_numbers_only,
        matches_only: matches.opt_present("matches-only"),
        no_file_names: no_file_names,
        no_line_numbers: no_line_numbers,
        no_match: matches.opt_present("no-match"),
        number: number,
        quiet: matches.opt_present("quiet"),
        recursive: matches.opt_present("recursive"),
        regex: regex,
        replace: replace,
        skip: skip,
        stdin: stdin,
        stdout: stdout,
        version: matches.opt_present("version"),
        whole_files: whole_files,
    })
}

fn convert_escapes(str: Option<String>) -> Option<String> {
    match str {
        Some(str) => {
            let mut escapes = HashMap::new();
            escapes.insert('\\', '\\');
            escapes.insert('n', '\n');
            escapes.insert('r', '\r');
            escapes.insert('t', '\t');

            let mut result = String::from("");
            // TODO: Figure out how to do this with Peekable.
            // Had trouble figuring out how to use peek while iterating
            // since the for has a mutable borrow of chars while peek is
            // trying to use it inside the for. It didn't result in less
            // lines of code anyway, but would be good to get rid of the
            // collect.
            let chars = str.chars().collect::<Vec<char>>();
            let mut i = 0;
            while i < chars.len() {
                if i < chars.len() - 1 && chars[i] == '\\' {
                    let char = escapes.get(&chars[i + 1]);
                    if char.is_some() {
                        // Escape sequences converted to the character they represent.
                        result.push(*char.unwrap());
                        i += 2;
                        continue;
                    }
                }
                // Unescaped characters unchanged,
                // unrecognised escape sequences unchanged,
                // backslash at end of string unchanged.
                result.push(chars[i]);
                i += 1;
            }
            Some(result)
        }
        None => {
            None
        }
    }
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

// TODO: Figure out how to refactor parse_optional_opt_str() and parse_opt_str() to get rid of the
// duplication. Change it to compose rather than if/match.

fn parse_optional_opt_str<T: FromStr>(matches: &Matches, option: &str) -> NedResult<Option<T>> {
    if let Some(value) = matches.opt_str(option) {
        match value.trim().parse::<T>() {
            Ok(value) => {
                return Ok(Some(value));
            }
            Err(_) => {
                return Err(NedError::ParameterError(StringError {
                    err: format!("invalid value for --{} option", option),
                }));
            }
        };
    }
    Ok(None)
}

fn parse_opt_str<T: FromStr>(matches: &Matches, option: &str, default: T) -> NedResult<T> {
    if let Some(value) = matches.opt_str(option) {
        match value.trim().parse::<T>() {
            Ok(value) => {
                return Ok(value);
            }
            Err(_) => {
                return Err(NedError::ParameterError(StringError {
                    err: format!("invalid value for --{} option", option),
                }));
            }
        };
    }
    Ok(default)
}
