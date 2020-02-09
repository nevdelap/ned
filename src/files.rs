//
// ned, https://github.com/nevdelap/ned, files.rs
//
// Copyright 2016-2020 Nev Delap (nevdelap at gmail)
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

use crate::ned_error::stderr_write_err;
use crate::parameters::Parameters;
use std::iter::IntoIterator;
use std::path::PathBuf;
use walkdir::{IntoIter, WalkDir};

pub struct Files {
    parameters: Parameters,
    walkdir: Box<IntoIter>,
}

impl Files {
    pub fn new(parameters: &Parameters, glob: &str) -> Files {
        let mut walkdir = WalkDir::new(&glob)
            .follow_links(parameters.follow)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()));
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
                Some(entry) => match entry {
                    Ok(entry) => {
                        if let Some(file_name) = entry.path().file_name() {
                            if let Some(file_name) = file_name.to_str() {
                                let file_type = entry.file_type();
                                let excluded_dir = file_type.is_dir()
                                    && self
                                        .parameters
                                        .exclude_dirs
                                        .iter()
                                        .any(|pattern| pattern.matches(file_name));
                                if excluded_dir {
                                    self.walkdir.skip_current_dir();
                                }
                                let included_file = file_type.is_file()
                                    && (self.parameters.includes.is_empty()
                                        || self
                                            .parameters
                                            .includes
                                            .iter()
                                            .any(|pattern| pattern.matches(file_name)));
                                let excluded_file = file_type.is_file()
                                    && self
                                        .parameters
                                        .excludes
                                        .iter()
                                        .any(|pattern| pattern.matches(file_name));
                                let all = self.parameters.all;
                                let hidden = file_name.starts_with('.');
                                if included_file && !excluded_file && (all || !hidden) {
                                    return Some(Box::new(entry.path().to_path_buf()));
                                }
                            }
                        }
                    }
                    Err(err) => {
                        stderr_write_err(&err);
                        continue;
                    }
                },
                None => {
                    return None;
                }
            }
        }
    }
}
