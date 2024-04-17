use crate::errors::*;
use crate::lexer::*;
use crate::parser::*;
use crate::types::*;

pub fn evaluate_sexpr(sexpr: String) -> Result<String, LiaXError> {
    let to_parse = Lexer::new(&sexpr).lex()?;

    let to_eval = Parser::new(&to_parse).parse()?;

    evaluate_parsed_sexpr(to_eval)
}

fn evaluate_parsed_sexpr(instr: Instruction) -> Result<String, LiaXError> {
    match instr {
        Instruction::Show(showable) => match showable {
            DataType::Int(i) => Ok(show_datatype(&DataType::Int(i))),
            DataType::Function(func) => Ok(show_datatype(&DataType::Function(func))),
            DataType::Unit => Ok(show_datatype(&DataType::Unit)),
        },
        Instruction::Call(func) => func.call().map(|res| show_datatype(&res)),
        Instruction::NoOp => Ok(s("")),
    }
}
