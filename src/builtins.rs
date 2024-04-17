use std::collections::HashMap;

use crate::{errors::*, types::*};

// TODO: introduce string functions
// TODO: introduce functional iteration (map, filter, reduce, all, etc.)
// TODO: introduce printing
// TODO: introduce file IO

pub fn builtins_map() -> HashMap<String, BuiltinFn> {
    [("+".to_owned(), plus as BuiltinFn)].into_iter().collect()
}

pub fn plus(to_add: Vec<DataType>) -> LiaXResult {
    if !to_add.iter().all(|e| matches!(e, DataType::Int(_))) {
        return Err(LiaXError::new(ErrorType::Eval(s(
            "ERROR: Currently, you can only perform arithmetic operations on ints.",
        ))));
    }

    Ok(DataType::Int(IntType::new(
        to_add
            .iter()
            .map(|dt| {
                if let DataType::Int(i) = dt {
                    i.value
                } else {
                    unreachable!("INTERNAL COMPILER ERROR: Non-int in addition!")
                }
            })
            .sum::<i64>(),
    )))
}
