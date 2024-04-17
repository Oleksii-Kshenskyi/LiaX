#![allow(non_snake_case)]

use liblia::errors::s;
use liblia::eval::evaluate_sexpr;

// TODO: Heavily cover evaluate_sexpr() with tests, including all corner cases.
pub const THE_SEXPR: &str = "(+ 3 3)";

// FIXME: Fix all the clippy issues and warnings.
fn main() {
    match evaluate_sexpr(s(THE_SEXPR)) {
        Ok(s) => println!("{} = {}", THE_SEXPR, s),
        Err(e) => println!("{}", e),
    }
}
