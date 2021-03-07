use std::fmt;

use crate::Properties::{is_term,is_well_formed_formula,is_var};
use crate::StringManipulation::{get_bound_vars};

#[derive(Debug)]
pub struct Term {
    pub s: String,
}

#[derive(Debug)]
pub struct Formula {
    pub s: String,
    pub bound_vars: Vec<String>
}

#[derive(Debug)]
pub struct Variable {
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
        if is_well_formed_formula(input) {
            return Formula{ s: input.to_owned(), bound_vars: get_bound_vars(input) }
        } else {
            panic!("{} is not a well formed formula",input)
        }
    }
}



impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Variable {
    pub fn new(input: &str) -> Variable {
        if is_var(input) {
            return Variable{ s: input.to_owned() }
        } else {
            panic!("{} is not a variable",input)
        }
    }
}
