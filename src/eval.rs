use crate::errors::*;
use crate::lexer::*;
use crate::parser::*;

// NOTE: eval.rs may not even be necessary at this point,
//       the entire process of evaluation and collapsing is
//       happening in parse.rs at this point.

pub fn evaluate_sexpr(sexpr: String) -> Result<String, LiaXError> {
    let to_parse = Lexer::new(&sexpr).lex()?;

    Parser::new(&to_parse).parse()
}
