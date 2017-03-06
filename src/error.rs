use std::result;
use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    EOF
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}


pub type Result<T> = result::Result<T, Error>;
