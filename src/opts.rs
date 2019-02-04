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
static USAGE: &'static str = "Usage: ned [OPTION...] [-p] PATTERN [FILE...]
       ned [OPTION...] [FILE...] -p PATTERN";

static PRE_DESCRIPTION: &'static str = "\
For regular expression power users, ned is like grep, but with
powerful replace capabilities, and unlike sed, as it
isn't restricted to line oriented editing.

FILEs are ASCII or UTF-8 text files. For regex syntax see:

  https://docs.rs/regex/1.1.0/regex/#syntax";

static POST_DESCRIPTION: &'static str = "Environment:
    NED_DEFAULTS        ned options added to the program's arguments. Is
                        a space delimited list of options and is not first
                        interpreted by a shell, so quotes are not required
                        around arguments. For example:

                        NED_DEFAULTS=\"-u -R --exclude *.bk --exclude-dir .git\"
Exit codes:
    0                   matches found/replaced
    1                   no matches

Quiet:
    When -q/--quiet is specified, ned tests for matches and returns an exit
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
        "Specify a pattern. If the option isn't used the pattern must precede the files. \
         The option allows the pattern to be put after the files for more convenient \
         editing.",
        "PATTERN",
    );
    opts.optopt(
        "r",
        "replace",
        "Replace matches. Replacements may include numbered and named groups. Replaces always operate on whole \
         files.",
        "REPLACEMENT",
    );
    opts.optflag(
        "w",
        "whole-files",
        "Operate on whole files. Otherwise matches are line oriented.",
    );
    opts.optopt("n", "number", "Match/replace N occurrences.", "N");
    opts.optopt(
        "k",
        "skip",
        "Skip N occurrences before matching/replacing.",
        "N",
    );
    opts.optflag(
        "b",
        "backwards",
        "Make -n/--number and -k/--skip options count backwards.",
    );
    opts.optflag("i", "ignore-case", "Ignore case.");
    opts.optflag(
        "s",
        "single",
        "'.' matches newlines, ^ and $ match the beginning and end of each file. Use \
         with --whole-files.",
    );
    opts.optflag(
        "m",
        "multiline",
        "Multiline, ^ and $ match the beginning and end of each line. Use with \
         --whole-files.",
    );
    opts.optflag("x", "extended", "Ignore whitespace and # comments.");
    opts.optflag(
        "",
        "case-replacements",
        "Enable \\U - uppercase, \\L - \
         lowercase, \\I - initial uppercase (title case), \\F - first uppercase \
         (sentence case) replacements. \\E marks the end of a case replacement.",
    );
    opts.optflag("o", "matches-only", "Show only matches.");
    opts.optopt(
        "g",
        "group",
        "Show the match group, specified by number or name.",
        "GROUP",
    );
    opts.optflag("v", "no-match", "Show only non-matching.");
    opts.optflag(
        "f",
        "filenames-only",
        "Show only filenames containing matches. Use with -v/\
         --no-match to show filenames without matches.",
    );
    opts.optflag("F", "no-filenames", "Don't show filenames.");
    opts.optflag(
        "l",
        "line-numbers-only",
        "Show only line numbers containing matches. Use with -v/\
         --no-match to show line numbers without matches. Use without -w/\
         --whole-files.",
    );
    opts.optflag(
        "L",
        "no-line-numbers",
        "Don't show line numbers. Use without -w/--whole-files.",
    );
    opts.optopt(
        "C",
        "context",
        "Show LINES lines around each matching line. Is the same as \
         specifying both -B/--before and -A/--after with the same LINES. Use without -w/\
         --whole-files.",
        "LINES",
    );
    opts.optopt(
        "B",
        "before",
        "Show LINES lines before each matching line. Use without -w/\
         --whole-files.",
        "LINES",
    );
    opts.optopt(
        "A",
        "after",
        "Show LINES lines after each matching line. Use without -w/\
         --whole-files.",
        "LINES",
    );
    opts.optflag("R", "recursive", "Recurse.");
    opts.optflag("l", "follow", "Follow symlinks. (Ignored on Windows.)");
    opts.optmulti("", "include", "Match only files that match GLOB.", "GLOB");
    opts.optmulti("", "exclude", "Skip files matching GLOB.", "GLOB");
    opts.optmulti("", "exclude-dir", "Skip directories matching GLOB.", "GLOB");
    opts.optflag(
        "u",
        "ignore-non-utf8",
        "Quietly ignore files that cannot be parsed as UTF-8 (or ASCII). Because \
         this requires reading the file, the --exclude option should be preferred.",
    );
    opts.optflag("a", "all", "Do not ignore files and directories starting with '.'.");
    opts.optflag("c", "", "Show filenames, line numbers, and matches in color. Is the same as --colors=always.");
    opts.optflagopt(
        "",
        "colors",
        "'auto' shows filenames, line numbers, and matches in color when stdout is a terminal, not when it is a pipe, 'always' shows color even when stdout is a pipe, and 'never' never shows colors.",
        "WHEN",
    );
    opts.optflag("", "stdout", "Output to stdout.");
    opts.optflag("q", "quiet", "Suppress all normal output. When matching terminate as soon as a match is found.");
    opts.optflag("V", "version", "Output version information and exit.");
    opts.optflag("h", "help", "Print this help and exit.");
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
        "{}\n\n{}",
        &USAGE, &PRE_DESCRIPTION
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
