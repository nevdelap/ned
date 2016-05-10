use getopts;
use glob;
use regex;
use std::string;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum NedError {
    GetOpts(getopts::Fail),
    GlobPattern(glob::PatternError),
    Io(io::Error),
    Regex(regex::Error),
    FromUtf8(string::FromUtf8Error),
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

impl From<string::FromUtf8Error> for NedError {
    fn from(err: string::FromUtf8Error) -> NedError {
        NedError::FromUtf8(err)
    }
}

impl fmt::Display for NedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NedError::GetOpts(ref err) => write!(f, "{}", err),
            NedError::GlobPattern(ref err) => write!(f, "{}", err),
            NedError::Io(ref err) => write!(f, "{}", err),
            NedError::Regex(ref err) => write!(f, "{}", err),
            NedError::FromUtf8(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for NedError {
    fn description(&self) -> &str {
        match *self {
            NedError::GetOpts(ref err) => err.description(),
            NedError::GlobPattern(ref err) => err.description(),
            NedError::Io(ref err) => err.description(),
            NedError::Regex(ref err) => err.description(),
            NedError::FromUtf8(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            NedError::GetOpts(ref err) => Some(err),
            NedError::GlobPattern(ref err) => Some(err),
            NedError::Io(ref err) => Some(err),
            NedError::Regex(ref err) => Some(err),
            NedError::FromUtf8(ref err) => Some(err),
        }
    }
}

pub type NedResult<T> = Result<T, NedError>;
