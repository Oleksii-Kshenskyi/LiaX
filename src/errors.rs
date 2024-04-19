use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorType {
    Lexing(String),
    Parsing(String),
    Eval(String),
    Collapse(String),
}

pub fn s(s: &str) -> String {
    s.to_string()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiaXError {
    pub etype: ErrorType,
}

impl Display for LiaXError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.etype.clone() {
            ErrorType::Lexing(m) => write!(f, "Lexing error: `{}`", m),
            ErrorType::Parsing(m) => write!(f, "Parsing error: `{}`", m),
            ErrorType::Eval(m) => write!(f, "Evaluation error: `{}`", m),
            ErrorType::Collapse(m) => {
                write!(f, "Error while collapsing a flat expression: `{}`", m)
            }
        }
    }
}

impl Error for LiaXError {}

impl LiaXError {
    pub fn new(etype: ErrorType) -> Self {
        Self { etype }
    }
}
