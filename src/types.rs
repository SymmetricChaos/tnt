
use std::fmt;

use crate::properties::{is_equation,is_num,is_var,is_simple_formula,is_formula};
use crate::latex::to_latex;

pub enum Formula {
    Simple(String),
    Complex(String),
}

impl Formula {
    pub fn new(input: &str) -> Formula {
        if is_simple_formula(input) {
            return Formula::Simple(input.to_owned())
        } else if is_formula(input) {
            return Formula::Complex(input.to_owned()) 
        } else {
            panic!("{} is not a well formed formula",input)
        }
    }

    // Fast creation of variants without checking
    pub fn new_simple(input: &str) -> Formula {
        return Formula::Simple(input.to_owned())
    }

    pub fn new_complex(input: &str) -> Formula {
        return Formula::Complex(input.to_owned())
    }

    pub fn latex(&self, dent: usize) -> String {
        to_latex(self.to_string(),dent)
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Formula::Simple(form) => write!(f, "{}", form),
            Formula::Complex(form) => write!(f, "{}", form),
        }
    }
}



pub enum Term {
    Variable(String),
    Number(String),
    Equation(String),
}


impl Term {
    pub fn new(input: &str) -> Term {
        if is_num(input) {
            return Term::Number(input.to_owned())
        } else if is_var(input) {
            return Term::Variable(input.to_owned())
        } else if is_equation(input) {
            return Term::Equation(input.to_owned())
        } else {
            panic!("{} is not a valid Term",input)
        }
    }

    // Fast creation of variants without checking
    pub fn new_variable(input: &str) -> Term {
        return Term::Variable(input.to_owned())
    }

    pub fn new_number(input: &str) -> Term {
        return Term::Number(input.to_owned())
    }

    pub fn new_equation(input: &str) -> Term {
        return Term::Equation(input.to_owned())
    }

    pub fn latex(&self, dent: usize) -> String {
        to_latex(self.to_string(), dent)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Term::Variable(term) => write!(f, "{}", term),
            Term::Number(term) => write!(f, "{}", term),
            Term::Equation(term) => write!(f, "{}", term),
        }
    }
}



// All types used are accounted for here
// This will allow us to parse a string into a type
pub enum TNT {
    Term(Term),
    Formula(Formula)
}