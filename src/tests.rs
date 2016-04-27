use regex::Regex;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use {make_opts, process_file, Source};

#[test]
fn basic_match() {

    let input = "This is a test.";
    let pattern = "is";
    let options = "";
    let expected_exit_code = 0;
    let expected_screen_output = "This is a test.";
    let expected_file_content = &input;

    test(input,
         pattern,
         options,
         expected_exit_code,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn colored_match() {

    let input = "This is a test.";
    let pattern = "is";
    let options = "--colors";
    let expected_exit_code = 0;
    let expected_screen_output = "Th\u{1b}[1;31mis\u{1b}[0m \u{1b}[1;31mis\u{1b}[0m a test.";
    let expected_file_content = &input; // TODO

    test(input,
         pattern,
         options,
         expected_exit_code,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn quiet_match() {

    let input = "This is a test.";
    let pattern = "is";
    let options = "--quiet";
    let expected_exit_code = 0;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(input,
         pattern,
         options,
         expected_exit_code,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn quiet_no_match() {

    let input = "This is a test.";
    let pattern = "as";
    let options = "--quiet";
    let expected_exit_code = 1;
    let expected_screen_output = "";
    let expected_file_content = &input;

    test(input,
         pattern,
         options,
         expected_exit_code,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn basic_replace() {

    let input = "This is a test.";
    let pattern = "is";
    let options = "--replace=at";
    let expected_exit_code = 0;
    let expected_screen_output = "";
    let expected_file_content = "That at a test.";

    test(input,
         pattern,
         options,
         expected_exit_code,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn basic_replace_to_stdout() {

    let input = "This is a test.";
    let pattern = "is";
    let options = "--replace=at --stdout";
    let expected_exit_code = 0;
    let expected_screen_output = "That at a test.";
    let expected_file_content = &input;

    test(input,
         pattern,
         options,
         expected_exit_code,
         expected_screen_output,
         expected_file_content);
}

fn test(input: &str,
        pattern: &str,
        options: &str,
        expected_exit_code: i32,
        expected_screen_output: &str,
        expected_file_content: &str) {

    let re = Regex::new(pattern).unwrap();

    let mut cursor = Cursor::<Vec<u8>>::new(vec![]);
    let _ = cursor.write(&input.to_string().into_bytes());
    let _ = cursor.seek(SeekFrom::Start(0));
    let mut file = Source::Cursor(Box::new(cursor));
    let mut screen_output: Vec<u8> = vec![];

    let opts = make_opts();
    let options: Vec<&str> = options.split_whitespace().collect();
    let matches = opts.parse(&options).unwrap();

    let exit_code = process_file(&re, &matches, &mut file, &mut screen_output).unwrap();

    let screen_output = String::from_utf8(screen_output).unwrap();

    let file_output;
    let mut buffer = Vec::new();
    if let Source::Cursor(ref mut cursor) = file {
        let _ = cursor.seek(SeekFrom::Start(0));
        let _ = cursor.read_to_end(&mut buffer);
        file_output = String::from_utf8(buffer).unwrap();
    } else {
        panic!("WTF?");
    }

    assert_eq!(exit_code, expected_exit_code);
    assert_eq!(screen_output, expected_screen_output);
    assert_eq!(file_output, expected_file_content);
}
