use crate::errors::*;
use crate::lexer::Token;

// TODO: Clean up the types.rs file. There are a couple warnings and commented out
//       pieces of code, they may be worth removing.

pub type LiaXResult = Result<DataType, LiaXError>;
pub type BuiltinFn = fn(Vec<DataType>) -> LiaXResult;

#[derive(Clone, Debug)]
pub struct IntType {
    pub value: i64,
}
impl IntType {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionType {
    // name: String,
    args: Vec<DataType>,
    pointer: BuiltinFn,
}

impl FunctionType {
    pub fn new(
        // name: String,
        args: Vec<DataType>,
        pointer: BuiltinFn,
    ) -> Self {
        Self {
            // name: name.to_owned(),
            args,
            pointer,
        }
    }

    pub fn call(&self) -> LiaXResult {
        (self.pointer)(self.args.clone())
    }
}

// TODO: introduce floats
// TODO: introduce strings
#[derive(Clone, Debug)]
pub enum DataType {
    Int(IntType),
    Function(FunctionType),
    Unit,
}

pub fn show_datatype(atom: &Token) -> String {
    match atom {
        Token::Int(i) => i.to_string(),
        Token::Unit => s("()"),
        t => format!("show_datatype(): don't know how to show `{:?}`.", *t),
    }
}
