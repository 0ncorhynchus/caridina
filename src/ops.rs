use error::*;
use types::*;
use core::eval;

/// The syntax `if`
///
/// (if <test> <consequent> <alternate>)
/// (if <test> <consequent>)
///
/// # Scheme Example
///
/// ```lisp
/// (if (> 3 2) 'yes 'no)  ; = 'yes
/// (if (> 2 3) 'yes 'no)  ; = 'no
/// (if (> 3 2)
///     (- 3 2)
///     (+ 3 2))           ; = 1
/// ```
pub fn if_proc(args:&[Datum]) -> Result<Object> {
    match args.len() {
        3 => {
            if eval(&args[0])? != Object::Boolean(false) {
                eval(&args[1])
            } else {
                eval(&args[2])
            }
        },
        2 => {
            if eval(&args[0])? != Object::Boolean(false) {
                eval(&args[1])
            } else {
                Ok(Object::Empty)
            }
        },
        _ => return Err(Error::SyntaxError)
    }
}

/// The procedure `+`
///
/// (+ z_1 ...)
///
/// # Scheme Example
///
/// ```lisp
/// (+ 3 4)  ; = 7
/// (+ 3)    ; = 3
/// (+)      ; = 0
/// ```
pub fn add(args: &[Datum]) -> Result<Object> {
    let mut sum = 0.0;
    for datum in args {
        match eval(datum)? {
            Object::Number(x) => sum += x,
            _ => return Err(Error::InvalidArgument)
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
            Datum::Number(3.0),
        ];
        let result = add(&args);
        assert!(result.is_ok());
        assert_eq!(Object::Number(6.0), result.unwrap());
    }

    #[test]
    fn test_if() {
        let args = [
            Datum::Boolean(true),
            Datum::Number(1.0),
            Datum::Number(2.0),
        ];
        let result = if_proc(&args);
        assert!(result.is_ok());
        assert_eq!(Object::Number(1.0), result.unwrap());
    }
}
