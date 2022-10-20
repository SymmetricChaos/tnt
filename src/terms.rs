use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
    ops::{Add, Mul},
};

use regex::Regex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ZERO: Term = Term::Zero;
    pub static ref ONE: Term = succ(&Term::Zero);
}

pub fn succ(term: &Term) -> Term {
    Term::Succ(Box::new(term.clone()))
}

pub fn sum(left: &Term, right: &Term) -> Term {
    Term::Sum(Box::new(left.clone()), Box::new(right.clone()))
}

pub fn prod(left: &Term, right: &Term) -> Term {
    Term::Prod(Box::new(left.clone()), Box::new(right.clone()))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    Zero,
    Var(&'static str),
    Succ(Box<Term>),
    Sum(Box<Term>, Box<Term>),
    Prod(Box<Term>, Box<Term>),
}

impl Term {
    pub fn zero() -> Term {
        Term::Zero
    }

    pub fn var(name: &'static str) -> Term {
        let re = Regex::new(r"^[a-z]'*$").unwrap();
        if re.is_match(name) {
            Term::Var(name)
        } else {
            panic!("invalid Variable name")
        }
    }

    // Determine if a Term contains a Variable with a particular name
    pub fn contains_var(&self, name: &str) -> bool {
        match self {
            Self::Zero => false,
            Self::Var(v) => *v == name,
            Self::Succ(inner) => inner.contains_var(name),
            Self::Sum(left, right) => left.contains_var(name) || right.contains_var(name),
            Self::Prod(left, right) => left.contains_var(name) || right.contains_var(name),
        }
    }

    pub fn get_vars(&self, var_names: &mut HashSet<&'static str>) {
        match self {
            Self::Zero => (),
            Self::Var(v) => {
                var_names.insert(v);
            }
            Self::Succ(inner) => inner.get_vars(var_names),
            Self::Sum(left, right) => {
                left.get_vars(var_names);
                right.get_vars(var_names);
            }
            Self::Prod(left, right) => {
                left.get_vars(var_names);
                right.get_vars(var_names);
            }
        }
    }

    // Replace a Variable with the provided name with the provided Term
    pub fn replace(&mut self, name: &str, term: &Term) {
        match self {
            Self::Zero => {}
            Self::Var(v) => {
                if *v == name {
                    *self = term.clone();
                }
            }
            Self::Succ(inner) => inner.replace(name, term),
            Self::Sum(left, right) => {
                left.replace(name, term);
                right.replace(name, term);
            }
            Self::Prod(left, right) => {
                left.replace(name, term);
                right.replace(name, term);
            }
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::Var(v) => write!(f, "{}", v),
            Self::Succ(inner) => write!(f, "S{inner}"),
            Self::Sum(left, right) => write!(f, "({left}+{right})"),
            Self::Prod(left, right) => write!(f, "({left}*{right})"),
        }
    }
}

impl Add<Term> for Term {
    type Output = Term;

    fn add(self, other: Term) -> Term {
        sum(&self, &other)
    }
}

impl Mul<Term> for Term {
    type Output = Term;

    fn mul(self, other: Term) -> Term {
        prod(&self, &other)
    }
}

impl<'a, 'b> Add<&'b Term> for &'a Term {
    type Output = &'a Term;

    fn add(self, other: &'b Term) -> &'a Term {
        &sum(self, other)
    }
}

impl<'a, 'b> Mul<&'b Term> for &'a Term {
    type Output = &'a Term;

    fn mul(self, other: &'b Term) -> &'a Term {
        &prod(self, other)
    }
}
