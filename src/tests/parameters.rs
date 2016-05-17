use parameters::Parameters;

#[test]
fn number_normal_range() {
    // Normal range.
    test_include_match(Some(4), 0, false, 0, 10, true);
    test_include_match(Some(4), 0, false, 3, 10, true);
    test_include_match(Some(4), 0, false, 4, 10, false);
    test_include_match(Some(4), 0, false, 5, 10, false);
}

#[test]
fn number_zero() {
    test_include_match(Some(0), 0, false, 0, 10, false);
    test_include_match(Some(0), 0, false, 10, 10, false);
}

#[test]
fn number_too_many() {
    test_include_match(Some(11), 0, false, 0, 10, true);
}

#[test]
fn skip_normal_range() {
    test_include_match(None, 0, false, 0, 10, true);
    test_include_match(None, 3, false, 0, 10, false);
    test_include_match(None, 3, false, 2, 10, false);
    test_include_match(None, 3, false, 4, 10, true);
    test_include_match(None, 3, false, 5, 10, true);
}

#[test]
fn skip_all() {
    test_include_match(None, 10, false, 5, 10, false);
}

#[test]
fn skip_too_many() {
    test_include_match(None, 11, false, 5, 10, false);
}

fn test_include_match(number: Option<usize>,
                      skip: usize,
                      backwards: bool,
                      index: usize,
                      count: usize,
                      expected: bool) {
    let parameters = Parameters {
        all: false,
        backwards: backwards,
        colors: false,
        exclude_dirs: vec![],
        excludes: vec![],
        filenames: false,
        follow: false,
        globs: vec![],
        group: None,
        help: false,
        ignore_non_utf8: false,
        includes: vec![],
        no_filenames: false,
        no_match: false,
        number: number,
        only_matches: false,
        quiet: false,
        regex: None,
        recursive: false,
        replace: None,
        skip: skip,
        stdin: false,
        stdout: false,
        version: false,
        whole_files: false,
    };
    assert_eq!(parameters.include_match(index, count), expected);
}
