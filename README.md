![ned Screenshot](https://github.com/nevdelap/ned/blob/master/img/nedScreenshot.png)

# Ned Usage

```text

Usage: ned [OPTION]... [-p] <PATTERN> [FILE]...

ned is a bit like grep and a bit like sed. But unlike grep you don't have to
choose which grep to use depending the regex features you want, and unlike
sed it can operate on whole files, so you're not restricted in how you can
edit files.

FILEs are ASCII or UTF-8 text files. For regex syntax see:

  https://docs.rs/regex/1.0.0/regex/#syntax

Options:
    -p, --pattern PATTERN
                        specify pattern. if the option isn't used the pattern
                        must precede the files. the option allows the pattern
                        to be put after the files for more convenient editing
    -r, --replace REPLACEMENT
                        replace matches, may include named groups. replaces
                        always operate on whole files
    -w, --whole-files   operate on whole files. otherwise matches are line
                        oriented
    -n, --number N      match/replace N occurrences
    -k, --skip N        skip N occurrences before matching/replacing
    -b, --backwards     -n --number and -k --skip options count backwards
    -i, --ignore-case   ignore case
    -s, --single        . matches newlines, ^ and $ match beginning and end of
                        each file. use with --whole-files
    -m, --multiline     multiline, ^ and $ match beginning and end of each
                        line. use with --whole-files
    -x, --extended      ignore whitespace and # comments
        --case-replacements
                        enable \U - uppercase, \L - lowercase, \I - initial
                        uppercase (title case), \F - first uppercase (sentence
                        case) replacements. \E marks the end of a case
                        replacement
    -o, --matches-only  show only matches
    -g, --group GROUP   show the match group, specified by number or name
    -v, --no-match      show only non-matching
    -f, --filenames-only
                        show only filenames containing matches. use with -v
                        --no-match to show filenames without matches
    -F, --no-filenames  don't show filesnames
    -l, --line-numbers-only
                        show only line numbers containing matches. use with -v
                        --no-match to show line numbers without matches. use
                        without -w --whole-files
    -L, --no-line-numbers
                        don't show line numbers, use without -w --whole-files
    -C, --context LINES show LINES lines around each matching line. is the
                        same as specifying both -B --before and -A --after
                        with the same LINES. use without -w --whole-files
    -B, --before LINES  show LINES lines before each matching line. use
                        without -w --whole-files
    -A, --after LINES   show LINES lines after each matching line. use without
                        -w --whole-files
    -R, --recursive     recurse
    -l, --follow        follow symlinks (Ignored on Windows.)
        --include GLOB  match only files that match GLOB
        --exclude GLOB  skip files matching GLOB
        --exclude-dir GLOB
                        skip directories matching GLOB
    -u, --ignore-non-utf8
                        quietly ignore files that cannot be parsed as UTF-8
                        (or ASCII). because this requires reading the file the
                        --exclude option should be preferred
    -a, --all           do not ignore entries starting with .
    -c, --colors [WHEN] show filenames and matches in color when a real
                        stdout. defaults to auto, can be set to always to show
                        color even when not a real stdout, or never
        --stdout        output to stdout
    -q, --quiet         suppress all normal output
    -V, --version       output version information and exit
    -h, --help          print this help and exit

Environment:
    NED_DEFAULTS        ned options added to the program's arguments. is
                        a space delimited list of options and is not first
                        interpreted by a shell, so quotes are not required.
                        for example...

                        NED_DEFAULTS="-u -R --exclude *.bk --exclude-dir .git"
Exit codes:
    0                   matches found/replaced
    1                   no matches

Quiet:
    When -q --quiet is specified ned tests for matches and returns an exit
    code of 0 if a match is found in ANY file. Quiet matches will only read
    as many files as needed to find a match. Even without this shortcutting
    behaviour quiet matches are more performant than non-quiet matches.

ned 1.2.4 Copyright (C) 2016-2018 Nev Delap - https://github.com/nevdelap/ned

License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

```

# I.A.Q. (Infrequently Asked Questions)

***Why isn't \U working? (or \L, \I, \F)***

Because case replacing is off by default to not waste cycles when you're not doing it,
since that is most of the time, generally. See the help... (as of v1.2.0)

```
       --case-replacements
                        enable \U - uppercase, \L - lowercase, \I - initial
                        uppercase (title case), \F - first uppercase (sentence
                        case) replacements. \E marks the end of a case
                        replacement
```

***Why do I get errors like ned: /path/file invalid utf-8 sequence of 1 bytes from index 25?***

Because by default ned reads everything unless you tell it not to read it. If you want it to always
ignore non-ASCII, non-UTF-8 files you can put this in NED_DEFAULTS. See the help...

```
   -u, --ignore-non-utf8
                        quietly ignore files that cannot be parsed as UTF-8
                        (or ASCII). because this requires reading the file the
                        --exclude option should be preferred
```

***Why don't the tests pass in Git Bash?***

Git Bash does not support colored output using ansi_term.


# Machine Setup To Build Ned

* Install rust as per: https://www.rust-lang.org/en-US/install.html
* (Windows) Install Visual Studio Build Tools 2017 as per: https://www.visualstudio.com/downloads/

# Build Ned

### To build for the current platform.

Last tested on Ubuntu 18.04, and on Windows 10.0.16299.192, using the x64 Native Tools Command Prompt for VS2017, and on OS X High Sierra, with Rust 1.27.2.

```
cd ned
cargo build --release
cargo test
...
test result: ok. 134 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

### To build for 64bit musl.

Last tested on Ubuntu 18.04 with Rust 1.27.2.

```
cd ned
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cargo test --target x86_64-unknown-linux-musl
...
test result: ok. 134 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```
