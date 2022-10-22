use std::fmt;

#[derive(Debug, Clone)]
pub struct LogicError(pub String);

impl LogicError {
    pub fn new(message: String) -> LogicError {
        LogicError(message)
    }
}

impl fmt::Display for LogicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// macro_rules! logic_err {
//     ($($arg:tt)*) => {
//         Err(LogicError(format!($($arg:tt)*)))
//     };
// }
