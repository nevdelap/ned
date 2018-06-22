use getopts::{Options, ParsingStyle};
use std::string::String;
use time;

pub static PROGRAM: &'static str = "ned";
static OPTS_AND_ARGS: &'static str = "[OPTION]... [-p] <PATTERN> [FILE]...";

static PRE_DESCRIPTION: &'static str = "\
ned is a bit like grep and a bit like sed. But unlike grep you don't have to
choose which grep to use depending the regex features you want, and unlike
sed it can operate on whole files, so you're not restricted in how you can
edit files.

FILEs are ASCII or UTF-8 text files. For regex syntax see:

  https://docs.rs/regex/1.0.0/regex/#syntax";

static POST_DESCRIPTION: &'static str = "Environment:
    NED_DEFAULTS        ned options prepended to the program's arguments. is
                        a space delimited list of options and is not first
                        interpreted by a shell, so quotes are not required.
                        for example...

                        NED_DEFAULTS=\"-u -c --exclude *.bk --exclude-dir .git\"
Exit codes:
    0                   matches found/replaced
    1                   no matches

Quiet:
    When -q --quiet is specified ned tests for matches and returns an exit
    code of 0 if a match is found in ANY file. Quiet matches will only read
    as many files as needed to find a match. Even without this shortcutting
    behaviour quiet matches are more performant than non-quiet matches.";

static COPYRIGHT: &'static str =
    "\
     Copyright (C) 2016-2018 Nev Delap - https://github.com/nevdelap/ned";

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
    opts.optflagmulti(
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
    opts.optflagmulti(
        "b",
        "backwards",
        "-n --number and -k --skip options count backwards",
    );
    opts.optflagmulti("i", "ignore-case", "ignore case");
    opts.optflagmulti(
        "s",
        "single",
        ". matches newlines, ^ and $ match beginning and end of each file. use \
         with --whole-files",
    );
    opts.optflagmulti(
        "m",
        "multiline",
        "multiline, ^ and $ match beginning and end of each line. use with \
         --whole-files",
    );
    opts.optflagmulti("x", "extended", "ignore whitespace and # comments");
    opts.optflagmulti(
        "",
        "case-replacements",
        "enable \\U - uppercase, \\L - \
         lowercase, \\I - initial uppercase (title case), \\F - first uppercase \
         (sentence case) replacements. \\E marks the end of a case replacement",
    );
    opts.optflagmulti("o", "matches-only", "show only matches");
    opts.optopt(
        "g",
        "group",
        "show the match group, specified by number or name",
        "GROUP",
    );
    opts.optflagmulti("v", "no-match", "show only non-matching");
    opts.optflagmulti(
        "f",
        "filenames-only",
        "show only filenames containing matches. use with -v \
         --no-match to show filenames without matches",
    );
    opts.optflagmulti("F", "no-filenames", "don't show filesnames");
    opts.optflagmulti(
        "l",
        "line-numbers-only",
        "show only line numbers containing matches. use with -v \
         --no-match to show line numbers without matches. use without -w \
         --whole-files",
    );
    opts.optflagmulti(
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
    opts.optflagmulti("R", "recursive", "recurse");
    opts.optflagmulti("l", "follow", "follow symlinks (Ignored on Windows.)");
    opts.optmulti("", "include", "match only files that match GLOB", "GLOB");
    opts.optmulti("", "exclude", "skip files matching GLOB", "GLOB");
    opts.optmulti("", "exclude-dir", "skip directories matching GLOB", "GLOB");
    opts.optflagmulti(
        "u",
        "ignore-non-utf8",
        "quietly ignore files that cannot be parsed as UTF-8 (or ASCII). because \
         this requires reading the file the --exclude option should be preferred",
    );
    opts.optflagmulti("a", "all", "do not ignore entries starting with .");
    opts.optflagmulti(
        "c",
        "colors",
        "show filenames and matches in color when a real stdout",
    );
    opts.optflagmulti("", "stdout", "output to stdout");
    opts.optflagmulti("q", "quiet", "suppress all normal output");
    opts.optflagmulti("V", "version", "output version information and exit");
    opts.optflagmulti("h", "help", "print this help and exit");
    opts
}

pub fn usage_version() -> String {
    let version = option_env!("CARGO_PKG_VERSION");
    format!(
        "{} {} {}\n\n{}",
        PROGRAM,
        version.unwrap(),
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
    let now = time::now();
    format!(
        "{}\n{}\n\n{} {} {}\n\n{}\n\nBuilt {}-{:02}-{:02}.",
        opts.usage(&usage_brief()),
        POST_DESCRIPTION,
        PROGRAM,
        version.unwrap(),
        COPYRIGHT,
        LICENSE,
        1900 + now.tm_year,
        now.tm_mon + 1,
        now.tm_mday
    )
}
