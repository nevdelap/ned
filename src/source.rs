use std::fs::File;
#[cfg(test)]
use std::io::Cursor;
use std::io::Read;

pub enum Source {
    Stdin(Box<Read>),
    File(Box<File>),
    #[cfg(test)]
    Cursor(Box<Cursor<Vec<u8>>>),
}
