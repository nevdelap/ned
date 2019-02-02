//
// ned, https://github.com/nevdelap/ned, opts.rs
//
// Copyright 2016-2019 Nev Delap (nevdelap at gmail)
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

use getopts::{Options, ParsingStyle};
use std::string::String;

pub static PROGRAM: &'static str = "ned";
static OPTS_AND_ARGS: &'static str = "[OPTION]... [-p] <PATTERN> [FILE]...";

static PRE_DESCRIPTION: &'static str = "\
For regular expression power users, ned is like grep, but with
powerful replace capabilities, and more powerful than sed, as it
isn't restricted to line oriented editing.

FILEs are ASCII or UTF-8 text files. For regex syntax see:

  https://docs.rs/regex/1.1.0/regex/#syntax";

static POST_DESCRIPTION: &'static str = "Environment:
    NED_DEFAULTS        ned options added to the program's arguments. is
                        a space delimited list of options and is not first
                        interpreted by a shell, so quotes are not required.
                        for example...

                        NED_DEFAULTS=\"-u -R --exclude *.bk --exclude-dir .git\"
Exit codes:
    0                   matches found/replaced
    1                   no matches

Quiet:
    When -q (--quiet) is specified, ned tests for matches and returns an exit
    code of 0 if a match is found in ANY file. Quiet matches will only read
    as many files as needed to find a match. Even without this shortcutting
    behaviour, quiet matches are more performant than non-quiet matches.";

static COPYRIGHT: &'static str =
    "\
     Copyright (C) 2016-2019 Nev Delap - https://github.com/nevdelap/ned";

static LICENSE: &'static str = "\
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.";

pub fn make_opts() -> Options {
    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::FloatingFrees);
    opts.optopt(
        "p",
        "pattern",
        "specify pattern. if the option isn't used the pattern must precede the files. \
         the option allows the pattern to be put after the files for more convenient \
         editing",
        "PATTERN",
    );
    opts.optopt(
        "r",
        "replace",
        "replace matches, may include named groups. replaces always operate on whole \
         files",
        "REPLACEMENT",
    );
    opts.optflag(
        "w",
        "whole-files",
        "operate on whole files. otherwise matches are line oriented",
    );
    opts.optopt("n", "number", "match/replace N occurrences", "N");
    opts.optopt(
        "k",
        "skip",
        "skip N occurrences before matching/replacing",
        "N",
    );
    opts.optflag(
        "b",
        "backwards",
        "-n --number and -k --skip options count backwards",
    );
    opts.optflag("i", "ignore-case", "ignore case");
    opts.optflag(
        "s",
        "single",
        ". matches newlines, ^ and $ match beginning and end of each file. use \
         with --whole-files",
    );
    opts.optflag(
        "m",
        "multiline",
        "multiline, ^ and $ match beginning and end of each line. use with \
         --whole-files",
    );
    opts.optflag("x", "extended", "ignore whitespace and # comments");
    opts.optflag(
        "",
        "case-replacements",
        "enable \\U - uppercase, \\L - \
         lowercase, \\I - initial uppercase (title case), \\F - first uppercase \
         (sentence case) replacements. \\E marks the end of a case replacement",
    );
    opts.optflag("o", "matches-only", "show only matches");
    opts.optopt(
        "g",
        "group",
        "show the match group, specified by number or name",
        "GROUP",
    );
    opts.optflag("v", "no-match", "show only non-matching");
    opts.optflag(
        "f",
        "filenames-only",
        "show only filenames containing matches. use with -v \
         --no-match to show filenames without matches",
    );
    opts.optflag("F", "no-filenames", "don't show filesnames");
    opts.optflag(
        "l",
        "line-numbers-only",
        "show only line numbers containing matches. use with -v \
         --no-match to show line numbers without matches. use without -w \
         --whole-files",
    );
    opts.optflag(
        "L",
        "no-line-numbers",
        "don't show line numbers, use without -w --whole-files",
    );
    opts.optopt(
        "C",
        "context",
        "show LINES lines around each matching line. is the same as \
         specifying both -B --before and -A --after with the same LINES. use without -w \
         --whole-files",
        "LINES",
    );
    opts.optopt(
        "B",
        "before",
        "show LINES lines before each matching line. use without -w \
         --whole-files",
        "LINES",
    );
    opts.optopt(
        "A",
        "after",
        "show LINES lines after each matching line. use without -w \
         --whole-files",
        "LINES",
    );
    opts.optflag("R", "recursive", "recurse");
    opts.optflag("l", "follow", "follow symlinks (Ignored on Windows.)");
    opts.optmulti("", "include", "match only files that match GLOB", "GLOB");
    opts.optmulti("", "exclude", "skip files matching GLOB", "GLOB");
    opts.optmulti("", "exclude-dir", "skip directories matching GLOB", "GLOB");
    opts.optflag(
        "u",
        "ignore-non-utf8",
        "quietly ignore files that cannot be parsed as UTF-8 (or ASCII). because \
         this requires reading the file the --exclude option should be preferred",
    );
    opts.optflag("a", "all", "do not ignore entries starting with .");
    opts.optflag("c", "", "show filenames, line numbers, and matches in color. is the same as --colors=always");
    opts.optflagopt(
        "",
        "colors",
        "'auto' shows filenames, line numbers, and matches in color when stdout is a terminal, not when it is a pipe, 'always' shows color even when stdout is a pipe, and 'never' never shows colors",
        "WHEN",
    );
    opts.optflag("", "stdout", "output to stdout");
    opts.optflag("q", "quiet", "suppress all normal output");
    opts.optflag("V", "version", "output version information and exit");
    opts.optflag("h", "help", "print this help and exit");
    opts
}

pub fn usage_version() -> String {
    let version = option_env!("CARGO_PKG_VERSION");
    format!(
        "{} {} {}\n\n{}\n",
        PROGRAM,
        version.expect("We know CARGO_PKG_VERSION will exist."),
        COPYRIGHT,
        LICENSE
    )
}

pub fn usage_brief() -> String {
    format!(
        "Usage: {} {}\n\n{}",
        PROGRAM, &OPTS_AND_ARGS, &PRE_DESCRIPTION
    )
}

pub fn usage_full(opts: &Options) -> String {
    let version = option_env!("CARGO_PKG_VERSION");
    format!(
        "{}\n{}\n\n{} {} {}\n\n{}\n",
        opts.usage(&usage_brief()),
        POST_DESCRIPTION,
        PROGRAM,
        version.expect("We know CARGO_PKG_VERSION will exist."),
        COPYRIGHT,
        LICENSE
    )
}
