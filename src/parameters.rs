extern crate regex;

use glob::Pattern;
use regex::Regex;
use std::string::String;

#[derive(Clone)]
pub struct Parameters {
    pub all: bool,
    pub colors: bool,
    pub exclude_dirs: Vec<Pattern>,
    pub excludes: Vec<Pattern>,
    pub follow: bool,
    pub globs: Vec<String>,
    pub group: Option<String>,
    pub help: bool,
    pub includes: Vec<Pattern>,
    pub line_oriented: bool,
    pub no_match: bool,
    pub only_matches: bool,
    pub quiet: bool,
    pub re: Option<Regex>,
    pub recursive: bool,
    pub replace: Option<String>,
    pub stdout: bool,
    pub version: bool,
}
