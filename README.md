# Ned Usage

```text
Usage: ned [OPTION]... [-p] <PATTERN> [FILE]...

ned is a bit like grep and a bit like sed. But unlike grep you don't have to
choose which grep to use depending the regex features you want, and unlike
sed it can operate on whole files, so you're not restricted in how you can
edit files.

FILEs are ASCII or UTF-8 text files. For regex syntax see:

  https://doc.rust-lang.org/regex/regex/index.html#syntax

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
    -c, --colors        show filenames and matches in color
        --stdout        output to stdout
    -q, --quiet         suppress all normal output
    -V, --version       output version information and exit
    -h, --help          print this help and exit

Environment:
    NED_DEFAULTS        ned options prepended to the program's arguments. is
                        a space delimited list of options and is not first
                        interpreted by a shell, so quotes are not required.
                        for example...

                        NED_DEFAULTS="-u -c --exclude *.bk --exclude-dir .git"
Exit codes:
    0                   matches found/replaced
    1                   no matches

Quiet:
    When -q --quiet is specified ned tests for matches and returns an exit
    code of 0 if a match is found in ANY file. Quiet matches will only read
    as many files as needed to find a match. Even without this shortcutting
    behaviour quiet matches are more performant than non-quiet matches.

ned 0.1.10-alpha2 Copyright (C) 2016-2018 Nev Delap - https://github.com/nevdelap/ned

License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Built 2018-03-18.
```

# Machine Setup To Build Ned

* Install rust as per: https://www.rust-lang.org/en-US/install.html

# Build Ned

### To build for the current Linux platform.

Last tested on Ubuntu 17.10 with Rust 1.24.1.

```
cd ned
cargo build --release
cargo test
...
test result: ok. 119 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

### To build for 64bit musl.

Last tested on Ubuntu 17.10 with Rust 1.24.1.

```
cd ned
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cargo test --target x86_64-unknown-linux-musl
...
test result: ok. 119 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

### To build for 64bit Windows without downloading a load of MSVC junk.

Last tested on Windows 10 1709 with Rust 1.24.1. Not all tests pass as per
https://github.com/nevdelap/ned/issues/11. The ansi_term crate doesn't seem to
write ansi colour sequences to the terminal in Windows.

```
cd ned
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```