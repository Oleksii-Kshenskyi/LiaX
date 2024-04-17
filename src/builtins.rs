use std::collections::HashMap;

use crate::{errors::*, types::*};

pub fn builtins_map() -> HashMap<String, BuiltinFn> {
    [("+".to_owned(), plus as BuiltinFn)].into_iter().collect()
}

pub fn plus(to_add: Vec<DataType>) -> LiaXResult {
    if !to_add.iter().all(|e| matches!(e, DataType::Int(_))) {
        return Err(LiaXError::new(s(
            "ERROR: Currently, you can only perform arithmetic operations on ints.",
        )));
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
