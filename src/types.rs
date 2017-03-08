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

pub type Procedure = Fn(&[Datum]) -> Datum;

pub struct Environment {
    procedure_map: HashMap<String, Box<Procedure>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            procedure_map: HashMap::new(),
        }
    }

    pub fn register_procedure(&mut self, name: &str, procedure: Box<Procedure>) {
        self.procedure_map.insert(name.to_string(), procedure);
    }
}
