mod error;
mod types;
mod core;
mod read;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use ::error::*;
    use ::types::*;
    use ::core::*;
    use ::read::*;

    #[test]
    fn test() {
        let expr = read("(+ 1 2)");
        assert!(expr.is_some());
        let result = eval(expr.unwrap());
        assert!(result.is_ok());
        assert_eq!(Datum::Number(3.0), result.unwrap());
    }
}
