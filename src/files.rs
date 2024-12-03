//
// ned, https://github.com/nevdelap/ned, files.rs
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

use crate::ned_error::stderr_write_err;
use crate::parameters::Parameters;
use std::iter::IntoIterator;
use std::path::Component;
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

    /// Normalize relative paths (remove ./ and normalize ../) without
    /// converting symlinks to the path they point to.
    fn normalize_relative_paths(input_path: PathBuf) -> std::io::Result<PathBuf> {
        let mut components: Vec<Component> = Vec::new();
        for component in input_path.components() {
            match component {
                Component::CurDir => {
                    // Ignore `./`
                    continue;
                }
                Component::ParentDir => {
                    // Resolve `../` by popping the last normal component
                    if let Some(last_component) = components.last() {
                        if *last_component != Component::ParentDir {
                            components.pop();
                            continue;
                        }
                    }
                    // If there's no preceding component to pop, keep `..`
                    components.push(component);
                }
                _ => {
                    // Keep normal components
                    components.push(component);
                }
            }
        }
        let mut output_path = PathBuf::new();
        for component in components {
            output_path.push(component.as_os_str());
        }
        Ok(output_path)
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
                                let all = self.parameters.all;
                                let hidden = file_name.starts_with('.');
                                let file_type = entry.file_type();
                                if file_type.is_dir() {
                                    let excluded_dir = (!all && hidden)
                                        || self
                                            .parameters
                                            .exclude_dirs
                                            .iter()
                                            .any(|pattern| pattern.matches(file_name));
                                    if excluded_dir {
                                        self.walkdir.skip_current_dir();
                                    }
                                    continue;
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
                                if included_file && !excluded_file && (all || !hidden) {
                                    return Some(Box::new(
                                        Self::normalize_relative_paths(entry.path().to_path_buf())
                                            .ok()?,
                                    ));
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
