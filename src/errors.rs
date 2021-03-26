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

/* 
macro_rules! logic_error {
    ($($arg:tt)*) => {
        LogicError::new(format!($arg))
    };
}
*/

#[test]
fn check_error() {
    let msg = format!("Specification Error");
    let err = LogicError::new(msg);
    println!("{:?}",err)
}
