use std::collections::HashMap;

use crate::{errors::*, types::*};

// TODO: introduce string functions
// TODO: introduce functional iteration (map, filter, reduce, all, etc.)
// TODO: introduce printing
// TODO: introduce file IO

pub fn builtins_map() -> HashMap<String, BuiltinFn> {
    [
        ("+".to_owned(), add as BuiltinFn),
        ("-".to_owned(), sub as BuiltinFn),
        ("*".to_owned(), mul as BuiltinFn),
        ("/".to_owned(), div as BuiltinFn),
    ]
    .into_iter()
    .collect()
}

pub fn add(to_add: Vec<DataType>) -> LiaXResult {
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

pub fn mul(to_add: Vec<DataType>) -> LiaXResult {
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
                    unreachable!("INTERNAL COMPILER ERROR: Non-int in multiplication!")
                }
            })
            .product::<i64>(),
    )))
}

pub fn sub(to_add: Vec<DataType>) -> LiaXResult {
    if !to_add.iter().all(|e| matches!(e, DataType::Int(_))) {
        return Err(LiaXError::new(ErrorType::Eval(s(
            "ERROR: Currently, you can only perform arithmetic operations on ints.",
        ))));
    }
    if to_add.len() == 1 {
        if let DataType::Int(it) = to_add.first().unwrap() {
            return Ok(DataType::Int(IntType::new(-it.value)));
        }
    }

    Ok(DataType::Int(IntType::new(
        to_add
            .iter()
            .enumerate()
            .map(|(index, dt)| {
                if let DataType::Int(i) = dt {
                    if index == 0 {
                        i.value
                    } else {
                        -i.value
                    }
                } else {
                    unreachable!("INTERNAL COMPILER ERROR: Non-int in multiplication!")
                }
            })
            .sum::<i64>(),
    )))
}

pub fn div(to_add: Vec<DataType>) -> LiaXResult {
    if !to_add.iter().all(|e| matches!(e, DataType::Int(_))) {
        return Err(LiaXError::new(ErrorType::Eval(s(
            "ERROR: Currently, you can only perform arithmetic operations on ints.",
        ))));
    }
    if to_add.len() > 1 && to_add[1..].iter().any(|e| {
        if let DataType::Int(IntType { value: i }) = e {
            *i == 0
        } else {
            unreachable!("Non-int in division while checking for zero division.")
        }
    }) {
        return Err(LiaXError::new(ErrorType::Eval(s(
            "Tried to divide by zero.",
        ))));
    }

    let mut iter = to_add.iter();
    let first = match iter.next() {
        None => 1 as i64,
        Some(DataType::Int(IntType { value: i })) => *i,
        dt => unreachable!("Rogue data type in division: `{:?}`", dt),
    };

    Ok(DataType::Int(IntType::new(
        iter
            .map(|dt| {
                if let DataType::Int(i) = dt {
                    i.value
                } else {
                    unreachable!("INTERNAL COMPILER ERROR: Non-int in division!")
                }
            })
            .fold(first, |acc, x| acc / x),
    )))
}
