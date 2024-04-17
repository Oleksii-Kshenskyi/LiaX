use crate::errors::LiaXError;
use crate::lexer::*;
use crate::parser::*;
use crate::types::*;

// TODO: remove debug printing once lexing/parsing are working.
pub fn evaluate_sexpr(sexpr: String) -> Result<String, LiaXError> {
    let to_parse = Lexer::new(&sexpr).lex()?;
    // println!("{} Lexed: {:#?}\n", &sexpr, to_parse);

    let to_eval = Parser::new(&to_parse).parse()?;
    // println!("{:#?} Parsed: {:#?}\n", &to_parse, &to_eval);

    evaluate_parsed_sexpr(to_eval)
}

fn evaluate_parsed_sexpr(instr: Instruction) -> Result<String, LiaXError> {
    match instr {
        Instruction::Show(showable) => match showable {
            DataType::Int(i) => Ok(show_datatype(&DataType::Int(i))),
            DataType::Function(func) => Ok(show_datatype(&DataType::Function(func))),
        },
        Instruction::Call(func) => func.call().map(|res| show_datatype(&res)),
    }
}
