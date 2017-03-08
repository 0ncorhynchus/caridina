use std::result;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    SyntaxError,
    InvalidArgument,
    NotCallable,
    UnknownOperator(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::SyntaxError => {
                write!(f, "Syntax error")
            },
            Error::InvalidArgument => {
                write!(f, "Invalid argument")
            },
            Error::NotCallable => {
                write!(f, "Not callable")
            },
            Error::UnknownOperator(ref op) => {
                write!(f, "The operator '{}' is unknown", op)
            }
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
