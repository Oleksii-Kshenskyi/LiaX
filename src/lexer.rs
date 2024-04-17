use crate::errors::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Int(i64),
    Identifier(String),
}

pub const VALID_ID_CHARS: &'static str = "+*-/";

pub struct Lexer<'a> {
    pos: usize,
    expr: &'a str,
}

impl<'a> Lexer<'a> {
    fn char_is_valid_id(c: char) -> bool {
        c.is_alphanumeric() || VALID_ID_CHARS.find(c).is_some()
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
        self.expr.chars().nth(self.pos).clone()
    }
    pub fn peek_next(&self) -> Option<char> {
        self.expr.chars().nth(self.pos + 1).clone()
    }

    // TODO: Figure out if you need this consuming function at all. If not, remove this and
    //       rename consume_noclone() into just consume() and leave it as the only option.
    // pub fn consume(&mut self) -> char {
    //     let consumed = self.expr.chars().nth(self.pos).clone().unwrap();
    //     self.pos += 1;
    //     consumed
    // }

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
        while self.pos < lex_len {
            // println!("self pos: {}, lex_len: {}", self.pos, lex_len);
            match self.peek() {
                Some('(') => {
                    println!("OpenParen, pos = {}", self.pos);
                    lexed.push(Token::OpenParen);
                    self.consume_noclone();
                }
                Some(')') => {
                    println!("CloseParen, pos = {}", self.pos);
                    lexed.push(Token::CloseParen);
                    self.consume_noclone();
                }
                Some(c) if c.is_digit(10) => {
                    // TODO: remove debug printing once lexing/parsing are working.
                    println!("C is digit: {}, pos = {}", c, self.pos);
                    let token_is_digit = lexing.chars().all(|c| c.is_digit(10));
                    if lexing.is_empty() || token_is_digit {
                        lexing.push(c);
                    }
                    if let Some(next) = self.peek_next() {
                        // TODO: remove debug printing once lexing/parsing are working.
                        println!("C is digit: {}, next is whitespace, pos = {}", c, self.pos);
                        if next.is_whitespace() || next == ')' {
                            lexed.push(Token::Int(lexing.parse::<i64>().unwrap()));
                            lexing.clear();
                        }
                    }
                    self.consume_noclone();
                }
                Some(c) if Self::char_is_valid_id(c) => {
                    // TODO: remove debug printing once lexing/parsing are working.
                    println!("C is id: {}, pos = {}", c, self.pos);
                    lexing.push(c);
                    if let Some(next) = self.peek_next() {
                        if next.is_whitespace() {
                            lexed.push(Token::Identifier(lexing.clone()));
                            lexing.clear();
                            self.consume_noclone();
                        }
                    }
                }
                Some(c) => {
                    println!("C error: {}, pos = {}", c, self.pos);
                    return Err(LiaXError::new(ErrorType::Lexing(format!(
                        "at pos {}: couldn't lex char `{}`.",
                        self.pos, c
                    ))));
                }
                None => {
                    println!("nope, pos is {}", self.pos);
                    break;
                }
            }
            self.consume_whitespaces();
            // println!("pos after consume whitespaces: {}", self.pos);
        }

        Ok(lexed)
    }
}
