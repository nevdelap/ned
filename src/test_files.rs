/// Test file related functionality - recursion, inclusion, exclusion, symlinks, etc.

use make_opts;
use parameters::get_parameters;
use files::Files;

#[test]
fn no_recursion() {

    let args = "pattern test";
    let expected_file_names = ["file1.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn no_recursion_all() {

    let args = "pattern --all test";
    let expected_file_names = [".hidden_file1", "file1.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn no_recursion_follow() {

    let args = "pattern --follow test";
    let expected_file_names = ["file8.txt", "file1.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn recursion() {

    let args = "pattern --recursive test";
    let expected_file_names = ["file6.txt",
                               "file7.txt",
                               "file3.txt",
                               "file2.txt",
                               "file5.txt",
                               "file4.txt",
                               "file1.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn recursion_all() {

    let args = "pattern --recursive --all test";
    let expected_file_names = ["file6.txt",
                               "file7.txt",
                               "file3.txt",
                               "file2.txt",
                               "file5.txt",
                               ".hidden_file2",
                               ".hidden_file1",
                               "file4.txt",
                               "file1.txt"];

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
    let expected_file_names = ["file6.txt",
                               "file3.txt",
                               "file2.txt",
                               "file5.txt",
                               "file4.txt",
                               "file1.txt"];

    test(&args, &expected_file_names);
}

#[test]
fn exclude_directory() {

    let args = "pattern -R test --exclude-dir dir4";
    let expected_file_names = ["file3.txt", "file2.txt", "file5.txt", "file4.txt", "file1.txt"];

    test(&args, &expected_file_names);
}

fn test(args: &str, expected_file_names: &[&str]) {
    let opts = make_opts();
    let args = args.split_whitespace().map(|arg| arg.to_string()).collect::<Vec<String>>();
    let parameters = get_parameters(&opts, &args).unwrap();
    let paths = Files::new(&parameters, &parameters.globs[0]);
    let file_names = paths.map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
                          .collect::<Vec<String>>();
    assert_eq!(&file_names,
               &expected_file_names.iter()
                                   .map(|file_name| file_name.to_string())
                                   .collect::<Vec<String>>());
}
