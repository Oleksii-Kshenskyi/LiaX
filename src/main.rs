#![allow(non_snake_case)]
mod builtins;
mod errors;
mod eval;
mod lexer;
mod types;

use crate::eval::evaluate_sexpr;
use crate::lexer::Lexer;

pub const THE_SEXPR: &str = "(+ 1 2)";

// TODO: Implemented the statement execution part, but not the parsing.
fn main() {
    // TODO: simple lexing seems to be working, now implement parsing and execution.
    let lexed = Lexer::new(THE_SEXPR).lex().unwrap();
    println!("{} Lexed: {:#?}\n", THE_SEXPR, lexed);
    println!("{} = {}", THE_SEXPR, evaluate_sexpr(THE_SEXPR));
}
