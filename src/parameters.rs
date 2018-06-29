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
    pub case_replacements: bool,
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
    let matches = opts.parse(args)?;

    let stdout = matches.opt_present("stdout");
    let replace = convert_escapes(matches.opt_str("replace"));
    // TODO: decide what is the best way to deal with STDOUT_FILENO not
    // being defined in the x86_64-pc-windows-gnu version of libc.
    let isatty = unsafe {
        libc::isatty(/*libc::STDOUT_FILENO as i32*/ 1)
    } != 0;

    // -C --context takes precedence over -B --before and -A --after.
    let mut context_before = parse_opt_str(&matches, "context", Some(0))?.expect("The default is a Some.");
    let context_after;
    if context_before != 0 {
        context_after = context_before;
    } else {
        context_before = parse_opt_str(&matches, "before", Some(0))?.expect("The default is a Some.");
        context_after = parse_opt_str(&matches, "after", Some(0))?.expect("The default is a Some.");
    }

    let mut exclude_dirs = Vec::<Pattern>::new();
    for exclude in matches.opt_strs("exclude-dir") {
        let pattern = Pattern::new(&exclude)?;
        exclude_dirs.push(pattern);
    }

    let mut excludes = Vec::<Pattern>::new();
    for exclude in matches.opt_strs("exclude") {
        let pattern = Pattern::new(&exclude)?;
        excludes.push(pattern);
    }

    let mut includes = Vec::<Pattern>::new();
    for include in matches.opt_strs("include") {
        let pattern = Pattern::new(&include)?;
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
        regex = Some(Regex::new(&pattern)?);
    } else if matches.free.len() > 0 {
        let pattern = add_re_options_to_pattern(&matches, &matches.free[0]);
        regex = Some(Regex::new(&pattern)?);
        glob_iter = Box::new(glob_iter.skip(1));
    } else {
        regex = None;
    }

    let globs = glob_iter.map(|glob| glob.clone()).collect::<Vec<String>>();

    let number = parse_opt_str(&matches, "number", None)?;
    let skip = parse_opt_str(&matches, "skip", Some(0))?.expect("The default is a Some.");

    let stdin = globs.len() == 0;

    Ok(Parameters {
        all: matches.opt_present("all"),
        backwards: matches.opt_present("backwards"),
        case_replacements: matches.opt_present("case-replacements"),
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
            let escapes = escapes;

            let mut result = String::new();
            let mut chars = str.chars().peekable().into_iter();
            while let Some(char) = chars.next() {
                let mut found_escape = false;
                if char == '\\' {
                    if let Some(next) = chars.peek() {
                        if let Some(escape) = escapes.get(&next) {
                            // Escape sequences converted to the character they represent.
                            result.push(*escape);
                            found_escape = true;
                        }
                    }
                }
                if found_escape {
                    chars.next();
                } else {
                    // Unescaped characters unchanged,
                    // unrecognised escape sequences unchanged,
                    // backslash at end of string unchanged.
                    result.push(char);
                }
            }
            Some(result)
        }
        None => None,
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

fn parse_opt_str<T: FromStr>(matches: &Matches, option: &str, default: Option<T>) -> NedResult<Option<T>> {
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
    Ok(default)
}
