use getopts;
use glob;
use opts::PROGRAM;
use regex;
use std::error;
use std::fmt;
use std::io::{self, Write};
use std::path;
use std::string;

#[derive(Debug)]
pub struct StringError {
    pub err: String,
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl error::Error for StringError {
    fn description(&self) -> &str {
        self.err.as_str()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
pub enum NedError {
    FromUtf8(string::FromUtf8Error),
    GetOpts(getopts::Fail),
    GlobPattern(glob::PatternError),
    Io(io::Error),
    ParameterError(StringError),
    Regex(regex::Error),
}

impl From<string::FromUtf8Error> for NedError {
    fn from(err: string::FromUtf8Error) -> NedError {
        NedError::FromUtf8(err)
    }
}

impl From<getopts::Fail> for NedError {
    fn from(err: getopts::Fail) -> NedError {
        NedError::GetOpts(err)
    }
}

impl From<glob::PatternError> for NedError {
    fn from(err: glob::PatternError) -> NedError {
        NedError::GlobPattern(err)
    }
}

impl From<io::Error> for NedError {
    fn from(err: io::Error) -> NedError {
        NedError::Io(err)
    }
}

impl From<regex::Error> for NedError {
    fn from(err: regex::Error) -> NedError {
        NedError::Regex(err)
    }
}

impl From<String> for NedError {
    fn from(err: String) -> NedError {
        NedError::ParameterError(StringError { err: err })
    }
}

impl fmt::Display for NedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NedError::FromUtf8(ref err) => write!(f, "{}", err),
            NedError::GetOpts(ref err) => write!(f, "{}", err),
            NedError::GlobPattern(ref err) => write!(f, "{}", err),
            NedError::Io(ref err) => write!(f, "{}", err),
            NedError::ParameterError(ref err) => write!(f, "{}", err),
            NedError::Regex(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for NedError {
    fn description(&self) -> &str {
        match *self {
            NedError::FromUtf8(ref err) => err.description(),
            NedError::GetOpts(ref err) => err.description(),
            NedError::GlobPattern(ref err) => err.description(),
            NedError::Io(ref err) => err.description(),
            NedError::ParameterError(ref err) => err.description(),
            NedError::Regex(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            NedError::FromUtf8(ref err) => Some(err),
            NedError::GetOpts(ref err) => Some(err),
            NedError::GlobPattern(ref err) => Some(err),
            NedError::Io(ref err) => Some(err),
            NedError::ParameterError(ref err) => Some(err),
            NedError::Regex(ref err) => Some(err),
        }
    }
}

pub type NedResult<T> = Result<T, NedError>;

pub fn stderr_write_err(err: &error::Error) {
    io::stderr()
        .write(&format!("{}: {}\n", PROGRAM, err.to_string()).into_bytes())
        .expect("Can't write to stderr!");
}

pub fn stderr_write_file_err(path_buf: &path::PathBuf, err: &error::Error) {
    io::stderr()
        .write(&format!("{}: {} {}\n",
                        PROGRAM,
                        path_buf.as_path().to_string_lossy(),
                        err.to_string())
                    .into_bytes())
        .expect("Can't write to stderr!");
}
