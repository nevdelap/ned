use std::path::Path;
use {get_files, make_opts};

#[test]
fn no_recursion() {

    let options = "test";
    let expected_file_names = ["file1.txt"];

    test(&options, &expected_file_names);
}

#[test]
fn no_recursion_all() {

    let options = "test --all";
    let expected_file_names = [".hidden_file1", "file1.txt"];

    test(&options, &expected_file_names);
}

#[test]
fn no_recursion_follow() {

    let options = "test --follow";
    let expected_file_names = ["file7.txt", "file1.txt"];

    test(&options, &expected_file_names);
}

#[test]
fn recursion() {

    let options = "test --recursive";
    let expected_file_names = ["file6.txt",
                               "file7.txt",
                               "file3.txt",
                               "file2.txt",
                               "file5.txt",
                               "file4.txt",
                               "file1.txt"];

    test(&options, &expected_file_names);
}

#[test]
fn recursion_all() {

    let options = "test --recursive --all";
    let expected_file_names = ["file6.txt",
                               "file7.txt",
                               "file3.txt",
                               "file2.txt",
                               "file5.txt",
                               "hidden_file2",
                               ".hidden_file1",
                               "file4.txt",
                               "file1.txt"];

    test(&options, &expected_file_names);
}

#[test]
fn include_files() {

    let options = "-R test --include file7*";
    let expected_file_names = ["file7.txt"];

    test(&options, &expected_file_names);
}

#[test]
fn exclude_files() {

    let options = "-R test --exclude file7*";
    let expected_file_names = ["file6.txt",
                               "file3.txt",
                               "file2.txt",
                               "file5.txt",
                               "file4.txt",
                               "file1.txt"];

    test(&options, &expected_file_names);
}

#[test]
fn exclude_directory() {

    let options = "-R test --exclude-dir dir4";
    let expected_file_names = ["file3.txt", "file2.txt", "file5.txt", "file4.txt", "file1.txt"];

    test(&options, &expected_file_names);
}

fn get_path_name(path: &Path) -> Result<String, String> {
    Ok(path.file_name().unwrap().to_str().unwrap().to_string().clone())
}

fn test(options: &str, expected_file_names: &[&str]) {
    let opts = make_opts();
    let options: Vec<&str> = options.split_whitespace().collect();
    let matches = opts.parse(&options).unwrap();
    let file_names = matches.free.iter().collect::<Vec<_>>();
    let files = get_files(&matches, &file_names, get_path_name).unwrap();
    assert_eq!(&files,
               &expected_file_names.iter()
                                   .map(|file_name| file_name.to_string())
                                   .collect::<Vec<String>>());
}
