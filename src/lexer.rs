use crate::errors::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Int(i64),
    Identifier(String),

    // Eval'ed tokens
    Unit,
    Borked(LiaXError),
}

pub const VALID_ID_CHARS: &str = "+*-/_";

// TODO: either lexer, or parser, or both have to track their position
//       via a line number and a char number in the line
//       and report the numbers on every error.
//       (this is after we start parsing LiaX programs from files)
pub struct Lexer<'a> {
    pos: usize,
    expr: &'a str,
}

impl<'a> Lexer<'a> {
    fn char_is_valid_id(full_id: &str, c: char) -> bool {
        let is_number_ey = full_id.chars().all(|c| c.is_ascii_digit())
            || full_id.chars().nth(0).unwrap().is_ascii_digit();
        !is_number_ey && (c.is_alphanumeric() || VALID_ID_CHARS.find(c).is_some())
    }
    fn consume_whitespaces(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.consume_noclone();
            } else {
                return;
            }
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.expr.chars().nth(self.pos)
    }
    pub fn peek_next(&self) -> Option<char> {
        self.expr.chars().nth(self.pos + 1)
    }

    #[inline]
    pub fn consume_noclone(&mut self) {
        self.pos += 1;
    }

    pub fn new(expr: &'a str) -> Self {
        Self { pos: 0, expr }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LiaXError> {
        let mut lexed: Vec<Token> = vec![];
        let mut lexing = String::new();

        let lex_len = self.expr.len();
        self.consume_whitespaces();
        while self.pos < lex_len {
            match self.peek() {
                Some('(') => {
                    lexed.push(Token::OpenParen);
                    self.consume_noclone();
                }
                Some(')') => {
                    lexed.push(Token::CloseParen);
                    self.consume_noclone();
                }
                Some(c) if Self::char_is_valid_id(&format!("{}{}", lexing, c), c) => {
                    lexing.push(c);
                    let maybe_next = self.peek_next();
                    if maybe_next.is_none() && !lexing.is_empty() {
                        lexed.push(Token::Identifier(lexing.clone()));
                        lexing.clear();
                    } else if let Some(next) = maybe_next {
                        if next.is_whitespace() || next == ')' {
                            lexed.push(Token::Identifier(lexing.clone()));
                            lexing.clear();
                        }
                    }
                    self.consume_noclone();
                }
                Some(c)
                    if Self::char_is_valid_id(&format!("c{}", c), c)
                        && !lexing.is_empty()
                        && !c.is_ascii_digit()
                        && lexing.chars().nth(0).unwrap().is_ascii_digit() =>
                {
                    return Err(LiaXError::new(ErrorType::Lexing(s("Identifiers can't start with a digit. Start ids with a letter or `_` instead."))));
                }
                Some(c) if c.is_ascii_digit() => {
                    let token_is_digit = lexing.chars().all(|c| c.is_ascii_digit());
                    if lexing.is_empty() || token_is_digit {
                        lexing.push(c);
                    }
                    let maybe_next = self.peek_next();
                    if maybe_next.is_none() && !lexing.is_empty() && token_is_digit {
                        lexed.push(Token::Int(lexing.parse::<i64>().unwrap()));
                        lexing.clear();
                    } else if let Some(next) = maybe_next {
                        if next.is_whitespace() || next == ')' {
                            lexed.push(Token::Int(lexing.parse::<i64>().unwrap()));
                            lexing.clear();
                        }
                    }
                    self.consume_noclone();
                }
                Some(c) => {
                    return Err(LiaXError::new(ErrorType::Lexing(format!(
                        "at pos {}: couldn't lex char `{}`.",
                        self.pos, c
                    ))));
                }
                None => {
                    break;
                }
            }
            self.consume_whitespaces();
        }

        Ok(lexed)
    }
}
