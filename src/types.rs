use std::collections::HashMap;

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

    /*
     * Compound Datum
     */
    List(Vec<Datum>),
    Vector(Vec<Datum>),
}
