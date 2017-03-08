use error::*;
use types::*;

fn add(args: &[Datum]) -> ProcedureResult<Object> {
    let mut sum = 0.0;
    for datum in args {
        match *datum {
            Datum::Number(x) => sum += x,
            _ => return Err(ProcedureCallError::InvalidArgument)
        }
    }
    Ok(Object::Number(sum))
}

pub fn call(operator: &Datum, args: &[Datum]) -> ProcedureResult<Object> {
    match *operator {
        Datum::Symbol(ref op) => {
            match op.as_str() {
                "+" => add(args),
                _ => Err(ProcedureCallError::UnknownOperator(
                        op.to_string()))
            }
        },
        _ => Err(ProcedureCallError::NotCallable)
    }
}

pub fn eval(expr: &Datum) -> ProcedureResult<Object> {
    match *expr {
        Datum::Boolean(flg)     => Ok(Object::Boolean(flg)),
        Datum::Number(x)        => Ok(Object::Number(x)),
        Datum::Character(c)     => Ok(Object::Char(c)),
        Datum::String(ref s)    => Ok(Object::String(s.clone())),
        Datum::Symbol(ref s)    => Ok(Object::Symbol(s.clone())),
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

