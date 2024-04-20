use std::collections::HashMap;

use crate::{errors::*, types::*};

// TODO: introduce string library
// TODO: introduce functional iteration (map, filter, reduce, all, etc.)
//       This also requires introducing functions as function arguments.
// TODO: introduce printing
// TODO: introduce file IO

fn dt_for_builtin(pointer: BuiltinFn) -> DataType {
    DataType::Function(FunctionType::new(pointer))
}

// TODO: Implement partial application of functions!
pub fn builtins_map() -> HashMap<String, DataType> {
    [
        ("+".to_owned(), dt_for_builtin(add)),
        ("-".to_owned(), dt_for_builtin(sub)),
        ("*".to_owned(), dt_for_builtin(mul)),
        ("/".to_owned(), dt_for_builtin(div)),
        ("list".to_owned(), dt_for_builtin(list)),
        ("map".to_owned(), dt_for_builtin(map)),
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
    if to_add.len() > 1
        && to_add[1..].iter().any(|e| {
            if let DataType::Int(IntType { value: i }) = e {
                *i == 0
            } else {
                unreachable!("Non-int in division while checking for zero division.")
            }
        })
    {
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
        iter.map(|dt| {
            if let DataType::Int(i) = dt {
                i.value
            } else {
                unreachable!("INTERNAL COMPILER ERROR: Non-int in division!")
            }
        })
        .fold(first, |acc, x| acc / x),
    )))
}

pub fn list(elems: Vec<DataType>) -> LiaXResult {
    Ok(DataType::List(elems))
}

pub fn map(args: Vec<DataType>) -> LiaXResult {
    if args.len() != 2 {
        return Err(LiaXError::new(ErrorType::Eval(format!("(map) function expects exactly two arguments: a function to apply to a list element and the list to apply it to. Got these arguments instead: `{:?}`", args))));
    }
    if let DataType::Function(f) = args.first().unwrap() {
        if let DataType::List(v) = args.get(1).unwrap() {
            let results = v.iter()
                    .map(|el| f.call(vec![el.clone()]))
                    .collect::<Vec<_>>();
            let mut oks: Vec<DataType> = vec![];
            for res in results {
                match res {
                    Err(e) => return Err(e),
                    Ok(dt) => oks.push(dt),
                }
            }
            return Ok(DataType::List(oks));
        } else {
            return Err(LiaXError::new(ErrorType::Eval(format!("(map) expects a list to apply the first argument's function to as the second argument. Got `{:?} instead.`", args.get(1).unwrap()))));
        }
    } else {
        return Err(LiaXError::new(ErrorType::Eval(format!(
            "(map) expects a function object as the first argument, got `{:?}` instead.",
            args.first().unwrap()
        ))));
    }
}
