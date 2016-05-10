use getopts::{Options, ParsingStyle};
use std::string::String;

pub static PROGRAM: &'static str = "ned";
static OPTS_AND_ARGS: &'static str = "[OPTION]... [-p] <PATTERN> [FILE]...";
static PRE_DESCRIPTION: &'static str = "
ned is a bit like grep and a bit like sed, but not really. FILEs are ascii or
utf-8 text files.

For regex syntax see: http://rust-lang-nursery.github.io/regex/regex/#syntax";
static POST_DESCRIPTION: &'static str = "
Environment:
    NED_DEFAULTS        ned options prepended to the program's arguments

Exit codes:
    0                   matches found/replaced
    1                   no matches

Quiet:
    When -q --quiet is  specified ned tests for matches and returns an exit code
    of 0 if a match is found in ANY file. Quiet matches will only read only as
    many files as needed to find a match. Quiet matches are more performant than
    non-quiet matches.
";
static VERSION: &'static str = "
ned 0.1.5 Copyright (C) 2016 Nev Delap - https://github.com/nevdelap/ned
";
static LICENSE: &'static str = "
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
";

pub fn make_opts() -> Options {
    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::FloatingFrees);
    opts.optopt("p",
                "pattern",
                "specify pattern, if the option isn't used the pattern must precede the files, \
                 the option allows the pattern to be put after the files for more convenient \
                 editing",
                "PATTERN");
    opts.optopt("r",
                "replace",
                "replace matches, may include named groups. replaces always operate on whole \
                 files",
                "REPLACEMENT");
    // opts.optopt("n", "number", "match/replace N occurrences", "N");
    // opts.optopt("k",
    // "skip",
    // "skip N occurrences before matching/replacing",
    // "N");
    // opts.optflag("b",
    // "backwards",
    // "-n --number and -k --skip options count backwards");
    opts.optflag("i", "ignore-case", "ignore case");
    opts.optflag("s",
                 "single",
                 ". matches newlines, ^ and $ match beginning and end of each file. use with \
                  --whole-files");
    opts.optflag("m",
                 "multiline",
                 "multiline, ^ and $ match beginning and end of each line. use with --whole-files");
    opts.optflag("x", "extended", "ignore whitespace and # comments");
    opts.optflag("o", "only-matches", "show only matches");
    opts.optopt("g",
                "group",
                "show the match group, specified by number or name",
                "GROUP");
    opts.optflag("w",
                 "whole-files",
                 "operate on whole files, rather than lines. otherwise matches are line oriented");
    opts.optflag("v", "no-match", "show only non-matching");
    opts.optflag("R", "recursive", "recurse");
    opts.optflag("",
                 "files-with-matches",
                 "show only filenames containing matches");
    opts.optflag("",
                 "files-without-matches",
                 "show only filenames containing no match");
    opts.optflag("f", "follow", "follow symlinks");
    opts.optmulti("", "include", "match only files that match GLOB", "GLOB");
    opts.optmulti("", "exclude", "skip files matching GLOB", "GLOB");
    opts.optmulti("", "exclude-dir", "skip directories matching GLOB", "GLOB");
    opts.optflag("c", "colors", "show filenames and matches in color");
    opts.optflag("", "stdout", "output to stdout");
    opts.optflag("q", "quiet", "suppress all normal output");
    opts.optflag("a", "all", "do not ignore entries starting with .");
    opts.optflag("V", "version", "output version information and exit");
    opts.optflag("h", "help", "print this help and exit");
    opts
}

pub fn usage_version() -> String {
    format!("{}{}", &VERSION, &LICENSE)
}

pub fn usage_brief() -> String {
    format!("Usage: {} {}\n{}",
            PROGRAM,
            &OPTS_AND_ARGS,
            &PRE_DESCRIPTION)
}

pub fn usage_full(opts: &Options) -> String {
    format!("\n{}{}{}{}",
            opts.usage(&usage_brief()),
            &POST_DESCRIPTION,
            &VERSION,
            &LICENSE)
}
