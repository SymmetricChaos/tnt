use std::{convert::TryFrom, fmt};
use num::bigint::BigUint;

use crate::properties::{is_term,is_num,is_var,is_simple_formula,is_formula};
use crate::translate::{to_latex,to_english,arithmetize,dearithmetize};




/// A Formula is a well-formed formula, either Simple or Complex
#[derive(Clone,Debug,PartialEq)]
pub enum Formula {
    /// Formula::Simple consists of an equality of two terms
    Simple(String),
    /// Formula::Complex consists of any well-formed formula
    Complex(String),
}

impl Formula {
    /// An &str is automatically converted to the correct variant, this requires potentially slow parsing of the &str
    pub fn new(input: &str) -> Formula {
        if is_simple_formula(input) {
            return Formula::Simple(input.to_owned())
        } else if is_formula(input) {
            return Formula::Complex(input.to_owned()) 
        } else {
            panic!("{} is not a well formed formula",input)
        }
    }

    /// Fast creation of Formula::Simple with no checks
    pub fn new_simple(input: &str) -> Formula {
        return Formula::Simple(input.to_owned())
    }

    /// Fast creation of Formula::Complex with no checks
    pub fn new_complex(input: &str) -> Formula {
        return Formula::Complex(input.to_owned())
    }

    /// Translate the Formula to LaTeX representation
    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    /// Translate the Formula to relatively readable English
    pub fn english(&self) -> String {
        to_english(self.to_string())
    }

    /// Return a BigUint that represents the Formula
    pub fn arithmetize(&self) -> BigUint {
        arithmetize(self.to_string())
    }

    /// Create a formula from a BigUint
    pub fn dearithmetize(number: &BigUint) -> Formula {
        Formula::new(&dearithmetize(number))
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





/// A valid Term consists of either a Variable, a Number, or an Equation. Currently no code cares about the Number and Equation variants
#[derive(Clone,Debug,PartialEq)]
pub enum Term {
    /// A Variable has the regex signature: "[a-z]\'*"
    Variable(String),
    /// A Number has the regex signature: "S*0"
    Number(String),
    /// An Equation is any valid combination of arithmetic relationships between Variables and Numbers. It has no simple regex signature due to the use of parentheses.
    Equation(String),
}

impl Term {
    /// An &str is automatically converted to the correct variant, this requires potentially slow parsing of the &str
    pub fn new(input: &str) -> Term {
        if is_num(input) {
            return Term::Number(input.to_owned())
        } else if is_var(input) {
            return Term::Variable(input.to_owned())
        } else if is_term(input) {
            return Term::Equation(input.to_owned())
        } else {
            panic!("{} is not a valid Term",input)
        }
    }

    /// Fast creation of Term::Variable with no checks
    pub fn new_variable(input: &str) -> Term {
        return Term::Variable(input.to_owned())
    }

    /// Fast creation of Term::Number with no checks
    pub fn new_number(input: &str) -> Term {
        return Term::Number(input.to_owned())
    }

    /// Fast creation of Term::Equation with no checks
    pub fn new_equation(input: &str) -> Term {
        return Term::Equation(input.to_owned())
    }

    /// Translate the Term to LaTeX representation
    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    /// Translate the Term to relatively readable English
    pub fn english(&self) -> String {
        to_english(self.to_string())
    }

    /// Return a BigUint that represents the Term
    pub fn arithmetize(&self) -> BigUint {
        arithmetize(self.to_string())
    }

    /// Create a Term from a BigUint
    pub fn dearithmetize(number: &BigUint) -> Term {
        Term::new(&dearithmetize(number))
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





/// TNT consists of any valid statement either Term or Number
#[derive(Clone,Debug,PartialEq)]
pub enum TNT {
    Term(Term),
    Formula(Formula)
}

impl TNT {
    pub fn new(input: &str) -> TNT {
        if is_term(input) {
            return TNT::Term(Term::new(input))
        } else if is_formula(input) {
            return TNT::Formula(Formula::new(input))
        } else {
            panic!()
        }
    }

    /// Translate the TNT to LaTeX representation
    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    /// Translate the TNT to relatively readable English
    pub fn english(&self) -> String {
        to_english(self.to_string())
    }
}

impl TryFrom<Term> for TNT {
    type Error = &'static str;

    fn try_from(value: Term) -> Result<Self, Self::Error> {
        Ok(TNT::Term(value))
    }
}

impl TryFrom<Formula> for TNT {
    type Error = &'static str;

    fn try_from(value: Formula) -> Result<Self, Self::Error> {
        Ok(TNT::Formula(value))
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