use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct LiaXError {
    text: &'static str,
}

impl Display for LiaXError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Error for LiaXError {}

impl LiaXError {
    pub fn new(text: &'static str) -> Self {
        Self { text }
    }
}