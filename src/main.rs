#![allow(non_snake_case)]
mod builtins;
mod errors;
mod eval;
mod lexer;
mod parser;
mod types;

use crate::errors::s;
use crate::eval::evaluate_sexpr;

pub const THE_SEXPR: &str = "(+ 10 20)";

// FIXME: Fix all the clippy issues and warnings.
fn main() {
    println!("{} = {}", THE_SEXPR, evaluate_sexpr(s(THE_SEXPR)).unwrap());
}
