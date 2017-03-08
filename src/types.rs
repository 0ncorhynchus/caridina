use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Object {
    Boolean(bool),
    Pair,
    Symbol(String),
    Number(f64),
    Char(char),
    String(String),
    Vector(Vec<Object>),
    Port,
    Procedure,
    Empty,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Boolean(flg)  => write!(f, "{}", flg),
            Object::Symbol(ref s) => write!(f, "'{}", s),
            Object::Number(x)     => write!(f, "{}", x),
            Object::Char(c)       => write!(f, "\\{}", c),
            Object::String(ref s) => write!(f, "\"{}\"", s),
            Object::Empty         => write!(f, "()"),
            _ => write!(f, "Cannot display")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Datum {
    /*
     * Simple Datum
     */
    Boolean(bool),
    Number(f64),
    Character(char),
    String(String),
    Symbol(String),
    Empty,

    /*
     * Compound Datum
     */
    List(Vec<Datum>),
    Vector(Vec<Datum>),
}
