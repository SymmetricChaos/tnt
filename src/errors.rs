use std::{error::Error, fmt};

#[derive(Debug)]
enum LogicError {
    Specification(String),
    Generalization(String),
}


impl fmt::Display for LogicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            LogicError::Specification(desc) => write!(f, "Specification Error: {}", desc),
            LogicError::Generalization(desc) => write!(f, "Generalization Error: {}", desc),
        }
    } 
}

impl Error for LogicError {}