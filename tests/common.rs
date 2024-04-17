use liblia::errors::{ErrorType, LiaXError};

pub fn assert_lexing_error(res: Result<String, LiaXError>) -> bool {
    match res {
        Ok(_) => false,
        Err(LiaXError {
            etype: ErrorType::Lexing(_),
        }) => true,
        Err(_) => false,
    }
}

pub fn assert_parsing_error(res: Result<String, LiaXError>) -> bool {
    match res {
        Ok(_) => false,
        Err(LiaXError {
            etype: ErrorType::Parsing(_),
        }) => true,
        Err(_) => false,
    }
}

pub fn assert_eval_error(res: Result<String, LiaXError>) -> bool {
    match res {
        Ok(_) => false,
        Err(LiaXError {
            etype: ErrorType::Eval(_),
        }) => true,
        Err(_) => false,
    }
}
