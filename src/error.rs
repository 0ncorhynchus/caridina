use std::result;

#[derive(Debug)]
pub enum ProcedureCallError {
    InvalidArgument,
    NotCallable,
    UnknownOperator(String),
}

#[derive(Debug)]
pub enum Error {
    ProcedureCall(ProcedureCallError)
}
