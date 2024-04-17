use crate::{builtins::builtins_map, errors::*, lexer::*, types::*};

pub struct Parser {
    to_parse: Vec<Token>,
    // pos: usize,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            to_parse: tokens.to_owned(),
            // pos: 0,
        }
    }

    // TODO: Make the parser capable of recursively parsing s-expressions.
    pub fn parse(&mut self) -> Result<Instruction, LiaXError> {
        let v = &self.to_parse;
        if v.len() == 2
            && *v.first().unwrap() == Token::OpenParen
            && *v.get(1).unwrap() == Token::CloseParen
        {
            return Ok(Instruction::Show(DataType::Unit));
        }
        if v.is_empty() {
            return Ok(Instruction::NoOp);
        }

        if *v.first().unwrap() != Token::OpenParen {
            // we are trying to parse an atom/literal, there should only be a single token in the vector at this point
            if v.len() != 1 {
                return Err(LiaXError::new(ErrorType::Parsing(s("ERROR: expresion is not an S-Expression and not a single literal. These two are the only cases LiaX is handling currently."))));
            }

            // If we're here, this means that we got a single token which has to be a DataType so we could Show it.
            match v.first().unwrap() {
                Token::OpenParen => unreachable!(
                    "Shouldn't happen. We've already asserted that this token is not OpenParen."
                ),
                Token::CloseParen => {
                    return Err(LiaXError::new(ErrorType::Parsing(s(
                        "ERROR: Got a single token, `)`. Can't do anything with it.",
                    ))))
                }
                Token::Identifier(id) => {
                    return Err(LiaXError::new(ErrorType::Eval(format!(
                        "ERROR: found a single token, an unknown identifier `{}`.",
                        id
                    ))))
                }
                Token::Int(num) => return Ok(Instruction::Show(DataType::Int(IntType::new(*num)))),
            }
        }

        // now we're out of a "single token" case, and we're considering this a function call.
        if v.len() < 2 {
            Err(LiaXError::new(ErrorType::Parsing(s(
                "ERR: Expected an identifier after opening paren.",
            ))))
        } else if let Some(Token::Identifier(id)) = v.get(1) {
            match builtins_map().get(id) {
                Some(func) => {
                    if *v.last().unwrap() != Token::CloseParen {
                        return Err(LiaXError::new(ErrorType::Parsing(s("ERROR: Did you forget to have a closing parenthesis at the end of your expression?"))));
                    }
                    let args = if v.len() > 2 {
                        v[2..=(v.len() - 2)].iter().map(|tok| match tok {
                                    Token::CloseParen => unreachable!("Should not be possible for the token to be CloseParen."),
                                    Token::OpenParen => unreachable!("Should not be possible for the token to be OpenParen."),
                                    Token::Int(i) => DataType::Int(IntType::new(*i)),
                                    Token::Identifier(id) => unreachable!("ERROR: function argument is an identifier ({}). We don't know what to do with those for now.", id),
                                }).collect()
                    } else {
                        Vec::new()
                    };
                    Ok(Instruction::Call(FunctionType::new(
                        id.clone(),
                        args,
                        0..usize::MAX,
                        *func,
                    )))
                }
                None => Err(LiaXError::new(ErrorType::Parsing(format!(
                    "Can't find function named {} in this scope.",
                    &id
                )))),
            }
        } else {
            Err(LiaXError::new(ErrorType::Parsing(format!(
                "ERROR: Expected identifier as a first argument in a call, got {:?}",
                v.get(1).unwrap()
            ))))
        }
    }
}
