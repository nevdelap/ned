//
// ned, https://github.com/nevdelap/ned, tests/files.rs
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

/// Test file related functionality - recursion, inclusion, exclusion, symlinks, etc.
use crate::files::Files;
use crate::options_with_defaults::OptionsWithDefaults;
use crate::opts::make_opts;
use crate::parameters::get_parameters;
use std::env;
use std::path::{Path, PathBuf};

// These tests are not running `ned` to find a pattern in the test files, they
// are simulating so to test `Files`. To compare the results of running `ned` to
// running the tests run `ned` with '.*' in place of 'pattern' in `args`, and
// with `-f` to show only the files. For example `ned -f '.*' test` will give
// the same results as the first test, no_recursion.

#[test]
fn no_recursion() {
    let test_path = Path::new("test");
    let mut expected_file_names = vec![
        test_path.join("file1.txt"),
        test_path.join("file9.txt"),
        test_path.join("longfile.txt"),
    ];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(1, test_path.join("file8.txt"));
    }
    test("pattern test", &expected_file_names);
    // A leading ./ results in the same normalised relative paths.
    test("pattern ./test", &expected_file_names);
}

#[test]
fn no_recursion_all() {
    let test_path = Path::new("test");
    let mut expected_file_names = vec![
        test_path.join(".hidden_file1"),
        test_path.join("file1.txt"),
        test_path.join("file9.txt"),
        test_path.join("longfile.txt"),
    ];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(2, test_path.join("file8.txt"));
    }
    test("pattern --all test", &expected_file_names);
}

#[test]
fn no_recursion_follow() {
    let test_path = Path::new("test");
    let expected_file_names = vec![
        test_path.join("file1.txt"),
        test_path.join("file8.txt"),  // TEST ON WINDOWS
        test_path.join("file9.txt"),
        test_path.join("longfile.txt"),
    ];
    test("pattern --follow test", &expected_file_names);
    // A leading ./ results in the same normalised relative paths.
    test("pattern --follow ./test", &expected_file_names);
}

#[test]
fn no_recursion_follow_all() {
    let test_path = Path::new("test");
    let expected_file_names = vec![
        test_path.join(".hidden_file1"),
        test_path.join("file1.txt"),
        test_path.join("file8.txt"),
        test_path.join("file9.txt"),
        test_path.join("longfile.txt"),
    ];
    test("pattern --follow --all test", &expected_file_names);
}

#[test]
fn recursion() {
    let test_path = Path::new("test");
    let mut expected_file_names = vec![
        test_path.join("dir1").join("dir4").join("dir5").join("file7.txt"),
        test_path.join("dir1").join("dir4").join("file6.txt"),
        test_path.join("dir1").join("file2.txt"),
        test_path.join("dir1").join("file3.txt"),
        test_path.join("dir2").join("file4.txt"),
        test_path.join("dir3").join("file5.txt"),
        test_path.join("file1.txt"),
        test_path.join("file9.txt"),
        test_path.join("longfile.txt"),
    ];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(7, test_path.join("file8.txt"));
    }
    test("pattern --recursive test", &expected_file_names);
}

#[test]
fn recursion_all() {
    let test_path = Path::new("test");
    let mut expected_file_names = vec![
        test_path.join(".hidden_dir").join(".hidden_file2"),
        test_path.join(".hidden_dir").join("file10.txt"),
        test_path.join(".hidden_file1"),
        test_path.join("dir1").join("dir4").join("dir5").join("file7.txt"),
        test_path.join("dir1").join("dir4").join("file6.txt"),
        test_path.join("dir1").join("file2.txt"),
        test_path.join("dir1").join("file3.txt"),
        test_path.join("dir2").join("file4.txt"),
        test_path.join("dir3").join("file5.txt"),
        test_path.join("file1.txt"),
        test_path.join("file9.txt"),
        test_path.join("longfile.txt"),
    ];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(10, test_path.join("file8.txt"));
    }
    test("pattern --recursive --all test", &expected_file_names);
}

#[test]
fn recursion_follow() {
    let test_path = Path::new("test");
    let expected_file_names = vec![
        test_path.join("file1.txt"),
        test_path.join("file8.txt"),
        test_path.join("file9.txt"),
        test_path.join("longfile.txt"),
    ];
    test("pattern --follow test", &expected_file_names);
}

#[test]
fn recursion_follow_all() {
    let test_path = Path::new("test");
    let expected_file_names = vec![
        test_path.join(".hidden_dir").join(".hidden_file2"),
        test_path.join(".hidden_dir").join("file10.txt"),
        test_path.join(".hidden_file1"),
        test_path.join("dir1").join("dir4").join("dir5").join("file7.txt"),
        test_path.join("dir1").join("dir4").join("file6.txt"),
        test_path.join("dir1").join("file2.txt"),
        test_path.join("dir1").join("file3.txt"),
        test_path.join("dir2").join("file4.txt"),
        test_path.join("dir3").join("file5.txt"),
        test_path.join("file1.txt"),
        test_path.join("file8.txt"),
        test_path.join("file9.txt"),
        test_path.join("longfile.txt"),
    ];
    test("pattern --recursive --follow --all test", &expected_file_names);
}

#[test]
fn include_files() {
    let test_path = Path::new("test");
    let expected_file_names = vec![test_path.join("dir1").join("dir4").join("dir5").join("file7.txt")];
    test("pattern -R test --include file7*", &expected_file_names);
}

#[test]
fn exclude_files() {
    let test_path = Path::new("test");
    let args = "pattern -R test --exclude file7*";
    let mut expected_file_names = vec![
        test_path.join("dir1").join("dir4").join("file6.txt"),
        test_path.join("dir1").join("file2.txt"),
        test_path.join("dir1").join("file3.txt"),
        test_path.join("dir2").join("file4.txt"),
        test_path.join("dir3").join("file5.txt"),
        test_path.join("file1.txt"),
        test_path.join("file9.txt"),
        test_path.join("longfile.txt"),
    ];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(6, test_path.join("file8.txt"));
    }
    test(args, &expected_file_names);
}

#[test]
fn exclude_directory() {
    let test_path = Path::new("test");
    let args = "pattern -R test --exclude-dir dir4";
    let mut expected_file_names = vec![
        test_path.join("dir1").join("file2.txt"),
        test_path.join("dir1").join("file3.txt"),
        test_path.join("dir2").join("file4.txt"),
        test_path.join("dir3").join("file5.txt"),
        test_path.join("file1.txt"),
        test_path.join("file9.txt"),
        test_path.join("longfile.txt"),
    ];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(5, test_path.join("file8.txt"));
    }
    test(args, &expected_file_names);
}

fn test(args: &str, expected_file_names: &Vec<PathBuf>) {
    let args = args
        .split_whitespace()
        .map(|arg| arg.to_string())
        .collect::<Vec<String>>();

    unsafe { env::set_var("NED_DEFAULTS", "") };

    let options_with_defaults = OptionsWithDefaults::new(make_opts(), &args).unwrap();
    let parameters = get_parameters(&options_with_defaults).unwrap();

    let file_names = Files::new(&parameters, &parameters.globs[0]);

    let mut file_names = file_names
        .map(|path| path.as_path().to_string_lossy().to_string())
        .collect::<Vec<String>>();
    file_names.sort();

    let expected_file_names = expected_file_names.iter()
        .map(|path| path.to_str().unwrap().to_string())
        .collect::<Vec<String>>();

    assert_eq!(&file_names, &expected_file_names);
}
