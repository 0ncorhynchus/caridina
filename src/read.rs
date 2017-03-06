use std::str::Chars;
use datum::Datum;

const DELIMITERS: &'static [char] = &[' ', '(', ')', '"', ';'];

fn is_delimiter(c: char) -> bool {
    DELIMITERS.into_iter().any(|&delimiter| delimiter == c)
}

pub fn next_token(chars: &mut Chars) -> (String, char) {
    let mut token = String::new();
    for c in chars {
        if is_delimiter(c) {
            return (token, c);
        } else {
            token.push(c);
        }
    }
    (token, ' ')
}

pub fn read(input: &str) -> Datum {
    let mut chars = input.trim_left().chars();

    let (repr, _) = next_token(&mut chars);

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
    fn test_next_token() {
        let repr = "#t #f (var 404)";
        let mut chars = repr.chars();
        assert_eq!(("#t".to_string(), ' '),  next_token(&mut chars));
        assert_eq!(("#f".to_string(), ' '),  next_token(&mut chars));
        assert_eq!(("".to_string(), '('),    next_token(&mut chars));
        assert_eq!(("var".to_string(), ' '), next_token(&mut chars));
        assert_eq!(("404".to_string(), ')'), next_token(&mut chars));
    }

    #[test]
    fn test_read() {
        assert_eq!(Datum::Boolean(true),  read("#t"));
        assert_eq!(Datum::Boolean(false), read("#f"));
        assert_eq!(Datum::Number(404f64), read("404"));
        assert_eq!(Datum::Symbol("var".to_string()), read("var"));
    }
}
