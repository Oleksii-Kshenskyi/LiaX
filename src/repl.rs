#![allow(non_snake_case)]

use liblia::errors::s;
use liblia::eval::evaluate_sexpr;

pub const THE_SEXPR: &str = "(+ 3 3)";

fn main() {
    match evaluate_sexpr(s(THE_SEXPR)) {
        Ok(s) => println!("{} = {}", THE_SEXPR, s),
        Err(e) => println!("{}", e),
    }
}
