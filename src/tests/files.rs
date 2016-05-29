/// Test file related functionality - recursion, inclusion, exclusion, symlinks, etc.

use opts::make_opts;
use parameters::get_parameters;
use files::Files;

#[test]
fn no_recursion() {

    let args = "pattern test";
    let expected_file_names = ["file1.txt", "longfile.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn no_recursion_all() {

    let args = "pattern --all test";
    let expected_file_names = [".hidden_file1", "file1.txt", "longfile.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn no_recursion_follow() {

    let args = "pattern --follow test";
    let expected_file_names = ["file1.txt", "file8.txt", "longfile.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn no_recursion_follow_all() {

    let args = "pattern --follow --all test";
    let expected_file_names = [".hidden_file1", "file1.txt", "file8.txt", "longfile.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn recursion() {

    let args = "pattern --recursive test";
    let expected_file_names = ["file1.txt",
                               "file2.txt",
                               "file3.txt",
                               "file4.txt",
                               "file5.txt",
                               "file6.txt",
                               "file7.txt",
                               "longfile.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn recursion_all() {

    let args = "pattern --recursive --all test";
    let expected_file_names = [".hidden_file1",
                               ".hidden_file2",
                               "file1.txt",
                               "file2.txt",
                               "file3.txt",
                               "file4.txt",
                               "file5.txt",
                               "file6.txt",
                               "file7.txt",
                               "longfile.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn recursion_follow() {

    let args = "pattern --follow test";
    let expected_file_names = ["file1.txt", "file8.txt", "longfile.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn recursion_follow_all() {

    let args = "pattern --recursive --follow --all test";
    let expected_file_names = [".hidden_file1",
                               ".hidden_file2",
                               "file1.txt",
                               "file2.txt",
                               "file3.txt",
                               "file4.txt",
                               "file5.txt",
                               "file6.txt",
                               "file7.txt",
                               "file8.txt",
                               "longfile.txt"];

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
    let expected_file_names = ["file1.txt",
                               "file2.txt",
                               "file3.txt",
                               "file4.txt",
                               "file5.txt",
                               "file6.txt",
                               "longfile.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn exclude_directory() {

    let args = "pattern -R test --exclude-dir dir4";
    let expected_file_names =
        ["file1.txt", "file2.txt", "file3.txt", "file4.txt", "file5.txt", "longfile.txt"];

    test(&args, &expected_file_names);
}

fn test(args: &str, expected_file_names: &[&str]) {
    let opts = make_opts();
    let args = args.split_whitespace().map(|arg| arg.to_string()).collect::<Vec<String>>();
    let parameters = get_parameters(&opts, &args).unwrap();
    let paths = Files::new(&parameters, &parameters.globs[0]);
    let mut filenames = paths.map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    filenames.sort();

    assert_eq!(&filenames,
               &expected_file_names.iter()
                   .map(|filename| filename.to_string())
                   .collect::<Vec<String>>());
}
