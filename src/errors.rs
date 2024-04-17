use std::error::Error;
use std::fmt::Display;

pub fn s(s: &str) -> String {
    s.to_string()
}

#[derive(Debug)]
pub struct LiaXError {
    text: String,
}

impl Display for LiaXError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Error for LiaXError {}

impl LiaXError {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
