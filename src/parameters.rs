//
// ned, https://github.com/nevdelap/ned, parameters.rs
//
// Copyright 2016-2021 Nev Delap (nevdelap at gmail)
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA
// 02110-1301, USA.
//

extern crate regex;

use crate::colors::Colors;
use crate::ned_error::{NedError, NedResult, StringError};
use crate::options_with_defaults::OptionsWithDefaults;
use glob::Pattern;
use libc;
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
        } else if let Some(number) = self.number {
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
        };
        index >= skip
            && if let Some(number) = number {
                index - skip < number
            } else {
                true
            }
    }
}

pub fn get_parameters(options_with_defaults: &OptionsWithDefaults) -> NedResult<Parameters> {
    // -C --context takes precedence over -B --before and -A --after.
    let mut context_before =
        parse_opt_str(options_with_defaults, "context", Some(0))?.expect("The default is a Some.");
    let context_after = if context_before != 0 {
        context_before
    } else {
        context_before = parse_opt_str(options_with_defaults, "before", Some(0))?
            .expect("The default is a Some.");
        parse_opt_str(options_with_defaults, "after", Some(0))?.expect("The default is a Some.")
    };

    let mut exclude_dirs = Vec::<Pattern>::new();
    for exclude in options_with_defaults.opt_strs("exclude-dir") {
        let pattern = Pattern::new(&exclude)?;
        exclude_dirs.push(pattern);
    }

    let mut excludes = Vec::<Pattern>::new();
    for exclude in options_with_defaults.opt_strs("exclude") {
        let pattern = Pattern::new(&exclude)?;
        excludes.push(pattern);
    }

    let mut includes = Vec::<Pattern>::new();
    for include in options_with_defaults.opt_strs("include") {
        let pattern = Pattern::new(&include)?;
        includes.push(pattern);
    }

    let whole_files = options_with_defaults.opt_present("whole-files");

    // TODO: Test combinations of file name and line number options.

    // file_names_only takes precedence over line_numbers_only.
    let file_names_only = options_with_defaults.opt_present("filenames-only");
    let line_numbers_only =
        !whole_files && !file_names_only && options_with_defaults.opt_present("line-numbers-only");

    // file_names_only takes precedence over no_file_names.
    let no_file_names = !file_names_only && options_with_defaults.opt_present("no-filenames");
    let no_line_numbers = !line_numbers_only
        && (file_names_only
            || !whole_files && options_with_defaults.opt_present("no-line-numbers"));

    let regex;
    let mut globs = options_with_defaults.free();

    if options_with_defaults.opt_present("pattern") {
        let pattern = add_regex_flags_to_pattern(
            options_with_defaults,
            &options_with_defaults.opt_str("pattern").expect(
                "Bug, already checked that pattern \
                 is present.",
            ),
        );
        regex = Some(Regex::new(&pattern)?);
    } else if !options_with_defaults.free().is_empty() {
        let pattern = globs.remove(0);
        let pattern = add_regex_flags_to_pattern(options_with_defaults, &pattern);
        regex = Some(Regex::new(&pattern)?);
    } else {
        regex = None;
    }

    let number = parse_opt_str(options_with_defaults, "number", None)?;
    let skip =
        parse_opt_str(options_with_defaults, "skip", Some(0))?.expect("The default is a Some.");

    let stdin = globs.is_empty();
    let stdout = stdin || options_with_defaults.opt_present("stdout");
    let replace = convert_escapes(options_with_defaults.opt_str("replace"));
    // TODO: decide what is the best way to deal with STDOUT_FILENO not being defined in the x86_64-pc-windows-gnu,
    // x86_64-pc-windows-msvc, or i686-pc-windows-msvc versions of libc.
    let isatty = unsafe {
        libc::isatty(/*libc::STDOUT_FILENO as i32*/ 1)
    } != 0;

    let c = options_with_defaults.opt_present("c");
    let mut colors = parse_opt_str(options_with_defaults, "colors", None)?;
    if colors.is_none() {
        // --color is a synonym of --colors, the original --colors is used if both are specified.
        colors = parse_opt_str(options_with_defaults, "color", Some(Colors::Off))?;
    }
    let colors = colors.expect("The default is a Some.");
    let colors = c
        || (colors == Colors::Always && (replace.is_none() || replace.is_some() && stdout)
            || colors == Colors::Auto && (replace.is_none() || stdout) && isatty)
            && colors != Colors::Never;

    Ok(Parameters {
        all: options_with_defaults.opt_present("all"),
        backwards: options_with_defaults.opt_present("backwards"),
        case_replacements: options_with_defaults.opt_present("case-replacements"),
        colors,
        context_after,
        context_before,
        exclude_dirs,
        excludes,
        file_names_only,
        follow: options_with_defaults.opt_present("follow"),
        globs,
        group: options_with_defaults.opt_str("group"),
        help: options_with_defaults.opt_present("help"),
        ignore_non_utf8: options_with_defaults.opt_present("ignore-non-utf8"),
        includes,
        line_numbers_only,
        matches_only: options_with_defaults.opt_present("matches-only"),
        no_file_names,
        no_line_numbers,
        no_match: options_with_defaults.opt_present("no-match"),
        number,
        quiet: options_with_defaults.opt_present("quiet"),
        recursive: options_with_defaults.opt_present("recursive"),
        regex,
        replace,
        skip,
        stdin,
        stdout,
        version: options_with_defaults.opt_present("version"),
        whole_files,
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
            let mut chars = str.chars().peekable();
            while let Some(char) = chars.next() {
                let mut found_escape = false;
                if char == '\\' {
                    if let Some(next) = chars.peek() {
                        if let Some(escape) = escapes.get(next) {
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

fn add_regex_flags_to_pattern(
    options_with_defaults: &OptionsWithDefaults,
    pattern: &str,
) -> String {
    let mut regex_flags = "".to_string();
    for option in &["i", "s", "m", "x"] {
        if options_with_defaults.opt_present(option) {
            regex_flags.push_str(option);
        }
    }
    if !regex_flags.is_empty() {
        format!("(?{}){}", &regex_flags, &pattern)
    } else {
        pattern.to_string()
    }
}

fn parse_opt_str<T: FromStr>(
    options_with_defaults: &OptionsWithDefaults,
    option: &str,
    default: Option<T>,
) -> NedResult<Option<T>> {
    let value;
    // If the string exists with a value...
    if let Some(v) = options_with_defaults.opt_str(option) {
        value = v;
    // ...or it exists without a value, in which case we assume it is empty string...
    } else if options_with_defaults.opt_present(option) {
        value = "".to_string();
    } else {
        return Ok(default);
    }
    // ..then parse it.
    match value.trim().parse::<T>() {
        Ok(value) => Ok(Some(value)),
        Err(_) => Err(NedError::ParameterError(StringError {
            err: format!("invalid value for --{} option", option),
        })),
    }
}
