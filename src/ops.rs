use error::*;
use types::*;

pub fn add(args: &[Datum]) -> ProcedureResult<Object> {
    let mut sum = 0.0;
    for datum in args {
        match *datum {
            Datum::Number(x) => sum += x,
            _ => return Err(ProcedureCallError::InvalidArgument)
        }
    }
    Ok(Object::Number(sum))
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::*;

    #[test]
    fn test_add() {
        let args = [
            Datum::Number(1.0),
            Datum::Number(2.0),
            Datum::Number(3.0)
        ];
        let result = add(&args);
        assert!(result.is_ok());
        assert_eq!(Object::Number(6.0), result.unwrap());
    }
}
