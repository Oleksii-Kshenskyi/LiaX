use crate::{builtins::builtins_map, errors::*, lexer::*, types::*};

pub struct Parser {
    to_parse: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            to_parse: tokens.to_owned(),
        }
    }

    // TODO: Parsing error reporting has to improve
    //       (especially once we switch to parsing from files).
    //       - start differentiating between different kinds of parsing errors;
    //       - start specifying where the actual errors happen
    //         (not just line/char number, but also argument number in a function, etc.)

    // TODO: Remove the old parsing code when the recursive refactoring is done.
    // pub fn parse(&mut self) -> Result<Instruction, LiaXError> {
    //     let v = &self.to_parse;
    //     if v.len() == 2
    //         && *v.first().unwrap() == Token::OpenParen
    //         && *v.get(1).unwrap() == Token::CloseParen
    //     {
    //         return Ok(Instruction::Show(DataType::Unit));
    //     }
    //     if v.is_empty() {
    //         return Ok(Instruction::NoOp);
    //     }

    //     if *v.first().unwrap() != Token::OpenParen {
    //         // we are trying to parse an atom/literal, there should only be a single token in the vector at this point
    //         if v.len() != 1 {
    //             return Err(LiaXError::new(ErrorType::Parsing(s("ERROR: expression is not an S-Expression and not a single literal. These two are the only cases LiaX is handling currently."))));
    //         }

    //         // If we're here, this means that we got a single token which has to be a DataType so we could Show it.
    //         match v.first().unwrap() {
    //             Token::OpenParen => unreachable!(
    //                 "Shouldn't happen. We've already asserted that this token is not OpenParen."
    //             ),
    //             Token::CloseParen => {
    //                 return Err(LiaXError::new(ErrorType::Parsing(s(
    //                     "ERROR: Got a single token, `)`. Can't do anything with it.",
    //                 ))))
    //             }
    //             Token::Identifier(id) => {
    //                 return Err(LiaXError::new(ErrorType::Eval(format!(
    //                     "ERROR: found a single token, an unknown identifier `{}`.",
    //                     id
    //                 ))))
    //             }
    //             Token::Int(num) => return Ok(Instruction::Show(DataType::Int(IntType::new(*num)))),
    //         }
    //     }

    //     // now we're out of a "single token" case, and we're considering this a function call.
    //     if v.len() < 2 {
    //         Err(LiaXError::new(ErrorType::Parsing(s(
    //             "ERR: Expected an identifier after opening paren.",
    //         ))))
    //     } else if let Some(Token::Identifier(id)) = v.get(1) {
    //         match builtins_map().get(id) {
    //             Some(func) => {
    //                 if *v.last().unwrap() != Token::CloseParen {
    //                     return Err(LiaXError::new(ErrorType::Parsing(s("ERROR: Did you forget to have a closing parenthesis at the end of your expression?"))));
    //                 }
    //                 let args = if v.len() > 2 {
    //                     v[2..=(v.len() - 2)].iter().map(|tok| match tok {
    //                                 Token::CloseParen => unreachable!("Should not be possible for the token to be CloseParen."),
    //                                 Token::OpenParen => unreachable!("Should not be possible for the token to be OpenParen."),
    //                                 Token::Int(i) => DataType::Int(IntType::new(*i)),
    //                                 Token::Identifier(id) => unreachable!("ERROR: function argument is an identifier ({}). We don't know what to do with those for now.", id),
    //                             }).collect()
    //                 } else {
    //                     Vec::new()
    //                 };
    //                 Ok(Instruction::Call(FunctionType::new(
    //                     id.clone(),
    //                     args,
    //                     0..usize::MAX,
    //                     *func,
    //                 )))
    //             }
    //             None => Err(LiaXError::new(ErrorType::Parsing(format!(
    //                 "Can't find function named {} in this scope.",
    //                 &id
    //             )))),
    //         }
    //     } else {
    //         Err(LiaXError::new(ErrorType::Parsing(format!(
    //             "ERROR: Expected identifier as a first argument in a call, got {:?}",
    //             v.get(1).unwrap()
    //         ))))
    //     }
    // }

    fn datatype_to_token(var: DataType) -> Token {
        match var {
            DataType::Unit => Token::Unit,
            DataType::Function(_) => unreachable!("Don't know how to show functions."),
            DataType::Int(i) => Token::Int(i.value),
        }
    }

    fn collapse_datatype(var: DataType) -> Result<Token, LiaXError> {
        match var {
            DataType::Unit => Ok(Token::Unit),
            DataType::Function(func) => func.call().map(|res| Self::datatype_to_token(res)),
            DataType::Int(i) => Ok(Token::Int(i.value)),
        }
    }

    fn collapse_expr(expr: &[Token]) -> Result<(usize, Token), LiaXError> {
        if expr.len() < 2
            || expr[0] != Token::OpenParen
            || expr[expr.len() - 1] != Token::CloseParen
        {
            return Err(LiaXError::new(ErrorType::Collapse(format!(
                "Error while collapsing `{:?}`: expected a single non-recursive expression.",
                expr
            ))));
        }
        if expr
            .iter()
            .filter(|el| matches!(el, Token::OpenParen))
            .count()
            != 1
            || expr
                .iter()
                .filter(|el| matches!(el, Token::CloseParen))
                .count()
                != 1
        {
            return Err(LiaXError::new(ErrorType::Collapse(format!("Error while collapsing `{:?}`: the expression has an uneven number of parentheses (expected 1 of `(` and 1 of `)`).", expr))));
        }

        if let Token::Identifier(id) = expr[1].clone() {
            if let Some(func) = builtins_map().get(&id) {
                let args: Vec<DataType> = expr[2..expr.len() - 1]
                    .iter()
                    .map(|el| match el {
                        Token::OpenParen => {
                            unreachable!("Open paren cannot be a function argument.")
                        }
                        Token::CloseParen => {
                            unreachable!("Close paren cannot be a function argument.")
                        }
                        Token::Unit => DataType::Unit,
                        Token::Int(i) => DataType::Int(IntType::new(*i)),
                        Token::Identifier(id) => unreachable!(
                            "Expected a function argument, got an identifier `{}`.",
                            id
                        ),
                    })
                    .collect();
                return Self::collapse_datatype(DataType::Function(FunctionType::new(
                    id, args, *func,
                )))
                .map(|t| (1, t));
            } else {
                return Err(LiaXError::new(ErrorType::Collapse(format!(
                    "Expected a known function name, got unknown identifier `{}`.",
                    id
                ))));
            }
        } else {
            return Err(LiaXError::new(ErrorType::Collapse(format!("Can only collapse function calls. Expected an identifier (a function name) as the first token in the expression, got `{:?}` instead.", expr))));
        }
    }

    fn eval_single_expr(starting_pos: usize, v: &[Token]) -> Result<(usize, Token), LiaXError> {
        if v.len() == 1 {
            match &v[0] {
                Token::OpenParen => {
                    return Err(LiaXError::new(ErrorType::Eval(s(
                        "The entire S-Expression is a single opening parenthesis.",
                    ))))
                }
                Token::CloseParen => {
                    return Err(LiaXError::new(ErrorType::Eval(s(
                        "The entire S-Expression is a single closing parenthesis.",
                    ))))
                }
                Token::Unit => return Ok((1, Token::Unit)),
                Token::Identifier(id) => {
                    return Err(LiaXError::new(ErrorType::Eval(format!(
                        "Got a single unknown identifier `{}`.",
                        id
                    ))))
                }
                Token::Int(i) => return Ok((1, Token::Int(*i))),
            }
        }

        let mut pos = starting_pos;
        let mut flattened: Vec<Token> = vec![];

        if v.len() == 2 {
            if v[0] == Token::OpenParen && v[1] == Token::CloseParen {
                return Ok((2, Token::Unit));
            } else {
                return Err(LiaXError::new(ErrorType::Eval(format!(
                    "Expected a full S-Expression, got `{:?}`.",
                    v
                ))));
            }
        }

        if v[pos] != Token::OpenParen {
            return Err(LiaXError::new(ErrorType::Eval(format!(
                "Expected an S-Expression to start with an `(`, it starts with `{:?}` instead.",
                v[pos]
            ))));
        }
        flattened.push(Token::OpenParen);
        let mut recursive = false;
        pos += 1;
        while v[pos] != Token::CloseParen {
            if pos >= v.len() - 1 {
                return Err(LiaXError::new(ErrorType::Parsing(s("Reached the end of S-Expression, but got no `)` at the end. Have you forgotten to close the corresponding `(`?"))));
            }

            if v[pos] == Token::OpenParen {
                recursive = true;
                match Self::eval_single_expr(pos, v) {
                    Err(e) => return Err(LiaXError::new(ErrorType::Eval(format!("{}", e)))),
                    Ok((shift, t)) => {
                        pos = pos + shift - 1;
                        flattened.push(t);
                    }
                }
            } else {
                flattened.push(v[pos].clone());
            }
            pos += 1;
        }
        flattened.push(v[pos].clone());
        if !recursive {
            return Self::collapse_expr(&flattened);
        }

        Self::eval_single_expr(0, &flattened)
    }

    pub fn parse(&mut self) -> Result<String, LiaXError> {
        let v = &self.to_parse;
        if v.is_empty() {
            return Ok(s(""));
        } else if v.len() == 1 {
            match v.first().unwrap() {
                Token::Identifier(id) => {
                    return Err(LiaXError::new(ErrorType::Eval(format!(
                        "expected a literal or an S-Expression, got an unknown identifier `{}`.",
                        id
                    ))))
                }
                Token::Int(i) => return Ok(i.to_string()),
                Token::OpenParen => {
                    return Err(LiaXError::new(ErrorType::Parsing(s(
                        "Got a single open paren.",
                    ))))
                }
                Token::CloseParen => {
                    return Err(LiaXError::new(ErrorType::Parsing(s(
                        "Got a single closing paren.",
                    ))))
                }
                Token::Unit => return Ok(s("()")),
            }
        }

        let mut token_pos: usize = 0;
        let mut flattened_expr: Vec<Token> = vec![];
        while token_pos < v.len() {
            match v.get(token_pos) {
                Some(t) => {
                    if let Token::OpenParen = t {
                        match Self::eval_single_expr(token_pos, &v) {
                            Ok((shift, new_tok)) => {
                                token_pos += shift;
                                flattened_expr.push(new_tok);
                                continue;
                            }
                            Err(e) => {
                                return Err(LiaXError::new(ErrorType::Parsing(format!("{}", e))))
                            }
                        }
                    }
                }
                None => {
                    return Err(LiaXError::new(ErrorType::Parsing(s(
                        "Unexpected end of tokens while parsing.",
                    ))))
                }
            }
            token_pos += 1;
        }

        let (_, final_res) = Self::eval_single_expr(0, &flattened_expr)?;
        Ok(show_datatype(&final_res))
    }
}
