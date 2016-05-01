use ned;

#[test]
fn basic_match() {

    let options = "accidentally test";
    let expected_exit_code = 0;
    let expected_screen_output = "The accidentally ghastly hand plans an escape from a cream puff \
                                  the placid widow. A slovenly\nonlooker rejoices, because some \
                                  single-handledly sheepish stalactite knowingly avoids contact \
                                  with a\nwisely rhetorical ballerina. Sometimes the waif about a \
                                  swamp rejoices, but a ruffian always barely\nbefriends an \
                                  unseemly dilettante! Unlike so many mastadons who have made \
                                  their lovely widow\nabhorrent to us, waifs remain womanly.";

    test(&options, expected_exit_code, &expected_screen_output);
}

#[test]
fn only_matches() {

    let options = "accidentally.*hand test --only-matches";
    let expected_exit_code = 0;
    let expected_screen_output = "accidentally ghastly hand";

    test(&options, expected_exit_code, &expected_screen_output);
}

#[test]
fn colored_match() {

    let options = "accidentally.*hand test --colors";
    let expected_exit_code = 0;
    let expected_screen_output = "The \u{1b}[1;31maccidentally ghastly hand\u{1b}[0m plans an \
                                  escape from a cream puff the placid widow. A slovenly\nonlooker \
                                  rejoices, because some single-handledly sheepish stalactite \
                                  knowingly avoids contact with a\nwisely rhetorical ballerina. \
                                  Sometimes the waif about a swamp rejoices, but a ruffian always \
                                  barely\nbefriends an unseemly dilettante! Unlike so many \
                                  mastadons who have made their lovely widow\nabhorrent to us, \
                                  waifs remain womanly.";

    test(&options, expected_exit_code, &expected_screen_output);
}

fn test(options: &str, expected_exit_code: i32, expected_screen_output: &str) {
    let program = "ned";
    let args: Vec<String> = options.split_whitespace()
                                   .map(|arg| arg.to_string())
                                   .collect::<Vec<String>>();
    let mut screen_output: Vec<u8> = vec![];

    let exit_code = ned(&program, &args, &mut screen_output).unwrap();

    let screen_output = String::from_utf8(screen_output).unwrap();

    assert_eq!(exit_code, expected_exit_code);
    assert_eq!(screen_output, expected_screen_output);
}
