#![allow(non_snake_case)]

use liblia::errors::s;
use liblia::eval::evaluate_sexpr;

pub const THE_SEXPR: &str = "(/ 50)";

// TODO: end goal: Write a program (in a file) in LiaX that
//       encrypts and decrypts a file with a Vigenere-style algorithm.
// TODO: end goal: make this main into an actual REPL.
fn main() {
    match evaluate_sexpr(s(THE_SEXPR)) {
        Ok(s) => println!("{} = {}", THE_SEXPR, s),
        Err(e) => println!("{}", e),
    }
}
