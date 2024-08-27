//
// ned, https://github.com/nevdelap/ned, tests/matches.rs
//
// Copyright 2016-2024 Nev Delap (nevdelap at gmail)
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

/// Test match related functionality - different types of matches, matches with color, quiet, etc.
/// The use of re, not re itself.
use crate::options_with_defaults::OptionsWithDefaults;
use crate::opts::make_opts;
use crate::parameters::get_parameters;
use crate::process_file;
use crate::source::Source;
use std::env;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

#[test]
fn basic_match_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is";
    let args = "";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:This is a test.
bogus_file.txt:2:This is a test.
bogus_file.txt:3:This is a test.
bogus_file.txt:4:This is a test.
bogus_file.txt:5:This is a test.
bogus_file.txt:6:This is a test.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn basic_match_whole_files_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is";
    let args = "--whole-files";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn no_match_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "wiggle";
    let args = "";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn no_match_whole_files_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "wiggle";
    let args = "--whole-files";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn ignore_case_match_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "IS";
    let args = "--ignore-case";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:This is a test.
bogus_file.txt:2:This is a test.
bogus_file.txt:3:This is a test.
bogus_file.txt:4:This is a test.
bogus_file.txt:5:This is a test.
bogus_file.txt:6:This is a test.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn ignore_case_match_whole_files_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "IS";
    let args = "--whole-files --ignore-case";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn single_line_match_quiet_and_not_quiet() {
    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = r"^\nThis.*read it.\n$";
    let args = "--single";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn single_line_match_whole_files_quiet_and_not_quiet() {
    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = r"^\nThis.*read it.\n$";
    let args = "--whole-files --single";
    let expected_found_matches = true;
    let expected_screen_output = &format!("bogus_file.txt:\n{}", input);
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn multi_line_match_beginning_and_end_of_file_quiet_and_not_quiet() {
    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = r"\A\nThis(.|[\n])+read it.\n\z";
    let args = "--multiline";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn multi_line_match_beginning_and_end_of_file_whole_files_quiet_and_not_quiet() {
    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = r"\A\nThis(.|[\n])+read it.\n\z";
    let args = "--whole-files --multiline";
    let expected_found_matches = true;
    let expected_screen_output = &format!("bogus_file.txt:\n{}", &input);
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn multi_line_match_beginning_and_end_of_lines_quiet_and_not_quiet() {
    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = r"^multiple(.|[\n])+for$";
    let args = "--multiline";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn multi_line_match_beginning_and_end_of_lines_whole_files_quiet_and_not_quiet() {
    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = r"^multiple(.|[\n])+for$";
    let args = "--whole-files --multiline";
    let expected_found_matches = true;
    let expected_screen_output = &format!("bogus_file.txt:\n{}", input);
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn single_and_multi_line_match_quiet_and_not_quiet() {
    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = r"\A\nThis.+read it.\n\z";
    let args = "--single --multiline";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn single_and_multi_line_match_whole_files_quiet_and_not_quiet() {
    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = r"\A\nThis.+read it.\n\z";
    let args = "--whole-files --single --multiline";
    let expected_found_matches = true;
    let expected_screen_output = &format!("bogus_file.txt:\n{}", input);
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn extended_match_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "
# this is a commented
# regex that will find
# the word is.
is # Look, that's it!
# Cool magool.";
    let args = "--extended";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:This is a test.
bogus_file.txt:2:This is a test.
bogus_file.txt:3:This is a test.
bogus_file.txt:4:This is a test.
bogus_file.txt:5:This is a test.
bogus_file.txt:6:This is a test.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn extended_match_whole_files_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "
# this is a commented
# regex that will find
# the word is.
is # Look, that's it!
# Cool magool.";
    let args = "--whole-files --extended";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn only_matches_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is";
    let args = "--matches-only";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:isis
bogus_file.txt:2:isis
bogus_file.txt:3:isis
bogus_file.txt:4:isis
bogus_file.txt:5:isis
bogus_file.txt:6:isis
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn only_matches_whole_files_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is";
    let args = "--whole-files --matches-only";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
isisisisisisisisisisisis
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn only_matches_skip_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "o.";
    let args = "--matches-only --skip 2"; // Skip 2 is on each line.
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:o onoo
bogus_file.txt:2:ouomonon
bogus_file.txt:3:of
bogus_file.txt:4:oyotow
bogus_file.txt:5:owonoo
bogus_file.txt:6:od
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn only_matches_skip_whole_files_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--matches-only --skip 2 --whole-files";
    let expected_found_matches = true;
    let expected_screen_output = "bogus_file.txt:\non ondon)onl\n";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn only_matches_skip_backwards_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "o.";
    let args = "--matches-only --skip 2 --backwards"; // Skip 2 is on each line.
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:owono \n\
bogus_file.txt:2:ouooouom
bogus_file.txt:3:on
bogus_file.txt:4:ououoy
bogus_file.txt:5:oto ow
bogus_file.txt:6:o \n";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn only_matches_skip_backwards_whole_files_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files --matches-only --skip 2 --backwards";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
onqon.on ond
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn match_line_oriented_quiet_and_not_quiet() {
    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = "on";
    let args = "";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:4:uninteresting content
bogus_file.txt:5:that is only good for
bogus_file.txt:6:tests because no one
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn show_unmatched_lines_quiet_and_not_quiet() {
    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = "on.";
    let args = "--no-match";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:\n\
bogus_file.txt:2:This is a test with
bogus_file.txt:3:multiple lines of very
bogus_file.txt:7:would want to read it.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn show_unmatched_lines_oriented_whole_files_quiet_and_not_quiet() {
    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = "wiggle";
    let args = "--whole-files --no-match";
    let expected_found_matches = false;
    let expected_screen_output = "\
bogus_file.txt:

This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn group_0_match_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "Th(is)";
    let args = "--group 0";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:This
bogus_file.txt:2:This
bogus_file.txt:3:This
bogus_file.txt:4:This
bogus_file.txt:5:This
bogus_file.txt:6:This
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn group_0_match_whole_files_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "Th(is)";
    let args = "--whole-files --group 0";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
ThisThisThisThisThisThis
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn group_1_match_quiet_and_not_quiet() {
    let input = "This is a test. This is a test.";
    let pattern = "Th(is)";
    let args = "--group 1";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:isis
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn group_1_match_whole_files_quiet_and_not_quiet() {
    let input = "This is a test. This is a test.";
    let pattern = "Th(is)";
    let args = "--whole-files --group 1";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
isis
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn group_1_multiple_match_quiet_and_not_quiet() {
    let input = "This is a test.";
    let pattern = "(is)";
    let args = "--group 1";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:isis
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn group_1_multiple_match_whole_files_hquiet_and_not_quiet() {
    let input = "This is a test.";
    let pattern = "(is)";
    let args = "--whole-files --group 1";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
isis
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn group_2_match_quiet_and_not_quiet() {
    let input = "This is a test. This is a test.";
    let pattern = "is (a) (test)";
    let args = "--group 2";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:testtest
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn group_2_match_whole_files_quiet_and_not_quiet() {
    let input = "This is a test. This is a test.";
    let pattern = "is (a) (test)";
    let args = "--whole-files --group 2";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
testtest
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn named_group_match_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is (a) (?P<dave>test)";
    let args = "--group dave";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:1:test
bogus_file.txt:2:test
bogus_file.txt:3:test
bogus_file.txt:4:test
bogus_file.txt:5:test
bogus_file.txt:6:test
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn named_group_match_whole_files_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is (a) (?P<dave>test)";
    let args = "--whole-files --group dave";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
testtesttesttesttesttest
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn invalid_named_group_match_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is (a) (?P<dave>test)";
    let args = "--group christine";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn invalid_named_group_match_whole_files_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is (a) (?P<dave>test)";
    let args = "--whole-files --group christine";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn colored_match_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is";
    let args = "--colors=always";
    let expected_found_matches = true;
    let expected_screen_output = "\
\u{1b}[35mbogus_file.txt:1:\u{1b}[0mTh\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
\u{1b}[35mbogus_file.txt:2:\u{1b}[0mTh\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
\u{1b}[35mbogus_file.txt:3:\u{1b}[0mTh\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
\u{1b}[35mbogus_file.txt:4:\u{1b}[0mTh\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
\u{1b}[35mbogus_file.txt:5:\u{1b}[0mTh\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
\u{1b}[35mbogus_file.txt:6:\u{1b}[0mTh\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn colored_match_whole_files_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is";
    let args = "--whole-files --colors=always";
    let expected_found_matches = true;
    let expected_screen_output = "\
\u{1b}[35mbogus_file.txt:
\u{1b}[0mTh\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
Th\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
Th\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
Th\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
Th\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
Th\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn basic_replace_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is";
    let args = "--replace=at";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "\
That at a test.
That at a test.
That at a test.
That at a test.
That at a test.
That at a test.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn basic_replace_whole_files_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is";
    let args = "--whole-files --replace=at";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "\
That at a test.
That at a test.
That at a test.
That at a test.
That at a test.
That at a test.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn basic_replace_to_stdout_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is";
    let args = "--replace=at --stdout";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
That at a test.
That at a test.
That at a test.
That at a test.
That at a test.
That at a test.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn basic_replace_to_stdout_whole_files_quiet_and_not_quiet() {
    let input = "\
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
This is a test.
";
    let pattern = "is";
    let args = "--whole-files --replace=at --stdout";
    let expected_found_matches = true;
    let expected_screen_output = "\
bogus_file.txt:
That at a test.
That at a test.
That at a test.
That at a test.
That at a test.
That at a test.
";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn replace_skip_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files -r XXX --skip 2";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbXXXbeyXXX a
curse, a few dahlias, and a ribbXXX to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an XXXooker sells a dissident
related to the hand to an ungodly dahlia.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn replace_skip_backwards_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files -r XXX --skip 2 --backwards";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "\
The shadow cXXXuers the hand related to a mastadXXX Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbXXXbeyXXX a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn replace_skip_all_quiet_and_not_quiet() {
    // TODO

    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files -r XXX --skip 10";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn only_matches_skip_all_whole_files_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files --matches-only --skip 10";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn replace_skip_all_backwards_quiet_and_not_quiet() {
    // TODO

    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files -r XXX --skip 10";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn replace_number_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files -r XXX --number 3";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "\
The shadow cXXXuers the hand related to a mastadXXX Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbXXXbeyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn replace_number_backwards_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files -r XXX --number 3 --backwards";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyXXX a
curse, a few dahlias, and a ribbXXX to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an XXXooker sells a dissident
related to the hand to an ungodly dahlia.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn replace_number_more_than_there_are_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files -r XXX --number 10";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "\
The shadow cXXXuers the hand related to a mastadXXX Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbXXXbeyXXX a
curse, a few dahlias, and a ribbXXX to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an XXXooker sells a dissident
related to the hand to an ungodly dahlia.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn replace_number_more_than_there_are_backwards_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files -r XXX --number 10 --backwards";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "\
The shadow cXXXuers the hand related to a mastadXXX Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbXXXbeyXXX a
curse, a few dahlias, and a ribbXXX to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an XXXooker sells a dissident
related to the hand to an ungodly dahlia.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn replace_skip_number_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files -r XXX --skip 2 --number 3";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbXXXbeyXXX a
curse, a few dahlias, and a ribbXXX to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

#[test]
fn replace_skip_number_backwards_quiet_and_not_quiet() {
    let input = "\
The shadow conquers the hand related to a mastadon. Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbon beyond a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";
    let pattern = "on.";
    let args = "--whole-files -r XXX --skip 2 --number 3 --backwards";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "\
The shadow conquers the hand related to a mastadXXX Jespera and I took a cup
around a toothache (with a lunatic around some debutante, a ribbXXXbeyXXX a
curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we
can accurately mourn our boy. When another espadrille wakes up, the cup toward
another swamp flies into a rage. Now and then, an onlooker sells a dissident
related to the hand to an ungodly dahlia.
";

    test(
        input,
        pattern,
        args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
}

fn test(
    input: &str,
    pattern: &str,
    args: &str,
    expected_found_matches: bool,
    expected_screen_output: &str,
    expected_file_content: &str,
) {
    println!("NOT QUIET");
    // The dummy glob argument prevents it from assuming --stdout.
    let args = format!("{} dummy", args);
    really_test(
        input,
        pattern,
        &args,
        expected_found_matches,
        expected_screen_output,
        expected_file_content,
    );
    println!("QUIET");
    let args = format!("{} --quiet dummy", args);
    really_test(
        input,
        pattern,
        &args,
        expected_found_matches,
        "",
        expected_file_content,
    );
}

fn really_test(
    input: &str,
    pattern: &str,
    args: &str,
    expected_found_matches: bool,
    expected_screen_output: &str,
    expected_file_content: &str,
) {
    let mut args = args
        .split_whitespace()
        .map(|arg| arg.to_string())
        .collect::<Vec<String>>();
    args.insert(0, pattern.to_string());
    env::set_var("NED_DEFAULTS", "");
    let options_with_defaults = OptionsWithDefaults::new(make_opts(), &args).unwrap();
    let parameters = get_parameters(&options_with_defaults).unwrap();

    let mut cursor = Cursor::<Vec<u8>>::new(vec![]);
    cursor.write_all(&input.to_string().into_bytes()).unwrap();
    cursor.seek(SeekFrom::Start(0)).unwrap();
    let mut file = Source::Cursor(Box::new(cursor));
    let mut screen_output: Vec<u8> = vec![];

    let found_matches = process_file(
        &mut screen_output,
        &parameters,
        &Some("bogus_file.txt".to_string()),
        &mut file,
    )
    .unwrap();

    let screen_output = String::from_utf8(screen_output).unwrap();

    let file_output;
    let mut buffer = Vec::new();
    if let Source::Cursor(ref mut cursor) = file {
        let _ = cursor.seek(SeekFrom::Start(0));
        let _ = cursor.read_to_end(&mut buffer);
        file_output = String::from_utf8(buffer).unwrap();
    } else {
        panic!("Oh oh?");
    }

    assert_eq!(found_matches, expected_found_matches);
    assert_eq!(screen_output, expected_screen_output);
    assert_eq!(file_output, expected_file_content);
}
