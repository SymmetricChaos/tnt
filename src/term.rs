use crate::parsing::parser::string_to_term;
use crate::LogicError;
use lazy_static::lazy_static;
use num::BigUint;
use regex::Regex;
use std::convert::TryFrom;
use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
    str::from_utf8,
};

lazy_static! {
    pub static ref ZERO: Term = Term::Zero;
    pub static ref ONE: Term = Term::try_from("S0").unwrap();
    pub static ref VARIABLE_NAME: Regex = Regex::new("[a-z]\'*").unwrap();
}

pub fn succ(term: &Term) -> Term {
    Term::Successor(Box::new(term.clone()))
}

pub fn sum(lhs: &Term, rhs: &Term) -> Term {
    Term::Sum(Box::new(lhs.clone()), Box::new(rhs.clone()))
}

pub fn prod(lhs: &Term, rhs: &Term) -> Term {
    Term::Product(Box::new(lhs.clone()), Box::new(rhs.clone()))
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

    pub fn pretty_string(&self) -> String {
        match self {
            Self::Zero => "0".into(),
            Self::Variable(v) => v.into(),
            Self::Successor(inner) => format!("S{inner}"),
            Self::Sum(lhs, rhs) => format!("({lhs} + {rhs})"),
            Self::Product(lhs, rhs) => format!("({lhs} × {rhs})"),
        }
    }

    pub fn to_latex(&self) -> String {
        match self {
            Self::Zero => "0".into(),
            Self::Variable(v) => v.into(),
            Self::Successor(inner) => format!("S{inner}"),
            Self::Sum(lhs, rhs) => format!("({lhs} + {rhs})"),
            Self::Product(lhs, rhs) => format!("({lhs} \\cdot {rhs})"),
        }
    }

    // Determine if a Term contains a Variable with a particular name
    pub fn contains_var<S: ToString>(&self, name: &S) -> bool {
        match self {
            Self::Zero => false,
            Self::Variable(v) => *v == name.to_string(),
            Self::Successor(inner) => inner.contains_var(name),
            Self::Sum(lhs, rhs) => lhs.contains_var(name) || rhs.contains_var(name),
            Self::Product(lhs, rhs) => lhs.contains_var(name) || rhs.contains_var(name),
        }
    }

    pub fn get_vars(&self, var_names: &mut HashSet<String>) {
        match self {
            Self::Zero => (),
            Self::Variable(v) => {
                var_names.insert(v.to_string());
            }
            Self::Successor(inner) => inner.get_vars(var_names),
            Self::Sum(lhs, rhs) => {
                lhs.get_vars(var_names);
                rhs.get_vars(var_names);
            }
            Self::Product(lhs, rhs) => {
                lhs.get_vars(var_names);
                rhs.get_vars(var_names);
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
            Self::Sum(lhs, rhs) => {
                lhs.replace(name, term);
                rhs.replace(name, term);
            }
            Self::Product(lhs, rhs) => {
                lhs.replace(name, term);
                rhs.replace(name, term);
            }
        }
    }

    // rename a variable without checking for correct form, used in .to_austere().
    pub(crate) fn rename_var<S: ToString>(&mut self, name: &S, new_name: &S) {
        match self {
            Self::Zero => {}
            Self::Variable(v) => {
                if *v == name.to_string() {
                    *self = Term::Variable(new_name.to_string());
                }
            }
            Self::Successor(inner) => inner.rename_var(name, new_name),
            Self::Sum(lhs, rhs) => {
                lhs.rename_var(name, new_name);
                rhs.rename_var(name, new_name);
            }
            Self::Product(lhs, rhs) => {
                lhs.rename_var(name, new_name);
                rhs.rename_var(name, new_name);
            }
        }
    }

    // pub fn is_num(&self) -> bool {
    //     match self {
    //         Self::Zero => true,
    //         Self::Successor(inner) => inner.is_num(),
    //         _ => false,
    //     }
    // }

    pub fn vars_in_order(&self, vec: &mut Vec<String>) {
        match self {
            Self::Zero => (),
            Self::Variable(v) => {
                if !vec.contains(v) {
                    vec.push(v.to_string());
                }
            }
            Self::Successor(inner) => inner.vars_in_order(vec),
            Self::Sum(lhs, rhs) => {
                lhs.vars_in_order(vec);
                rhs.vars_in_order(vec);
            }
            Self::Product(lhs, rhs) => {
                lhs.vars_in_order(vec);
                rhs.vars_in_order(vec);
            }
        }
    }

    /// Produces the Term in its austere form. The lhsmost variable is renamed `a` in all appearances, the next is renamed `a'` and so on.
    pub fn to_austere(&self) -> Term {
        let mut t = self.clone();
        let vars = {
            let mut v = Vec::new();
            t.vars_in_order(&mut v);
            v
        };
        let mut mask = String::from("#");
        for v in vars.iter() {
            t.replace(v, &Term::Variable(mask.clone()));
            mask.push('\'');
        }
        let mut mask = String::from("#");
        let mut a = String::from("a");
        for _ in vars.iter() {
            t.replace(&mask, &Term::Variable(a.clone()));
            mask.push('\'');
            a.push('\'');
        }
        t
    }

    /// Create the unique BigUint that characterizes the Term. This is done by converting the Term to its austere form and then reading the bytes as a bigendian number.
    pub fn arithmetize(&self) -> BigUint {
        let s = self.clone().to_austere().to_string();
        BigUint::from_bytes_be(s.as_bytes())
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::Variable(v) => write!(f, "{}", v),
            Self::Successor(inner) => write!(f, "S{inner}"),
            Self::Sum(lhs, rhs) => write!(f, "({lhs}+{rhs})"),
            Self::Product(lhs, rhs) => write!(f, "({lhs}*{rhs})"),
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

impl TryFrom<BigUint> for Term {
    type Error = LogicError;

    fn try_from(value: BigUint) -> Result<Self, Self::Error> {
        match from_utf8(&value.to_bytes_be()) {
            Ok(s) => Term::try_from(s),
            Err(e) => Err(LogicError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn austere() {
        let t0 = Term::try_from("((j+(a''+SS0))+(a''*SSc))")
            .unwrap()
            .to_austere();
        let t1 = Term::try_from("((a+(a'+SS0))+(a'*SSa''))").unwrap();
        assert_eq!(t0, t1);
    }

    #[test]
    fn replace() {
        let mut t0 = Term::try_from("((j+(a''+SS0))+(a''*SSc))").unwrap();
        let term = Term::try_from("(a+a)").unwrap();
        let t1 = Term::try_from("((j+((a+a)+SS0))+((a+a)*SSc))").unwrap();
        t0.replace(&"a''".to_string(), &term);
        assert_eq!(t0, t1);
    }

    #[test]
    fn from_big_uint() {
        let t0 = Term::try_from("SS(a+a')").unwrap();
        let t1 = Term::try_from(t0.arithmetize()).unwrap();
        assert_eq!(t0, t1);
    }
}
