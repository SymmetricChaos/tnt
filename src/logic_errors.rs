use std::{error::Error, fmt};

#[derive(Debug)]
pub struct LogicError {
    message: String
}

impl LogicError {
    pub fn new(message: String) -> LogicError {
        LogicError{ message }
    }
}


impl fmt::Display for LogicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    } 
}

impl Error for LogicError {}