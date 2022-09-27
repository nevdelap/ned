//
// ned, https://github.com/nevdelap/ned, tests/parameters.rs
//
// Copyright 2016-2022 Nev Delap (nevdelap at gmail)
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

use crate::parameters::Parameters;

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

fn test_include_match(
    number: Option<usize>,
    skip: usize,
    index: usize,
    count: usize,
    forward_expected: bool,
    backward_expected: bool,
) {
    let mut parameters = Parameters {
        all: false,
        backwards: false,
        case_replacements: false,
        colors: false,
        context_after: 0,
        context_before: 0,
        exclude_dirs: vec![],
        excludes: vec![],
        file_names_only: false,
        follow: false,
        globs: vec![],
        group: None,
        help: false,
        ignore_non_utf8: false,
        includes: vec![],
        line_numbers_only: false,
        matches_only: false,
        no_file_names: false,
        no_line_numbers: false,
        no_match: false,
        number,
        quiet: false,
        regex: None,
        recursive: false,
        replace: None,
        skip,
        stdin: false,
        stdout: false,
        version: false,
        whole_files: false,
    };
    assert_eq!(parameters.include_match(index, count), forward_expected);

    parameters.backwards = true;
    assert_eq!(parameters.include_match(index, count), backward_expected);
}
