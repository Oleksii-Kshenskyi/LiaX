use crate::{builtins::builtins_map, errors::*, lexer::*, types::*};

use std::collections::HashMap;

pub struct Parser {
    to_parse: Vec<Token>,
    identifiers: HashMap<String, DataType>,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            to_parse: tokens.to_owned(),
            identifiers: builtins_map(),
        }
    }

    // TODO: Parsing error reporting has to improve
    //       (especially once we switch to parsing from files).
    //       - start differentiating between different kinds of parsing errors;
    //       - start specifying where the actual errors happen
    //         (not just line/char number, but also argument number in a function, etc.)

    // REFACTOR: DataType and Token have many mirroring variants.
    //           Figure out a way to use DataType instead of Token in return type here.
    fn datatype_to_token(var: DataType) -> Token {
        match var {
            DataType::Unit => Token::Unit,
            DataType::Function(_) => unreachable!("Don't know how to show functions."),
            DataType::Int(i) => Token::Int(i.value),
            DataType::Borked(e) => Token::Borked(e),
        }
    }

    fn collapse_datatype(var: DataType, maybe_func_args: Option<Vec<DataType>>) -> Result<Token, LiaXError> {
        match var {
            DataType::Unit => Ok(Token::Unit),
            DataType::Function(func) => func.call(maybe_func_args.unwrap_or(vec![])).map(Self::datatype_to_token),
            DataType::Int(i) => Ok(Token::Int(i.value)),
            DataType::Borked(e) => Err(e),
        }
    }

    fn collapse_expr(&self, expr_size: usize, expr: &[Token]) -> Result<(usize, Token), LiaXError> {
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
            if let Some(func) = self.identifiers.get(&id) {
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
                        Token::Identifier(id) => {
                            if let Some(obj) = self.identifiers.get(id) {
                                obj.clone()
                            } else {
                                return DataType::Borked(LiaXError::new(ErrorType::Collapse(format!("Expected function argument, got unexpected identifier `{}`.", id))))
                            }
                        },
                        Token::Borked(e) => DataType::Borked(e.clone()),
                    })
                    .collect();
                Self::collapse_datatype(func.clone(), Some(args))
                .map(|t| (expr_size, t))
            } else {
                Err(LiaXError::new(ErrorType::Collapse(format!(
                    "Expected a known function name, got unknown identifier `{}`.",
                    id
                ))))
            }
        } else {
            Err(LiaXError::new(ErrorType::Collapse(format!("Can only collapse function calls. Expected an identifier (a function name) as the first token in the expression, got `{:?}` instead.", expr))))
        }
    }

    fn eval_single_expr(&self, starting_pos: usize, v: &[Token]) -> Result<(usize, Token), LiaXError> {
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
                Token::Borked(e) => return Err(e.clone()),
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
        pos += 1;
        while v[pos] != Token::CloseParen {
            if pos >= v.len() - 1 {
                return Err(LiaXError::new(ErrorType::Parsing(s("Reached the end of S-Expression, but got no `)` at the end. Have you forgotten to close the corresponding `(`?"))));
            }

            if v[pos] == Token::OpenParen {
                match self.eval_single_expr(pos, v) {
                    Err(e) => return Err(LiaXError::new(ErrorType::Eval(format!("{}", e)))),
                    Ok((shift, t)) => {
                        pos += shift;
                        flattened.push(t);
                    }
                }
            } else {
                flattened.push(v[pos].clone());

                pos += 1;
            }
        }
        flattened.push(v[pos].clone());

        self.collapse_expr(pos - starting_pos + 1, &flattened)
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
                Token::Borked(e) => return Err(e.clone()),
            }
        }

        let fst = v.first().unwrap();
        if v.len() >= 2 && *fst != Token::OpenParen {
            return Err(LiaXError::new(ErrorType::Parsing(format!(
                "Expected expression to start with an `(`, but it starts with `{:?}` instead.",
                fst
            ))));
        }

        let mut token_pos: usize = 1;
        let mut flattened_expr: Vec<Token> = vec![];
        if let Token::OpenParen = v.first().unwrap() {
            flattened_expr.push(Token::OpenParen);
        }
        while token_pos < v.len() {
            match v.get(token_pos) {
                Some(t) => {
                    if let Token::OpenParen = t {
                        match self.eval_single_expr(token_pos, v) {
                            Ok((shift, new_tok)) => {
                                token_pos += shift;
                                flattened_expr.push(new_tok);
                            }
                            Err(e) => {
                                return Err(LiaXError::new(ErrorType::Parsing(format!("{}", e))))
                            }
                        }
                    } else {
                        flattened_expr.push(t.clone());
                        token_pos += 1;
                    }
                }
                None => {
                    return Err(LiaXError::new(ErrorType::Parsing(s(
                        "Unexpected end of tokens while parsing.",
                    ))))
                }
            }
        }

        let (check_index, final_res) = self.eval_single_expr(0, &flattened_expr)?;
        if check_index < flattened_expr.len() {
            return Err(LiaXError::new(ErrorType::Parsing(format!("Expected the end of an S-Expression, but still have `{:?}` left. Please check that you don't have any rogue symbols outside of S-Expressions.", &flattened_expr[check_index..]))));
        }

        Ok(show_datatype(&final_res))
    }
}
