use {process_file, Source};
use regex::Regex;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

#[test]
fn basic_match() {

    let input = "This is a test.";
    let re = "is";
    let colors = false;
    let group = None;
    let invert_match = false;
    let line_oriented = false;
    let only_matches = false;
    let quiet = false;
    let replace = None;
    let stdout = false;

    let (exit_code, screen_output, file_content) = test(input,
                                                        re,
                                                        colors,
                                                        &group,
                                                        invert_match,
                                                        line_oriented,
                                                        only_matches,
                                                        quiet,
                                                        &replace,
                                                        stdout);

    assert_eq!(exit_code, 0);
    assert_eq!(screen_output, "isis");
    assert_eq!(file_content, input);
}

#[test]
fn colored_match() {

    let input = "This is a test.";
    let re = "is";
    let colors = true;
    let group = None;
    let invert_match = false;
    let line_oriented = false;
    let only_matches = false;
    let quiet = false;
    let replace = None;
    let stdout = false;

    let (exit_code, screen_output, file_content) = test(input,
                                                        re,
                                                        colors,
                                                        &group,
                                                        invert_match,
                                                        line_oriented,
                                                        only_matches,
                                                        quiet,
                                                        &replace,
                                                        stdout);

    assert_eq!(exit_code, 0);
    assert_eq!(screen_output, "isis");
    assert_eq!(file_content, input);
}

#[test]
fn quiet_match() {

    let input = "This is a test.";
    let re = "is";
    let colors = false;
    let group = None;
    let invert_match = false;
    let line_oriented = false;
    let only_matches = false;
    let quiet = true;
    let replace = None;
    let stdout = false;

    let (exit_code, screen_output, file_content) = test(input,
                                                        re,
                                                        colors,
                                                        &group,
                                                        invert_match,
                                                        line_oriented,
                                                        only_matches,
                                                        quiet,
                                                        &replace,
                                                        stdout);

    assert_eq!(exit_code, 0);
    assert_eq!(screen_output, "");
    assert_eq!(file_content, input);
}

#[test]
fn quiet_no_match() {

    let input = "This is a test.";
    let re = "as";
    let colors = false;
    let group = None;
    let invert_match = false;
    let line_oriented = false;
    let only_matches = false;
    let quiet = true;
    let replace = None;
    let stdout = false;

    let (exit_code, screen_output, file_content) = test(input,
                                                        re,
                                                        colors,
                                                        &group,
                                                        invert_match,
                                                        line_oriented,
                                                        only_matches,
                                                        quiet,
                                                        &replace,
                                                        stdout);

    assert_eq!(exit_code, 1);
    assert_eq!(screen_output, "");
    assert_eq!(file_content, input);
}

fn test(input: &str,
        re: &str,
        colors: bool,
        group: &Option<String>,
        invert_match: bool,
        line_oriented: bool,
        only_matches: bool,
        quiet: bool,
        replace: &Option<String>,
        stdout: bool)
        -> (i32, String, String) {

    let re = Regex::new(re).unwrap();

    let mut cursor = Cursor::<Vec<u8>>::new(vec![]);
    let _ = cursor.write(&input.to_string().into_bytes());
    let _ = cursor.seek(SeekFrom::Start(0));
    let mut file = Source::Cursor(Box::new(cursor));
    let mut screen_output: Vec<u8> = vec![];

    let exit_code = process_file(&re,
                                 colors,
                                 &group,
                                 invert_match,
                                 line_oriented,
                                 only_matches,
                                 quiet,
                                 replace,
                                 stdout,
                                 &mut file,
                                 &mut screen_output)
                        .unwrap();

    let screen_output = String::from_utf8(screen_output).unwrap();
    let file_content;

    let mut buffer = Vec::new();
    if let Source::Cursor(ref mut cursor) = file {
        let _ = cursor.seek(SeekFrom::Start(0));
        let _ = cursor.read_to_end(&mut buffer);
        file_content = String::from_utf8(buffer).unwrap();
    } else {
        panic!("WTF?");
    }

    (exit_code, screen_output, file_content)
}

#[test]
fn basic_replace() {

    let input = "This is a test.";
    let re = "is";
    let colors = false;
    let group = None;
    let invert_match = false;
    let line_oriented = false;
    let only_matches = false;
    let quiet = false;
    let replace = Some("at".to_string());
    let stdout = false;

    let (exit_code, screen_output, file_content) = test(input,
                                                        re,
                                                        colors,
                                                        &group,
                                                        invert_match,
                                                        line_oriented,
                                                        only_matches,
                                                        quiet,
                                                        &replace,
                                                        stdout);

    assert_eq!(exit_code, 0);
    assert_eq!(screen_output, "");
    assert_eq!(file_content, "That at a test.");
}

#[test]
fn basic_replace_to_stdout() {

    let input = "This is a test.";
    let re = "is";
    let colors = false;
    let group = None;
    let invert_match = false;
    let line_oriented = false;
    let only_matches = false;
    let quiet = false;
    let replace = Some("at".to_string());
    let stdout = true;

    let (exit_code, screen_output, file_content) = test(input,
                                                        re,
                                                        colors,
                                                        &group,
                                                        invert_match,
                                                        line_oriented,
                                                        only_matches,
                                                        quiet,
                                                        &replace,
                                                        stdout);

    assert_eq!(exit_code, 0);
    assert_eq!(screen_output, "That at a test.");
    assert_eq!(file_content, "This is a test.");
}

#[test]
#[should_panic]
fn something_else() {
    assert_eq!(1, 0);
}
