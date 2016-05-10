```text


Usage: ned [OPTION]... [-p] <PATTERN> [FILE]...

ned is a bit like grep and a bit like sed, but not really. FILEs are ascii or
utf-8 text files.

For regex syntax see: http://rust-lang-nursery.github.io/regex/regex/#syntax

Options:
    -p, --pattern PATTERN
                        specify pattern, if the option isn't used the pattern
                        must precede the files, the option allows the pattern
                        to be put after the files for more convenient editing
    -r, --replace REPLACEMENT
                        replace matches, may include named groups. replaces
                        always operate on whole files
    -i, --ignore-case   ignore case
    -s, --single        . matches newlines, ^ and $ match beginning and end of
                        each file. use with --whole-files
    -m, --multiline     multiline, ^ and $ match beginning and end of each
                        line. use with --whole-files
    -x, --extended      ignore whitespace and # comments
    -o, --only-matches  show only matches
    -g, --group GROUP   show the match group, specified by number or name
    -w, --whole-files   operate on whole files, rather than lines. otherwise
                        matches are line oriented
    -v, --no-match      show only non-matching
    -R, --recursive     recurse
        --files-with-matches 
                        show only filenames containing matches
        --files-without-matches 
                        show only filenames containing no match
    -f, --follow        follow symlinks
        --include GLOB  match only files that match GLOB
        --exclude GLOB  skip files matching GLOB
        --exclude-dir GLOB
                        skip directories matching GLOB
    -c, --colors        show filenames and matches in color
        --stdout        output to stdout
    -q, --quiet         suppress all normal output
    -a, --all           do not ignore entries starting with .
    -V, --version       output version information and exit
    -h, --help          print this help and exit

Environment:
    NED_DEFAULTS        ned options prepended to the program's arguments

Exit codes:
    0                   matches found/replaced
    1                   no matches

Quiet:
    When -q --quiet is  specified ned tests for matches and returns an exit code
    of 0 if a match is found in ANY file. Quiet matches will only read only as
    many files as needed to find a match. Even without this shortcutting behaviour
    quiet matches are more performant than non-quiet matches.

ned 0.1.5 Copyright (C) 2016 Nev Delap - https://github.com/nevdelap/ned

License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

```
