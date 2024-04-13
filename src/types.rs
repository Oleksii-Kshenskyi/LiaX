use crate::errors::LiaXError;

pub type LiaXResult = Result<DataType, LiaXError>;
pub type BuiltinFn = fn(Vec<DataType>) -> LiaXResult;

#[derive(Clone, Debug)]
pub struct IntType {
    pub value: i64
}
impl IntType {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionType {
    name: String,
    args: Vec<DataType>,
    pointer: BuiltinFn,
}

impl FunctionType {
    pub fn new(name: &str, args: Vec<DataType>, pointer: BuiltinFn) -> Self {
        Self {
            name: name.to_owned(),
            args,
            pointer,
        }
    }

    pub fn call(&self) -> LiaXResult {
        (self.pointer)(self.args.clone())
    }
}

#[derive(Clone, Debug)]
pub enum DataType {
    Int(IntType),
    Function(FunctionType),
}