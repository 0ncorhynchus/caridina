use error::*;
use types::Datum;

fn add(args: &[Datum]) -> ProcedureResult<Datum> {
    let mut sum = 0.0;
    for datum in args {
        match *datum {
            Datum::Number(x) => sum += x,
            _ => return Err(ProcedureCallError::InvalidArgument)
        }
    }
    Ok(Datum::Number(sum))
}

pub fn call(operator: &Datum, args: &[Datum]) -> ProcedureResult<Datum> {
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

pub fn eval(expr: Datum) -> ProcedureResult<Datum> {
    match expr {
        Datum::List(list) => call(&list[0], &list[1..]),
        _ => Ok(expr)
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
        assert_eq!(Datum::Number(3.0), result.unwrap());
    }

    #[test]
    fn test_eval() {
        let expr = Datum::List(
            vec![
                Datum::Symbol("+".to_string()),
                Datum::Number(1.0),
                Datum::Number(2.0)
            ]);
        let result = eval(expr);

        assert!(result.is_ok());
        assert_eq!(Datum::Number(3.0), result.unwrap());
    }
}

