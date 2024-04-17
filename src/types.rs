use crate::errors::LiaXError;

use std::ops::Range;

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
    name: String,
    args_limit_lower: usize,
    args_limit_higher: Option<usize>,
    args: Vec<DataType>,
    pointer: BuiltinFn,
}

impl FunctionType {
    pub fn new(
        name: String,
        args: Vec<DataType>,
        arg_limits: Range<usize>,
        pointer: BuiltinFn,
    ) -> Self {
        Self {
            name: name.to_owned(),
            args_limit_lower: arg_limits.start,
            args_limit_higher: if arg_limits.end != usize::MAX {
                Some(arg_limits.end)
            } else {
                None
            },
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
    // FIXME: Fix this warning.
    Function(FunctionType),
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Show(DataType),
    Call(FunctionType),
}

pub fn show_datatype(atom: &DataType) -> String {
    match atom {
        DataType::Int(i) => i.value.to_string(),
        DataType::Function(func) => format!("({} {})", func.name, format!("{{{} to {:?} args}}", func.args_limit_lower, func.args_limit_higher)),
    }
}
