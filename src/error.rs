use std::result;

#[derive(Debug)]
pub enum Error {
    InvalidArgument,
    NotCallable,
    UnknownOperator(String),
}

pub type Result<T> = result::Result<T, Error>;
