#![allow(non_snake_case)]
mod eval;
mod builtins;
mod types;
mod errors;

use crate::eval::evaluate_sexpr;

pub const THE_SEXPR: &str = "(+ 1 2)";

fn main() {
    println!("{} = {}", THE_SEXPR, evaluate_sexpr(THE_SEXPR));
}
