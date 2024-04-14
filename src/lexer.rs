use crate::errors::*;

#[derive(Debug)]
pub enum Lexed {
    OpenParen,
    CloseParen,
    Int(i64),
    Identifier(String),
}

pub const VALID_ID_CHARS: &'static str = "+*-/";

pub struct Lexer {
    pos: usize,
    expr: &'static str,
}

impl Lexer {
    fn char_is_valid_id(c: char) -> bool {
        c.is_alphanumeric() || VALID_ID_CHARS.find(c).is_some()
    }
    fn consume_whitespaces(&mut self) {
        while self.peek().is_whitespace() {
            self.consume_noclone();
        }
    }


    pub fn peek(&self) -> char {
        self.expr.chars().nth(self.pos).clone().unwrap()
    }
    pub fn peek_next(&self) -> Option<char> {
        self.expr.chars().nth(self.pos + 1).clone()
    }

    pub fn consume(&mut self) -> char {
        let consumed = self.expr.chars().nth(self.pos).clone().unwrap();
        self.pos += 1;
        consumed
    }
    #[inline]
    pub fn consume_noclone(&mut self) {
        self.pos += 1;
    }

    pub fn new(expr: &'static str) -> Self {
        Self { pos: 0, expr }
    }

    pub fn lex(&mut self) -> Result<Vec<Lexed>, LiaXError> {
        let mut lexed: Vec<Lexed> = vec![];
        let mut lexing = String::new();

        let lex_len = self.expr.len(); 
        while self.pos < lex_len {
            match self.peek() {
                '(' => {
                lexed.push(Lexed::OpenParen);
                self.consume_noclone();
                }
                ')' => {
                    lexed.push(Lexed::CloseParen);
                    self.consume_noclone();
                }
                c if c.is_digit(10) => {
                    let token_is_digit = lexing.chars().all(|c| c.is_digit(10));
                    if lexing.is_empty() || token_is_digit {
                        lexing.push(c);
                    }
                    if let Some(next) = self.peek_next() {
                        if next.is_whitespace() {
                            lexed.push(Lexed::Int(lexing.parse::<i64>().unwrap()));
                            lexing.clear();
                            self.consume_noclone();
                        }
                    }
                }
                c if Self::char_is_valid_id(c) => {
                    lexing.push(c);
                    if let Some(next) = self.peek_next() {
                        if next.is_whitespace() {
                            lexed.push(Lexed::Identifier(lexing.clone()));
                            lexing.clear();
                            self.consume_noclone();
                        }
                    }
                }
                c => return Err(LiaXError::new(format!("Lexer Error at pos {}: couldn't lex char `{}`.", self.pos, c))),
            }
            self.consume_whitespaces();
        }

        Ok(lexed)
    }
}