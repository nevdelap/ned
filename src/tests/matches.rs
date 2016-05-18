/// Test match related functionality - different types of matches, matches with color, quiet, etc.
/// The use of re, not re itself.

use process_file;
use opts::make_opts;
use parameters::get_parameters;
use source::Source;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

#[test]
fn basic_match_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "is";
    let args = "--whole-files";
    let expected_found_matches = true;
    let expected_screen_output = "bogus_file.txt:\nThis is a test.\n";
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
    let pattern = "wiggle";
    let args = "--whole-files";
    let expected_found_matches = false;
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
fn ignore_case_match_quiet_and_not_quiet() {

    let input = "This is a test.";
    let pattern = "IS";
    let args = "--whole-files --ignore-case";
    let expected_found_matches = true;
    let expected_screen_output = "bogus_file.txt:\nThis is a test.\n";
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
    let args = "--whole-files --single";
    let expected_found_matches = true;
    let expected_screen_output = &format!("bogus_file.txt:\n{}", input);
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
    let args = "--whole-files --multiline";
    let expected_found_matches = true;
    let expected_screen_output = &format!("bogus_file.txt:\n{}", &input);
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
    let args = "--whole-files --multiline";
    let expected_found_matches = true;
    let expected_screen_output = &format!("bogus_file.txt:\n{}", input);
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
    let args = "--whole-files --single --multiline";
    let expected_found_matches = true;
    let expected_screen_output = &format!("bogus_file.txt:\n{}", input);
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
    let args = "--whole-files --extended";
    let expected_found_matches = true;
    let expected_screen_output = "bogus_file.txt:\nThis is a test.\n";
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
    let args = "--whole-files --matches-only";
    let expected_found_matches = true;
    let expected_screen_output = "bogus_file.txt:\nis\nis\n";
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
    let args = "";
    let expected_found_matches = true;
    let expected_screen_output = "bogus_file.txt: uninteresting content\nbogus_file.txt: that is \
                                  only good for\nbogus_file.txt: tests because no one\n";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
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
    let pattern = "wiggle";
    let args = "--whole-files --no-match";
    let expected_found_matches = false;
    let expected_screen_output = &format!("bogus_file.txt:\n{}", input);
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn show_unmatched_lines_oriented_quiet_and_not_quiet() {

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
    let expected_screen_output = "bogus_file.txt:\n
This is a test with
multiple lines of very
uninteresting content
that is only good for
tests because no one
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
    let args = "--whole-files --group 0";
    let expected_found_matches = true;
    let expected_screen_output = "bogus_file.txt:\nThis\n";
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
    let args = "--whole-files --group 1";
    let expected_found_matches = true;
    let expected_screen_output = "bogus_file.txt:\nis\n";
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
    let args = "--whole-files --group 2";
    let expected_found_matches = true;
    let expected_screen_output = "bogus_file.txt:\ntest\n";
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
    let args = "--whole-files --group dave";
    let expected_found_matches = true;
    let expected_screen_output = "bogus_file.txt:\ntest\n";
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
    let args = "--whole-files --colors";
    let expected_found_matches = true;
    let expected_screen_output = "\u{1b}[35mbogus_file.txt\u{1b}[0m:\nTh\u{1b}[1;31mis\u{1b}[0m \
                                  \u{1b}[1;31mis\u{1b}[0m a test.\n";
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
    let args = "--whole-files --replace=at";
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
    let args = "--whole-files --replace=at --stdout";
    let expected_found_matches = true;
    let expected_screen_output = "bogus_file.txt:\nThat at a test.";
    let expected_file_content = &input;

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn replace_skip_quiet_and_not_quiet() {

    let input = "The shadow conquers the hand related to a mastadon. Jespera and I took a cup \
                 around a toothache (with a lunatic around some debutante, a ribbon beyond a \
                 curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we \
                 can accurately mourn our boy. When another espadrille wakes up, the cup toward \
                 another swamp flies into a rage. Now and then, an onlooker sells a dissident \
                 related to the hand to an ungodly dahlia.";
    let pattern = "on.";
    let args = "--whole-files -r XXX --skip 2";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "The shadow conquers the hand related to a mastadon. Jespera and \
                                  I took a cup around a toothache (with a lunatic around some \
                                  debutante, a ribbXXXbeyXXX a curse, a few dahlias, and a \
                                  ribbXXX to arrive at a state of intimacy where we can \
                                  accurately mourn our boy. When another espadrille wakes up, the \
                                  cup toward another swamp flies into a rage. Now and then, an \
                                  XXXooker sells a dissident related to the hand to an ungodly \
                                  dahlia.";

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn replace_skip_backwards_quiet_and_not_quiet() {

    let input = "The shadow conquers the hand related to a mastadon. Jespera and I took a cup \
                 around a toothache (with a lunatic around some debutante, a ribbon beyond a \
                 curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we \
                 can accurately mourn our boy. When another espadrille wakes up, the cup toward \
                 another swamp flies into a rage. Now and then, an onlooker sells a dissident \
                 related to the hand to an ungodly dahlia.";
    let pattern = "on.";
    let args = "--whole-files -r XXX --skip 2 --backwards";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "The shadow cXXXuers the hand related to a mastadXXX Jespera and \
                                  I took a cup around a toothache (with a lunatic around some \
                                  debutante, a ribbXXXbeyXXX a curse, a few dahlias, and a \
                                  ribbon) to arrive at a state of intimacy where we can \
                                  accurately mourn our boy. When another espadrille wakes up, the \
                                  cup toward another swamp flies into a rage. Now and then, an \
                                  onlooker sells a dissident related to the hand to an ungodly \
                                  dahlia.";

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn replace_skip_all_quiet_and_not_quiet() {

    let input = "The shadow conquers the hand related to a mastadon. Jespera and I took a cup \
                 around a toothache (with a lunatic around some debutante, a ribbon beyond a \
                 curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we \
                 can accurately mourn our boy. When another espadrille wakes up, the cup toward \
                 another swamp flies into a rage. Now and then, an onlooker sells a dissident \
                 related to the hand to an ungodly dahlia.";
    let pattern = "on.";
    let args = "--whole-files -r XXX --skip 10";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = "The shadow conquers the hand related to a mastadon. Jespera and \
                                  I took a cup around a toothache (with a lunatic around some \
                                  debutante, a ribbon beyond a curse, a few dahlias, and a \
                                  ribbon) to arrive at a state of intimacy where we can \
                                  accurately mourn our boy. When another espadrille wakes up, the \
                                  cup toward another swamp flies into a rage. Now and then, an \
                                  onlooker sells a dissident related to the hand to an ungodly \
                                  dahlia.";

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn replace_skip_all_backwards_quiet_and_not_quiet() {

    let input = "The shadow conquers the hand related to a mastadon. Jespera and I took a cup \
                 around a toothache (with a lunatic around some debutante, a ribbon beyond a \
                 curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we \
                 can accurately mourn our boy. When another espadrille wakes up, the cup toward \
                 another swamp flies into a rage. Now and then, an onlooker sells a dissident \
                 related to the hand to an ungodly dahlia.";
    let pattern = "on.";
    let args = "--whole-files -r XXX --skip 10";
    let expected_found_matches = false;
    let expected_screen_output = "";
    let expected_file_content = "The shadow conquers the hand related to a mastadon. Jespera and \
                                  I took a cup around a toothache (with a lunatic around some \
                                  debutante, a ribbon beyond a curse, a few dahlias, and a \
                                  ribbon) to arrive at a state of intimacy where we can \
                                  accurately mourn our boy. When another espadrille wakes up, the \
                                  cup toward another swamp flies into a rage. Now and then, an \
                                  onlooker sells a dissident related to the hand to an ungodly \
                                  dahlia.";

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn replace_number_quiet_and_not_quiet() {

    let input = "The shadow conquers the hand related to a mastadon. Jespera and I took a cup \
                 around a toothache (with a lunatic around some debutante, a ribbon beyond a \
                 curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we \
                 can accurately mourn our boy. When another espadrille wakes up, the cup toward \
                 another swamp flies into a rage. Now and then, an onlooker sells a dissident \
                 related to the hand to an ungodly dahlia.";
    let pattern = "on.";
    let args = "--whole-files -r XXX --number 3";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "The shadow cXXXuers the hand related to a mastadXXX Jespera and \
                                  I took a cup around a toothache (with a lunatic around some \
                                  debutante, a ribbXXXbeyond a curse, a few dahlias, and a \
                                  ribbon) to arrive at a state of intimacy where we can \
                                  accurately mourn our boy. When another espadrille wakes up, the \
                                  cup toward another swamp flies into a rage. Now and then, an \
                                  onlooker sells a dissident related to the hand to an ungodly \
                                  dahlia.";

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn replace_number_backwards_quiet_and_not_quiet() {

    let input = "The shadow conquers the hand related to a mastadon. Jespera and I took a cup \
                 around a toothache (with a lunatic around some debutante, a ribbon beyond a \
                 curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we \
                 can accurately mourn our boy. When another espadrille wakes up, the cup toward \
                 another swamp flies into a rage. Now and then, an onlooker sells a dissident \
                 related to the hand to an ungodly dahlia.";
    let pattern = "on.";
    let args = "--whole-files -r XXX --number 3 --backwards";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "The shadow conquers the hand related to a mastadon. Jespera and \
                                  I took a cup around a toothache (with a lunatic around some \
                                  debutante, a ribbon beyXXX a curse, a few dahlias, and a \
                                  ribbXXX to arrive at a state of intimacy where we can \
                                  accurately mourn our boy. When another espadrille wakes up, the \
                                  cup toward another swamp flies into a rage. Now and then, an \
                                  XXXooker sells a dissident related to the hand to an ungodly \
                                  dahlia.";

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn replace_number_more_than_there_are_quiet_and_not_quiet() {

    let input = "The shadow conquers the hand related to a mastadon. Jespera and I took a cup \
                 around a toothache (with a lunatic around some debutante, a ribbon beyond a \
                 curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we \
                 can accurately mourn our boy. When another espadrille wakes up, the cup toward \
                 another swamp flies into a rage. Now and then, an onlooker sells a dissident \
                 related to the hand to an ungodly dahlia.";
    let pattern = "on.";
    let args = "--whole-files -r XXX --number 10";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "The shadow cXXXuers the hand related to a mastadXXX Jespera and \
                                  I took a cup around a toothache (with a lunatic around some \
                                  debutante, a ribbXXXbeyXXX a curse, a few dahlias, and a \
                                  ribbXXX to arrive at a state of intimacy where we can \
                                  accurately mourn our boy. When another espadrille wakes up, the \
                                  cup toward another swamp flies into a rage. Now and then, an \
                                  XXXooker sells a dissident related to the hand to an ungodly \
                                  dahlia.";

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn replace_number_more_than_there_are_backwards_quiet_and_not_quiet() {

    let input = "The shadow conquers the hand related to a mastadon. Jespera and I took a cup \
                 around a toothache (with a lunatic around some debutante, a ribbon beyond a \
                 curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we \
                 can accurately mourn our boy. When another espadrille wakes up, the cup toward \
                 another swamp flies into a rage. Now and then, an onlooker sells a dissident \
                 related to the hand to an ungodly dahlia.";
    let pattern = "on.";
    let args = "--whole-files -r XXX --number 10";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "The shadow cXXXuers the hand related to a mastadXXX Jespera and \
                                  I took a cup around a toothache (with a lunatic around some \
                                  debutante, a ribbXXXbeyXXX a curse, a few dahlias, and a \
                                  ribbXXX to arrive at a state of intimacy where we can \
                                  accurately mourn our boy. When another espadrille wakes up, the \
                                  cup toward another swamp flies into a rage. Now and then, an \
                                  XXXooker sells a dissident related to the hand to an ungodly \
                                  dahlia.";

    test(input,
         pattern,
         args,
         expected_found_matches,
         expected_screen_output,
         expected_file_content);
}

#[test]
fn replace_skip_number_quiet_and_not_quiet() {

    let input = "The shadow conquers the hand related to a mastadon. Jespera and I took a cup \
                 around a toothache (with a lunatic around some debutante, a ribbon beyond a \
                 curse, a few dahlias, and a ribbon) to arrive at a state of intimacy where we \
                 can accurately mourn our boy. When another espadrille wakes up, the cup toward \
                 another swamp flies into a rage. Now and then, an onlooker sells a dissident \
                 related to the hand to an ungodly dahlia.";
    let pattern = "on.";
    let args = "--whole-files -r XXX --skip 2 --number 3";
    let expected_found_matches = true;
    let expected_screen_output = "";
    let expected_file_content = "The shadow conquers the hand related to a mastadon. Jespera and \
                                  I took a cup around a toothache (with a lunatic around some \
                                  debutante, a ribbXXXbeyXXX a curse, a few dahlias, and a \
                                  ribbXXX to arrive at a state of intimacy where we can \
                                  accurately mourn our boy. When another espadrille wakes up, the \
                                  cup toward another swamp flies into a rage. Now and then, an \
                                  onlooker sells a dissident related to the hand to an ungodly \
                                  dahlia.";

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
    let parameters = get_parameters(&opts, &args).unwrap();

    let mut cursor = Cursor::<Vec<u8>>::new(vec![]);
    cursor.write(&input.to_string().into_bytes()).unwrap();
    cursor.seek(SeekFrom::Start(0)).unwrap();
    let mut file = Source::Cursor(Box::new(cursor));
    let mut screen_output: Vec<u8> = vec![];

    let found_matches = process_file(&parameters,
                                     &Some("bogus_file.txt".to_string()),
                                     &mut file,
                                     &mut screen_output)
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
