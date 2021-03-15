use std::fmt;
use num::bigint::BigUint;

use crate::properties::{is_equation,is_num,is_var,is_simple_formula,is_formula};
use crate::translate::{to_latex,to_english,arithmetize};




#[derive(Clone,Debug,PartialEq)]
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

    // Pretty names
    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    pub fn english(&self) -> String {
        to_english(self.to_string())
    }

    pub fn arithmetize(&self) -> BigUint {
        arithmetize(self.to_string())
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





#[derive(Clone,Debug,PartialEq)]
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

    // Pretty names
    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    pub fn english(&self) -> String {
        to_english(self.to_string())
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
#[derive(Clone,Debug,PartialEq)]
pub enum TNT {
    Term(Term),
    Formula(Formula)
}

impl TNT {
    pub fn new(input: &str) -> TNT {
        if is_equation(input) {
            return TNT::Term(Term::new(input))
        } else if is_formula(input) {
            return TNT::Formula(Formula::new(input))
        } else {
            panic!()
        }
    }
}

impl fmt::Display for TNT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TNT::Term(term) => write!(f, "{}", term),
            TNT::Formula(term) => write!(f, "{}", term),
        }
    }
}