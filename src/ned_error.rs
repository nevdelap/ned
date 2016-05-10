use std::io;

enum NedError {
    Io(io::Error),
}

impl From<io::Error> for NedError {
    fn from(err: io::Error) -> NedError {
        NedError::Io(err)
    }
}

type NedResult<T> = Result<T, NedError>;
