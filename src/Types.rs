use std::fmt;

use crate::Properties::{is_term,is_atom,is_compound};


#[derive(Debug)]
pub struct Term {
    pub s: String,
}

#[derive(Debug)]
pub struct Formula {
    pub s: String,
}


impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Term {
    pub fn new(input: &str) -> Term {
        if is_term(input) {
            return Term{ s: input.to_owned() }
        } else {
            panic!("{} is not a well formed term",input)
        }
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Formula {
    pub fn new(input: &str) -> Formula {
        if is_compound(input) || is_atom(input) {
            return Formula{ s: input.to_owned() }
        } else {
            panic!("{} is not a well formed formula",input)
        }
    }
}
