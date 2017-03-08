mod error;
mod types;
mod ops;
mod core;
mod read;

use std::io::{stdin, stdout, Write};
use read::read;
use core::eval;

fn main() {
    let mut input = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();

        input.clear();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "(exit)" {
                    break;
                }
                match read(&input) {
                    Some(datum) => match eval(&datum) {
                        Ok(object) => println!("{}", object),
                        Err(error) => println!("{}", error)
                    },
                    None =>
                        println!("Can't read data from {}.", input)
                }
            },
            Err(error) => println!("Error has occurred: {}", error)
        }
    }
}

#[cfg(test)]
mod tests {
    use ::types::*;
    use ::core::*;
    use ::read::*;

    #[test]
    fn test() {
        let expr = read("(+ 1 2)");
        assert!(expr.is_some());
        let result = eval(&expr.unwrap());
        assert!(result.is_ok());
        assert_eq!(Object::Number(3.0), result.unwrap());
    }
}
