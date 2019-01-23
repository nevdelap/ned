//
// ned, https://github.com/nevdelap/ned, tests/files.rs
//
// Copyright 2016-2019 Nev Delap (nevdelap at gmail)
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
use files::Files;
use options_with_defaults::OptionsWithDefaults;
use opts::make_opts;
use parameters::get_parameters;
use std::env;

#[test]
fn no_recursion() {
    let args = "pattern test";
    let mut expected_file_names = vec!["file1.txt", "file9.txt", "longfile.txt"];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(1, "file8.txt");
    }

    test(&args, &expected_file_names);
}

#[test]
fn no_recursion_all() {
    let args = "pattern --all test";
    let mut expected_file_names = vec![".hidden_file1", "file1.txt", "file9.txt", "longfile.txt"];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(2, "file8.txt");
    }

    test(&args, &expected_file_names);
}

#[test]
fn no_recursion_follow() {
    let args = "pattern --follow test";
    let expected_file_names = ["file1.txt", "file8.txt", "file9.txt", "longfile.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn no_recursion_follow_all() {
    let args = "pattern --follow --all test";
    let expected_file_names = [
        ".hidden_file1",
        "file1.txt",
        "file8.txt",
        "file9.txt",
        "longfile.txt",
    ];

    test(&args, &expected_file_names);
}

#[test]
fn recursion() {
    let args = "pattern --recursive test";
    let mut expected_file_names = vec![
        "file1.txt",
        "file2.txt",
        "file3.txt",
        "file4.txt",
        "file5.txt",
        "file6.txt",
        "file7.txt",
        "file9.txt",
        "longfile.txt",
    ];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(7, "file8.txt");
    }

    test(&args, &expected_file_names);
}

#[test]
fn recursion_all() {
    let args = "pattern --recursive --all test";
    let mut expected_file_names = vec![
        ".hidden_file1",
        ".hidden_file2",
        "file1.txt",
        "file2.txt",
        "file3.txt",
        "file4.txt",
        "file5.txt",
        "file6.txt",
        "file7.txt",
        "file9.txt",
        "longfile.txt",
    ];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(9, "file8.txt");
    }

    test(&args, &expected_file_names);
}

#[test]
fn recursion_follow() {
    let args = "pattern --follow test";
    let expected_file_names = ["file1.txt", "file8.txt", "file9.txt", "longfile.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn recursion_follow_all() {
    let args = "pattern --recursive --follow --all test";
    let expected_file_names = [
        ".hidden_file1",
        ".hidden_file2",
        "file1.txt",
        "file2.txt",
        "file3.txt",
        "file4.txt",
        "file5.txt",
        "file6.txt",
        "file7.txt",
        "file8.txt",
        "file9.txt",
        "longfile.txt",
    ];

    test(&args, &expected_file_names);
}

#[test]
fn include_files() {
    let args = "pattern -R test --include file7*";
    let expected_file_names = ["file7.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn exclude_files() {
    let args = "pattern -R test --exclude file7*";
    let mut expected_file_names = vec![
        "file1.txt",
        "file2.txt",
        "file3.txt",
        "file4.txt",
        "file5.txt",
        "file6.txt",
        "file9.txt",
        "longfile.txt",
    ];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(6, "file8.txt");
    }

    test(&args, &expected_file_names);
}

#[test]
fn exclude_directory() {
    let args = "pattern -R test --exclude-dir dir4";
    let mut expected_file_names = vec![
        "file1.txt",
        "file2.txt",
        "file3.txt",
        "file4.txt",
        "file5.txt",
        "file9.txt",
        "longfile.txt",
    ];
    if cfg!(windows) {
        // Windows presents the symlink as a regular file.
        expected_file_names.insert(5, "file8.txt");
    }

    test(&args, &expected_file_names);
}

fn test(args: &str, expected_file_names: &[&str]) {
    let args = args
        .split_whitespace()
        .map(|arg| arg.to_string())
        .collect::<Vec<String>>();
    env::set_var("NED_DEFAULTS", "");
    let options_with_defaults = OptionsWithDefaults::new(make_opts(), &args).unwrap();
    let parameters = get_parameters(&options_with_defaults).unwrap();
    let paths = Files::new(&parameters, &parameters.globs[0]);
    let mut file_names = paths
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    file_names.sort();

    assert_eq!(
        &file_names,
        &expected_file_names
            .iter()
            .map(|file_name| file_name.to_string())
            .collect::<Vec<String>>()
    );
}
