![ned Screenshot](https://github.com/nevdelap/ned/blob/master/img/nedScreenshot.png)

# `ned` Usage

The program's help gives a comprehensive description of its available
options, the [wiki](https://github.com/nevdelap/ned/wiki) gives
further details, and the [TL;DR](#tldr) section below has a list of
quick and easy example usages.

```text

> ned --help

Usage: ned [OPTION...] [-p] PATTERN [FILE...]
       ned [OPTION...] [FILE...] -p PATTERN

For regular expression power users, ned is like grep, but with
powerful replace capabilities, and unlike sed, as it
isn't restricted to line oriented editing.

FILEs are ASCII or UTF-8 text files. For regex syntax see:

  https://docs.rs/regex/1.3.4/regex/#syntax

Options:
    -p, --pattern PATTERN
                        Specify a pattern. If the option isn't used, the
                        pattern must precede the files. The option allows the
                        pattern to be put after the files for more convenient
                        editing.
    -r, --replace REPLACEMENT
                        Replace matches. Replacements may include numbered and
                        named groups. Replaces always operate on whole files.
    -w, --whole-files   Operate on whole files. Otherwise matches are line
                        oriented.
    -n, --number N      Match/replace N occurrences.
    -k, --skip N        Skip N occurrences before matching/replacing.
    -b, --backwards     Make -n/--number and -k/--skip options count
                        backwards.
    -i, --ignore-case   Ignore case.
    -s, --single        '.' matches newlines, ^ and $ match the beginning and
                        end of each file. Use with --whole-files.
    -m, --multiline     Multiline, ^ and $ match the beginning and end of each
                        line. Use with --whole-files.
    -x, --extended      Ignore whitespace and # comments.
        --case-replacements
                        Enable \U - uppercase, \L - lowercase, \I - initial
                        uppercase (title case), \F - first uppercase (sentence
                        case) replacements. \E marks the end of a case
                        replacement.
    -o, --matches-only  Show only matches.
    -g, --group GROUP   Show the match group, specified by number or name.
    -v, --no-match      Show only non-matching.
    -f, --filenames-only
                        Show only filenames containing matches. Use with
                        -v/--no-match to show filenames without matches.
    -F, --no-filenames  Don't show filenames.
    -l, --line-numbers-only
                        Show only line numbers containing matches. Use with
                        -v/--no-match to show line numbers without matches.
                        Use without -w/--whole-files.
    -L, --no-line-numbers
                        Don't show line numbers. Use without -w/--whole-files.
    -C, --context LINES Show LINES lines around each matching line. Is the
                        same as specifying both -B/--before and -A/--after
                        with the same LINES. Use without -w/--whole-files.
    -B, --before LINES  Show LINES lines before each matching line. Use
                        without -w/--whole-files.
    -A, --after LINES   Show LINES lines after each matching line. Use without
                        -w/--whole-files.
    -R, --recursive     Recurse.
    -l, --follow        Follow symlinks. (Ignored on Windows.)
        --include GLOB  Match only files that match GLOB.
        --exclude GLOB  Skip files matching GLOB.
        --exclude-dir GLOB
                        Skip directories matching GLOB.
    -u, --ignore-non-utf8
                        Quietly ignore files that cannot be parsed as UTF-8
                        (or ASCII). Because this requires reading the file,
                        the --exclude option should be preferred.
    -a, --all           Do not ignore files and directories starting with '.'.
    -c                  Show filenames, line numbers, and matches in color. Is
                        the same as --colors=always.
        --colors [WHEN] 'auto' shows filenames, line numbers, and matches in
                        color when stdout is a terminal, not when it is a
                        pipe, 'always' shows color even when stdout is a pipe,
                        and 'never' never shows colors.
        --stdout        Output to stdout.
    -q, --quiet         Suppress all normal output. When matching terminate as
                        soon as a match is found.
    -V, --version       Output version information and exit.
    -h, --help          Print this help and exit.

Environment:
    NED_DEFAULTS        ned options added to the program's arguments. Is
                        a space delimited list of options and is not first
                        interpreted by a shell, so quotes are not required
                        around arguments. For example:

                        NED_DEFAULTS="-u -R --exclude *.bk --exclude-dir .git"
Exit codes:
    0                   matches found/replaced
    1                   no matches

Quiet:
    When -q/--quiet is specified, ned tests for matches and returns an exit
    code of 0, if a match is found in ANY file. Quiet matches will only read
    as many files as needed to find a match. Even without this shortcutting
    behaviour, quiet matches are more performant than non-quiet matches.

ned 1.2.9 Copyright (C) 2016-2019 Nev Delap - https://github.com/nevdelap/ned

License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

```

# I.A.Q. (Infrequently Asked Questions)

**_Why isn't \U working? (or \L, \I, \F)_**

Because case replacing is off by default to not waste cycles when you're not doing it,
since that is most of the time, generally. See the help... (as of v1.2.0)

```
       --case-replacements
                        Enable \U - uppercase, \L - lowercase, \I - initial
                        uppercase (title case), \F - first uppercase (sentence
                        case) replacements. \E marks the end of a case
                        replacement.
```

**_Why do I get errors like ned: /path/file invalid utf-8 sequence of 1 bytes from index 25?_**

Because, by default, `ned` reads everything unless you tell it not to read it. If you want it to always
ignore non-ASCII, non-UTF-8 files, you can put this in NED_DEFAULTS. See the help...

```
   -u, --ignore-non-utf8
                        Quietly ignore files that cannot be parsed as UTF-8
                        (or ASCII). Because this requires reading the file,
                        the --exclude option should be preferred.
```

**_Why don't the tests pass in Git Bash?_**

Git Bash does not support colored output using ansi_term. Run the tests in cmd.exe.

# Building `ned`

### Machine Setup To Build `ned`

-   Install rust as per: https://www.rust-lang.org/en-US/install.html
-   (Windows) Install Visual Studio Build Tools 2017 as per: https://www.visualstudio.com/downloads/

### To build for the current platform.

Last tested on Manjaro 18.0.2 up-to-date, on Windows 10.0.17134.523, and on OS X High Sierra 13.1.6, with Rust 1.32.0.

```
cd ned
cargo build --release
cargo test
...
test result: ok. 137 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

### To build for 64bit musl.

Last tested on Manjaro 18.0.2 up-to-date, with Rust 1.32.0.

```
cd ned
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cargo test --target x86_64-unknown-linux-musl
...
test result: ok. 137 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

# Installing `ned`

### To Install in Arch and Manjaro and other Arch based distros.

```
yaourt -S ned
```

### To Install in Debian, Ubuntu and other Debian based distros, or Red Hat or CentOS.

Download the deb or rpm file from the latest release: https://github.com/nevdelap/ned/releases. They package a single musl binary of `ned` and its man page with no dependencies. Install with `dpkg -i` or `rpm -i`.

### To Install in Other Linux Distros, Mac OS X, or Windows.

Download the appropriate binary and put it on your path. In Windows install the dependencies as described in the latest release: https://github.com/nevdelap/ned/releases

# TL;DR

**IMPORTANT NOTE:** The search capabilities of `ned` are not so interesting, you can do them all with `grep` or `ripgrep`, see https://github.com/BurntSushi/ripgrep. It is the replace that is interesting, which `grep` and `ripgrep` cannot do, and `sed` can only do in a line oriented way. Examples of searching are shown first, followed by examples of replacing. Replacing with `ned` is a very powerful way of doing bulk editing from the terminal. Stage or commit your files before doing bulk edits, so that you can undo changes until you get your regex right.

These examples use short options and search for 'dog' and replace with 'cat' wherever the example doesn't need a regular expression to demonstrate what it is doing.

#### Search non-hidden files in the current directory.

```
ned dog .
```

#### Search txt files in the current directory.

```
ned dog *.txt
```

#### Search including hidden files.

```
ned -a dog .
```

#### Search recursively.

```
ned -R dog .
```

#### Search case insensitively.

```
ned -i dog .
```

#### Search always showing colored output.

```
ned -c dog .
ned --colors=always dog .
```

#### Search never showing colored output.

```
ned --colors=never dog .
```

#### Search showing colored output when outputting to a terminal, but don't send colored output, if piped.

```
ned --colors=auto dog .
```

#### Set default arguments in your terminal environment.

```
export NED_DEFAULTS='-i --colors=always'
```

#### Search showing colored output through less.

```
ned -c dog . | less -R
```

#### Search showing no output, to just use the exit code in a script, if something is found or not found.

This is more efficient when you don't need the output since it shortcuts when it finds the first match.

```
ned -q dog .; echo $?
0 # Found.
ned -q dinosaur .; echo $?
1 # Not found.
```

#### Search specifying the pattern at the end of the command - for convenience of editing when you have a lot of options.

```
ned . -p dog
```

#### Search not showing line numbers.

```
ned -L dog .
```

#### Search only showing numbers of matched lines.

```
ned -l dog .
```

#### Search not showing file names

```
ned -F dog .
```

#### Search only showing file names of matched files.

```
ned -f dog .
```

#### Search showing only matches.

```
ned -o dog .
```

#### Search really showing only matches.

```
ned -oFL dog .
```

#### Search matching first 3 occurences per line.

```
ned -n 3 dog .
```

#### Search matching first 3 occurences per file.

```
ned -w -n 3 dog .
```

#### Search backwards, matching first 3 occurences per line.

```
ned -b -n 3 dog .
```

#### Search backwards, matching first 3 occurences per file.

```
ned -b -w -n 3 dog .
```

#### Search skipping 3 occurrences and finding 2 occurences.

**Note:** -k is the short form of --skip. (-s is the short form of the --single option.)

```
ned -k 3 -n 2 dog .
```

#### Search backwards, skipping 3 occurrences and finding 2 occurences.

```
ned -b -k 3 -n 2 dog .
```

#### Search recursively only including certain files.

```
ned -R --include '*.txt' dog .
```

#### Search ignoring certain files.

```
ned -R --exclude '*.htm' dog .
```

#### Search ignoring all non-utf8 files.

Quietly ignore files that cannot be parsed as UTF-8 (or ASCII). Because this requires reading the file the --exclude option should be preferred. E.g. --exclude '\*.png'

```
ned -u dog .
```

#### Search ignoring certain directories.

```
ned -R --exclude-dir '.git' dog .
```

#### Search showing context of 5 lines around each match.

```
ned -C 5 dog .
```

#### Search showing context of 5 lines before each match.

```
ned -B 5 dog .
```

#### Search showing context of 5 lines after match.

```
ned -A 5 dog .
```

#### Search matching the beginnings of lines.

```
ned '^dog' .
```

#### Search matching the ends of lines.

```
ned 'dog$' .
```

#### Search matching the beginnings of files.

```
ned -w '^dog' .
```

#### Search matching the ends of files.

```
ned -w 'dog$' .
```

#### Search spanning lines.

Search for any and all three consecutive lines containing the word dog.

```
ned -w 'dog.*\n.*dog.*\n.*dog' .
```

#### Replace.

```
ned dog -r cat .
```

#### Replace using numbered group references.

'the big dog and the smelly dog' replaced with 'the smelly dog and the big dog'.

```
ned 'the ([a-z]+) dog and the ([a-z]+) dog' -r 'the $2 dog and the $1 dog' .
```

#### Replace using named group references.

'the big dog and the smelly dog' replaced with 'the smelly dog and the big dog'.

```
ned 'the (?P<first>[a-z]+) dog and the (?P<second>[a-z]+) dog' -r 'the $second dog and the $first dog' .
```

#### Replace spanning lines.

Delete any and all three consecutive lines containing the word dog.

```
ned -w '\n.*dog.*\n.*dog.*\n.*dog.*\n' -r '\n'
```

#### Replace changing case.

'big dog' and 'smelly dog' replaced with 'BIG! dog' and 'SMELLY! dog'.
Available case replacements: \U - uppercase, \L - lowercase, \I - initial uppercase (title case), \F - first uppercase (sentence case).

```
ned ' ([a-z]+) dog' --case-replacements -r '\U$1\E! dog' --stdout .
```

#### Replace and see the results in the terminal without updating the target files.

```
ned dog -r cat --stdout .
```

#### Replace and treat no replacements as success.

```
ned trggde -r cat . || true; echo $?
0 # Maybe not found, but if so, not an error.
```

#### Strip blank lines from files.

```
ned -w '(\s*\n)+' -r '\n' .
```

#### Strip blank lines from the ends of files.

```
ned -w '(\s*\n?)*$' -r '' .
```

#### Use `ned` in `clean` and `smudge` filters in Git.

This example is stripping the value of the `toolbar_placement` configuration setting, and the values of all configuration settings that end in `_height` or `_width`, when doing a `git add` of `remmina.pref` in my personal config repo. The `-m/--multiline` means match the `toolbar_placement` at the beginning of a line, not the beginning of the file. **Note:** For a `clean` operation we want no replacements to be treated as success, hence the `|| true`.

```
# .gitattributes
remmina.pref  filter=clean_remmina_pref
```

```
# .gitconfig
[filter "clean_remmina_pref"]
    clean = ned -m --colors=never '(^toolbar_placement|_height|_width)=.*' -r '$1=' --stdout || true
    smudge = cat
```

#### Unident tables and lists in XHTML files, ignoring the .git directory.

```
ned -R --include '*.htm' --exclude-dir '.git'    (</?(table|col|tbody|tr|th|td|ol|ul|li)[^>]*>)' -r '$1'
```
