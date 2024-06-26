use crate::errors::*;
use crate::lexer::Token;

// TODO: Move everything but show_datatype() to LiaXResult instead of
//       String in the Ok type.
pub type LiaXResult = Result<DataType, LiaXError>;
pub type BuiltinFn = fn(Vec<DataType>) -> LiaXResult;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntType {
    pub value: i64,
}
impl IntType {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionType {
    // name: String,
    // args: Vec<DataType>,
    pointer: BuiltinFn,
}

impl FunctionType {
    pub fn new(
        // name: String,
        // args: Vec<DataType>,
        pointer: BuiltinFn,
    ) -> Self {
        Self {
            // name: name.to_owned(),
            // args,
            pointer,
        }
    }

    pub fn call(&self, args: Vec<DataType>) -> LiaXResult {
        (self.pointer)(args)
    }
}

// TODO: introduce floats
// TODO: introduce strings
// TODO: introduce function type (functions as arguments for higher order functions)
// TODO: separate but related to the previous TODO: implement lambda (anonymous) functions
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataType {
    Int(IntType),
    Function(FunctionType),
    List(Vec<DataType>),
    Unit,
    Borked(LiaXError),
}

// REFACTOR: Once all Tokens are switched to DataTypes and the "Eval'ed tokens" are gone,
//           refactor showing DataType::List properly, using show_datatype for each
//           member of the list.
pub fn show_datatype(atom: &Token) -> String {
    match atom {
        Token::Int(i) => i.to_string(),
        Token::Unit => s("()"),
        Token::List(v) => format!("{:?}", v),
        t => format!("show_datatype(): don't know how to show `{:?}`.", *t),
    }
}
