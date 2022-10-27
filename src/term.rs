use crate::parsing::parser::string_to_term;
use crate::LogicError;
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;
use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
};

lazy_static! {
    pub static ref ZERO: Term = Term::Zero;
    pub static ref ONE: Term = Term::try_from("S0").unwrap();
    pub static ref VARIABLE_NAME: Regex = Regex::new("[a-z]\'*").unwrap();
}

pub fn succ(term: &Term) -> Term {
    Term::Successor(Box::new(term.clone()))
}

pub fn sum(left: &Term, right: &Term) -> Term {
    Term::Sum(Box::new(left.clone()), Box::new(right.clone()))
}

pub fn prod(left: &Term, right: &Term) -> Term {
    Term::Product(Box::new(left.clone()), Box::new(right.clone()))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    Zero,
    Variable(String),
    Successor(Box<Term>),
    Sum(Box<Term>, Box<Term>),
    Product(Box<Term>, Box<Term>),
}

impl Term {
    pub fn zero() -> Term {
        Term::Zero
    }

    pub fn var<S: ToString>(name: S) -> Term {
        if VARIABLE_NAME.is_match(&name.to_string()) {
            Term::Variable(name.to_string())
        } else {
            panic!("invalid Variable name")
        }
    }

    pub fn to_english(&self) -> String {
        match self {
            Self::Zero => "0".into(),
            Self::Variable(v) => v.into(),
            Self::Successor(inner) => format!("S{inner}"),
            Self::Sum(left, right) => format!("({left} + {right})"),
            Self::Product(left, right) => format!("({left} × {right})"),
        }
    }

    pub fn to_latex(&self) -> String {
        match self {
            Self::Zero => "0".into(),
            Self::Variable(v) => v.into(),
            Self::Successor(inner) => format!("S{inner}"),
            Self::Sum(left, right) => format!("({left} + {right})"),
            Self::Product(left, right) => format!("({left} \\cdot {right})"),
        }
    }

    // Determine if a Term contains a Variable with a particular name
    pub fn contains_var<S: ToString>(&self, name: &S) -> bool {
        match self {
            Self::Zero => false,
            Self::Variable(v) => *v == name.to_string(),
            Self::Successor(inner) => inner.contains_var(name),
            Self::Sum(left, right) => left.contains_var(name) || right.contains_var(name),
            Self::Product(left, right) => left.contains_var(name) || right.contains_var(name),
        }
    }

    pub fn get_vars(&self, var_names: &mut HashSet<String>) {
        match self {
            Self::Zero => (),
            Self::Variable(v) => {
                var_names.insert(v.to_string());
            }
            Self::Successor(inner) => inner.get_vars(var_names),
            Self::Sum(left, right) => {
                left.get_vars(var_names);
                right.get_vars(var_names);
            }
            Self::Product(left, right) => {
                left.get_vars(var_names);
                right.get_vars(var_names);
            }
        }
    }

    // Replace a Variable with the provided name with the provided Term
    pub fn replace<S: ToString>(&mut self, name: &S, term: &Term) {
        match self {
            Self::Zero => {}
            Self::Variable(v) => {
                if *v == name.to_string() {
                    *self = term.clone();
                }
            }
            Self::Successor(inner) => inner.replace(name, term),
            Self::Sum(left, right) => {
                left.replace(name, term);
                right.replace(name, term);
            }
            Self::Product(left, right) => {
                left.replace(name, term);
                right.replace(name, term);
            }
        }
    }

    pub fn is_num(&self) -> bool {
        match self {
            Self::Zero => true,
            Self::Successor(inner) => inner.is_num(),
            _ => false,
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::Variable(v) => write!(f, "{}", v),
            Self::Successor(inner) => write!(f, "S{inner}"),
            Self::Sum(left, right) => write!(f, "({left}+{right})"),
            Self::Product(left, right) => write!(f, "({left}*{right})"),
        }
    }
}

impl TryFrom<&str> for Term {
    type Error = LogicError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match string_to_term(value) {
            Ok(f) => Ok(f),
            Err(s) => Err(LogicError(s.to_string())),
        }
    }
}

impl TryFrom<String> for Term {
    type Error = LogicError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match string_to_term(&value) {
            Ok(f) => Ok(f),
            Err(s) => Err(LogicError(s.to_string())),
        }
    }
}
