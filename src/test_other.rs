use std::path::Path;
use {get_files, make_opts};

#[test]
fn no_recursion() {

    let options = "/home/nev/dev/ned/test";
    let expected_file_names = ["file1.txt"];

    test(&options, &expected_file_names);
}

#[test]
fn recursion() {

    let options = "-R /home/nev/dev/ned/test";
    let expected_file_names = ["file6.txt",
                               "file7.txt",
                               "file3.txt",
                               "file2.txt",
                               "file5.txt",
                               "file4.txt",
                               "file1.txt"];

    test(&options, &expected_file_names);
}

fn get_path_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string().clone()
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
