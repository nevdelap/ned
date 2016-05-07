extern crate ansi_term;
extern crate getopts;
extern crate glob;
extern crate regex;
extern crate walkdir;

use ansi_term::Colour::Red;
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

pub enum Source {
    Stdin(Box<Read>),
    File(Box<File>),
    #[cfg(test)]
    Cursor(Box<Cursor<Vec<u8>>>),
}
