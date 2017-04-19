use error::*;
use types::*;
use ops;

fn call(operator: &Datum, args: &[Datum]) -> Result<Object> {
    match *operator {
        Datum::Symbol(ref op) => {
            match op.as_str() {
                "+"  => ops::add(args),
                "*"  => ops::mul(args),
                "if" => ops::if_proc(args),
                _ => Err(Error::UnknownOperator(op.to_string()))
            }
        },
        _ => Err(Error::NotCallable)
    }
}

pub fn eval(expr: &Datum) -> Result<Object> {
    match *expr {
        Datum::Boolean(flg)     => Ok(Object::Boolean(flg)),
        Datum::Number(x)        => Ok(Object::Number(x)),
        Datum::Character(c)     => Ok(Object::Char(c)),
        Datum::String(ref s)    => Ok(Object::String(s.clone())),
        Datum::Symbol(ref s)    => Ok(Object::Symbol(s.clone())),
        Datum::Empty            => Ok(Object::Empty),
        Datum::List(ref list)   => call(&list[0], &list[1..]),
        Datum::Vector(ref list) => {
            let results: Vec<_> = list.iter()
                                      .map(|datum| eval(datum))
                                      .collect();
            let mut inner = vec!{};
            for x in results {
                inner.push(x?);
            }
            Ok(Object::Vector(inner))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call() {
        let ops = Datum::Symbol("+".to_string());
        let args = vec![Datum::Number(1.0), Datum::Number(2.0)];
        let result = call(&ops, &args);

        assert!(result.is_ok());
        assert_eq!(Object::Number(3.0), result.unwrap());
    }

    #[test]
    fn test_eval() {
        let expr = Datum::List(
            vec![
                Datum::Symbol("+".to_string()),
                Datum::Number(1.0),
                Datum::Number(2.0)
            ]);
        let result = eval(&expr);

        assert!(result.is_ok());
        assert_eq!(Object::Number(3.0), result.unwrap());
    }
}

