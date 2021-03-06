use std::str::Chars;
use types::Datum;

const DELIMITERS: &'static [char]
    = &[' ', '\n', '\t', '(', ')', '"', ';'];

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

fn convert_simple_token(token: &str) -> Datum {
    match token {
        "#t" => Datum::Boolean(true),
        "#f" => Datum::Boolean(false),
        _ => {
            let num = token.parse::<f64>();
            if num.is_ok() {
                Datum::Number(num.unwrap())
            } else {
                Datum::Symbol(token.to_string())
            }
        }
    }
}

fn read_string_datum(chars: &mut Chars) -> String {
    let mut string = String::new();
    loop {
        let (token, delimiter) = next_token(chars);
        string = (string + &token).to_string();
        if delimiter == '"' {
            break;
        } else {
            string.push(delimiter);
        }
    }
    string
}

fn read_list_datum(chars: &mut Chars) -> Vec<Datum> {
    let mut vec = vec![];
    loop {
        let (token, delimiter) = next_token(chars);
        if !token.is_empty() {
            vec.push(convert_simple_token(&token));
        }
        match delimiter {
            ')' => break,
            '(' => vec.push(Datum::List(read_list_datum(chars))),
            '"' => vec.push(Datum::String(read_string_datum(chars))),
            _ => {}
        }
    }
    vec
}

pub fn read(input: &str) -> Option<Datum> {
    let mut chars = input.trim_left().chars();

    let (token, delimiter) = next_token(&mut chars);

    if !token.is_empty() {
        return Some(convert_simple_token(&token));
    }

    match delimiter {
        '"' => Some(Datum::String(read_string_datum(&mut chars))),
        '(' => Some(Datum::List(read_list_datum(&mut chars))),
        _   => None
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
        assert_eq!(Some(Datum::Boolean(true)),  read("#t"));
        assert_eq!(Some(Datum::Boolean(false)), read("#f"));
        assert_eq!(Some(Datum::Number(404f64)), read("404"));
        assert_eq!(Some(Datum::Symbol("var".to_string())),
                   read("var"));
        assert_eq!(Some(Datum::String("spam ham".to_string())),
                   read("\"spam ham\""));

        assert_eq!(Some(Datum::List(vec![Datum::Boolean(true),
                                         Datum::Boolean(false)])),
                   read("(#t #f)"));
        assert_eq!(Some(Datum::List(vec![
                       Datum::Boolean(true),
                       Datum::List(vec![
                           Datum::Symbol("a".to_string()),
                           Datum::Number(1f64)])])),
                   read("(#t (a 1))"));
        assert_eq!(Some(Datum::List(vec![
                       Datum::Boolean(true),
                       Datum::List(vec![
                           Datum::Symbol("a".to_string()),
                           Datum::Number(1f64)])])),
                   read("(#t(a 1))"));
    }
}
