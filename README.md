```text
Usage: ned [OPTION]... [-p] <PATTERN> [FILE]...

ned is a bit like grep and a bit like sed, but not really. FILEs are ASCII or
UTF-8 text files.

For regex syntax see: http://rust-lang-nursery.github.io/regex/regex/#syntax

Options:
    -p, --pattern PATTERN
                        specify pattern. if the option isn't used the pattern
                        must precede the files. the option allows the pattern
                        to be put after the files for more convenient editing
    -r, --replace REPLACEMENT
                        replace matches, may include named groups. replaces
                        always operate on whole files
    -n, --number N      (not yet implemented) match/replace N occurrences
    -k, --skip N        (not yet implemented) skip N occurrences before
                        matching/replacing
    -b, --backwards     (not yet implemented) -n --number and -k --skip
                        options count backwards
    -i, --ignore-case   ignore case
    -s, --single        . matches newlines, ^ and $ match beginning and end of
                        each file. use with --whole-files
    -m, --multiline     multiline, ^ and $ match beginning and end of each
                        line. use with --whole-files
    -x, --extended      ignore whitespace and # comments
    -o, --matches-only  show only matches
    -g, --group GROUP   show the match group, specified by number or name
    -w, --whole-files   operate on whole files, rather than lines. otherwise
                        matches are line oriented
    -v, --no-match      show only non-matching
    -f, --filenames-only 
                        show only filenames containing matches. use with -v
                        --no-match to show filenames without matches
    -F, --no-filenames  don't show filesnames
    -R, --recursive     recurse
    -l, --follow        follow symlinks
        --include GLOB  match only files that match GLOB
        --exclude GLOB  skip files matching GLOB
        --exclude-dir GLOB
                        skip directories matching GLOB
    -u, --ignore-non-utf8 
                        quietly ignore files that cannot be parsed as UTF-8
                        (or ASCII). this requires reading the file. the
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
    When -q --quiet is  specified ned tests for matches and returns an exit
    code of 0 if a match is found in ANY file. Quiet matches will only read
    only as many files as needed to find a match. Even without this
    shortcutting behaviour quiet matches are more performant than non-quiet
    matches.

ned 0.1.9 Copyright (C) 2016 Nev Delap - https://github.com/nevdelap/ned

License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

```
