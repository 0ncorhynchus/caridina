use datum::Datum;

pub fn read(input: &str) -> Datum {
    let chars = input.trim_left().chars();

    let mut repr = String::new();
    for c in chars {
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
    fn test_read() {
        assert_eq!(Datum::Boolean(true),  read("#t"));
        assert_eq!(Datum::Boolean(false), read("#f"));
        assert_eq!(Datum::Number(404f64), read("404"));
        assert_eq!(Datum::Symbol("var".to_string()), read("var"));
    }
}
