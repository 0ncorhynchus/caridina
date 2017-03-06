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

    Empty
}
