use parameters::Parameters;

#[test]
fn number_normal_range() {
    // number, skip, index, count, forward_expected, backward_expected
    test_include_match(Some(4), 0, 0, 10, true, false);
    test_include_match(Some(4), 0, 3, 10, true, false);
    test_include_match(Some(4), 0, 4, 10, false, false);
    test_include_match(Some(4), 0, 6, 10, false, true);
    test_include_match(Some(4), 0, 10, 10, false, false);
    test_include_match(Some(4), 0, 100, 10, false, false);
}

#[test]
fn number_zero() {
    // number, skip, index, count, forward_expected, backward_expected
    test_include_match(Some(0), 0, 0, 10, false, false);
    test_include_match(Some(0), 0, 4, 10, false, false);
    test_include_match(Some(0), 0, 10, 10, false, false);
    test_include_match(Some(0), 0, 100, 10, false, false);
}

#[test]
fn number_too_many() {
    // number, skip, index, count, forward_expected, backward_expected
    test_include_match(Some(11), 0, 0, 10, true, true);
}

#[test]
fn skip_normal_range() {
    // number, skip, index, count, forward_expected, backward_expected
    test_include_match(None, 0, 0, 10, true, true);
    test_include_match(None, 3, 0, 10, false, true);
    test_include_match(None, 3, 2, 10, false, true);
    test_include_match(None, 3, 4, 10, true, true);
    test_include_match(None, 3, 6, 10, true, true);
    test_include_match(None, 3, 7, 10, true, false);
    test_include_match(None, 3, 8, 10, true, false);
    test_include_match(None, 3, 9, 10, true, false);
    test_include_match(None, 3, 10, 10, false, false);
    test_include_match(None, 3, 100, 10, false, false);
}

#[test]
fn skip_all() {
    // number, skip, index, count, forward_expected, backward_expected
    test_include_match(None, 10, 0, 10, false, false);
    test_include_match(None, 10, 5, 10, false, false);
    test_include_match(None, 10, 9, 10, false, false);
    test_include_match(None, 10, 10, 10, false, false);
    test_include_match(None, 10, 100, 10, false, false);
}

#[test]
fn skip_too_many() {
    // number, skip, index, count, forward_expected, backward_expected
    test_include_match(None, 11, 0, 10, false, false);
    test_include_match(None, 11, 5, 10, false, false);
    test_include_match(None, 11, 9, 10, false, false);
    test_include_match(None, 11, 10, 10, false, false);
    test_include_match(None, 11, 100, 10, false, false);
}

fn test_include_match(number: Option<usize>,
                      skip: usize,
                      index: usize,
                      count: usize,
                      forward_expected: bool,
                      backward_expected: bool) {
    let mut parameters = Parameters {
        all: false,
        backwards: false,
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
    assert_eq!(parameters.include_match(index, count), forward_expected);

    parameters.backwards = true;
    assert_eq!(parameters.include_match(index, count), backward_expected);
}
