use std::io::{Read, BufRead, BufReader};
use error::*;
use datum::Datum;

pub struct CharReader<R> {
    handle: BufReader<R>,
    line: String
}

impl<R: Read> CharReader<R> {
    pub fn new(reader: R) -> Self {
        CharReader {
            handle: BufReader::new(reader),
            line: String::new(),
        }
    }

    pub fn next_char(&mut self) -> Result<char> {
        if self.line.is_empty() {
            self.handle.read_line(&mut self.line)?;
            if self.line.is_empty() {
                return Err(Error::EOF);
            }
        }
        Ok(self.line.remove(0))
    }
}

pub fn read<R: Read>(input: &mut CharReader<R>) -> Datum {
    let mut repr = String::new();

    while let Some(c) = input.next_char().ok() {
        if c != ' ' {
            repr.push(c);
            break;
        }
    }

    while let Some(c) = input.next_char().ok() {
        if c == ' ' {
            break;
        }
        repr.push(c);
    }

    if repr == "#t" {
        Datum::Boolean(true)
    } else if repr == "#f" {
        Datum::Boolean(false)
    } else {
        let num = repr.parse::<f64>();
        if num.is_ok() {
            Datum::Number(num.unwrap())
        } else {
            Datum::Symbol(repr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let source: &[u8] = b"spam\nham";
        let mut input = CharReader::new(source);
        assert_eq!('s', input.next_char().unwrap());
        assert_eq!('p', input.next_char().unwrap());
        assert_eq!('a', input.next_char().unwrap());
        assert_eq!('m', input.next_char().unwrap());
        assert_eq!('\n', input.next_char().unwrap());
        assert_eq!('h', input.next_char().unwrap());
        assert_eq!('a', input.next_char().unwrap());
        assert_eq!('m', input.next_char().unwrap());
        assert!(input.next_char().is_err());
    }

    #[test]
    fn test_read() {
        let source: &[u8] = b" #t #f 404 var";
        let mut input = CharReader::new(source);
        assert_eq!(Datum::Boolean(true),  read(&mut input));
        assert_eq!(Datum::Boolean(false), read(&mut input));
        assert_eq!(Datum::Number(404f64), read(&mut input));
        assert_eq!(Datum::Symbol("var".to_string()), read(&mut input));
    }
}
