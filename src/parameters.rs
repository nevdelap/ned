// TODO:
// exit_codes, put back in tests.
// make it to stdin if there are globs.
// remove the Option from walkdirs.
// stderr in Files.
// write a test for files: check behaviour if Files is passed zero globs.
// move Source, Files, Parameters, opts funcs, constants to their own files.

extern crate ansi_term;
extern crate getopts;
extern crate glob;
extern crate regex;
extern crate walkdir;

use getopts::{Matches, Options, ParsingStyle};
use glob::Pattern;
use regex::Regex;
use std::fs::{File, OpenOptions};
#[cfg(test)]
use std::io::Cursor;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::iter::Iterator;
use std::path::PathBuf;
use std::string::String;
use std::{env, path, process};
use walkdir::{WalkDir, WalkDirIterator};

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
