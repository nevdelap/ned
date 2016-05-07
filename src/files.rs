use ansi_term::Colour::Red;
use getopts::{Matches, Options, ParsingStyle};
use glob::Pattern;
use regex::Regex;
use parameters::Parameters;
use source::Source;
use std::fs::OpenOptions;
#[cfg(test)]
use std::io::Cursor;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::iter::Iterator;
use std::path::PathBuf;
use std::string::String;
use std::{env, path, process};
use walkdir::{self, WalkDir, WalkDirIterator};

pub struct Files {
    parameters: Parameters,
    walkdir: Box<walkdir::Iter>,
}

impl Files {
    pub fn new(parameters: &Parameters, glob: &String) -> Files {
        let mut walkdir = WalkDir::new(&glob).follow_links(parameters.follow);
        if !parameters.recursive {
            walkdir = walkdir.max_depth(1);
        }
        Files {
            parameters: parameters.clone(),
            walkdir: Box::new(walkdir.into_iter()),
        }
    }
}

impl Iterator for Files {
    type Item = Box<PathBuf>;

    fn next(&mut self) -> Option<Box<PathBuf>> {
        loop {
            match self.walkdir.next() {
                Some(entry) => {
                    match entry {
                        Ok(entry) => {
                            if let Some(file_name) = entry.path().file_name() {
                                if let Some(file_name) = file_name.to_str() {
                                    let file_type = entry.file_type();
                                    let excluded_dir = entry.file_type().is_dir() &&
                                                       self.parameters
                                                           .exclude_dirs
                                                           .iter()
                                                           .any(|pattern| {
                                                               pattern.matches(file_name)
                                                           });
                                    if excluded_dir {
                                        self.walkdir.skip_current_dir();
                                    }
                                    let included_file = file_type.is_file() &&
                                                        (self.parameters.includes.len() == 0 ||
                                                         self.parameters
                                                             .includes
                                                             .iter()
                                                             .any(|pattern| {
                                                                 pattern.matches(file_name)
                                                             }));
                                    let excluded_file = file_type.is_file() &&
                                                        self.parameters
                                                            .excludes
                                                            .iter()
                                                            .any(|pattern| {
                                                                pattern.matches(file_name)
                                                            });
                                    let all = self.parameters.all;
                                    let hidden = file_name.starts_with(".");
                                    if included_file && !excluded_file && (all || !hidden) {
                                        return Some(Box::new(entry.path().to_path_buf()));
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            panic!("Ouch! {}", err);
                            // err to stdout, call self again
                            // continue;
                        }
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }
}
