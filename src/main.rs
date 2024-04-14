#![allow(non_snake_case)]
mod eval;
mod builtins;
mod types;
mod errors;
mod lexer;

use crate::eval::evaluate_sexpr;
use crate::lexer::Lexer;

pub const THE_SEXPR: &str = "(+ 1 2)";

// TODO: Implemented the statement execution part, but not the parsing.
fn main() {
    // FIXME: lexing is here, but it's giga dumb and stuck on infinite loop right now.
    let lexed = Lexer::new(THE_SEXPR).lex().unwrap();
    println!("{} Lexed: {:#?}\n", THE_SEXPR, lexed);
    println!("{} = {}", THE_SEXPR, evaluate_sexpr(THE_SEXPR));
}
