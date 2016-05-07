/// Test match related functionality - different types of matches, matches with color, quiet, etc.
/// The use of re, not re itself.

use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use {get_parameters, make_opts, process_file, Source};

#[test]
fn basic_match_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "is";
    let args = "";
    let expected_found_matches = true;
    let expected_screen_output = "This is a test.";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn ignore_case_match_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "IS";
    let args = "--ignore-case";
    let expected_found_matches = true;
    let expected_screen_output = "This is a test.";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
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
    let expected_found_matches = true;
    let expected_screen_output = &input;
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
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
    let expected_found_matches = true;
    let expected_screen_output = &input;
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
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
    let expected_found_matches = true;
    let expected_screen_output = &input;
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
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
    let expected_found_matches = true;
    let expected_screen_output = &input;
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn extended_match_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "
# this is a commented
# regex that will find
# the word is.
is # Look, that's it!
# Cool magool.";
    let args = "--extended";
    let expected_found_matches = true;
    let expected_screen_output = "This is a test.";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn only_matches_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "is";
    let args = "--only-matches";
    let expected_found_matches = true;
    let expected_screen_output = "isis";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
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
    let args = "--line-oriented";
    let expected_found_matches = true;
    let expected_screen_output = "\
uninteresting content
that is only good for
tests because no one
";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn no_match_quiet_and_not_quiet() {

    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = "on";
    let args = "--no-match";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn no_match_line_oriented_quiet_and_not_quiet() {

    let input = "
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
would want to read it.
";
    let pattern = "on";
    let args = "--no-match --line-oriented";
    let expected_found_matches = true;
    let expected_screen_output = "
This is a test with
multiple lines of very
would want to read it.
";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn group_0_match_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "Th(is)";
    let args = "--group 0";
    let expected_found_matches = true;
    let expected_screen_output = "This";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn group_1_match_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "Th(is)";
    let args = "--group 1";
    let expected_found_matches = true;
    let expected_screen_output = "is";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn group_2_match_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "is (a) (test)";
    let args = "--group 2";
    let expected_found_matches = true;
    let expected_screen_output = "test";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn named_group_match_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "is (a) (?P<dave>test)";
    let args = "--group dave";
    let expected_found_matches = true;
    let expected_screen_output = "test";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn colored_match_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "is";
    let args = "--colors";
    let expected_found_matches = true;
    let expected_screen_output = "Th\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn basic_replace_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "is";
    let args = "--replace=at";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "That at a test.";

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn basic_replace_to_stdout_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "is";
    let args = "--replace=at --stdout";
    let expected_found_matches = true;
    let expected_screen_output = "That at a test.";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

fn test(input: &str,
        pattern: &str,
        args: &str,
        expected_found_matches: bool,
        expected_screen_output: &str,
        expected_file_content: &str) {

    really_test(input,
                pattern,
                args,
                expected_found_matches,
                expected_screen_output,
                expected_file_content);
    let args = format!("{} --quiet", args);
    really_test(input,
                pattern,
                &args,
                expected_found_matches,
                "",
                expected_file_content);
}

fn really_test(input: &str,
               pattern: &str,
               args: &str,
               expected_found_matches: bool,
               expected_screen_output: &str,
               expected_file_content: &str) {

    let opts = make_opts();
    let mut args = args.split_whitespace().map(|arg| arg.to_string()).collect::<Vec<String>>();
    args.insert(0, pattern.to_string());
    let mut parameters = get_parameters(&opts, &args).unwrap();

    let mut cursor = Cursor::<Vec<u8>>::new(vec![]);
    cursor.write(&input.to_string().into_bytes()).unwrap();
    cursor.seek(SeekFrom::Start(0)).unwrap();
    let mut file = Source::Cursor(Box::new(cursor));
    let mut screen_output: Vec<u8> = vec![];

    let found_matches = process_file(&parameters, &mut file, &mut screen_output).unwrap();

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
