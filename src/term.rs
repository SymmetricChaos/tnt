use crate::parsing::parser::string_to_term;
use crate::LogicError;
use indexmap::IndexSet;
use lazy_static::lazy_static;
use num::BigUint;
use regex::Regex;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;
use std::{
    fmt::{self, Display, Formatter},
    str::from_utf8,
};

lazy_static! {
    pub static ref VARIABLE_NAME: Regex = Regex::new("[a-z]\'*").unwrap();
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    Zero,
    Variable(String),
    Successor(Rc<RefCell<Term>>),
    Sum(Rc<RefCell<Term>>, Rc<RefCell<Term>>),
    Product(Rc<RefCell<Term>>, Rc<RefCell<Term>>),
}

impl Term {
    pub fn one() -> Term {
        Self::succ(&Self::Zero)
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
            Self::Successor(inner) => format!("S{}", inner.borrow()),
            Self::Sum(lhs, rhs) => format!("({} + {})", lhs.borrow(), rhs.borrow()),
            Self::Product(lhs, rhs) => format!("({} × {})", lhs.borrow(), rhs.borrow()),
        }
    }

    pub fn to_latex(&self) -> String {
        match self {
            Self::Zero => "0".into(),
            Self::Variable(v) => v.into(),
            Self::Successor(inner) => format!("S{}", inner.borrow()),
            Self::Sum(lhs, rhs) => format!("({} + {})", lhs.borrow(), rhs.borrow()),
            Self::Product(lhs, rhs) => {
                format!("({} \\cdot {})", lhs.borrow(), rhs.borrow())
            }
        }
    }

    // Determine if a Term contains a Variable with a particular name
    pub fn contains_var<S: ToString>(&self, name: &S) -> bool {
        match self {
            Self::Zero => false,
            Self::Variable(v) => *v == name.to_string(),
            Self::Successor(inner) => inner.borrow().contains_var(name),
            Self::Sum(lhs, rhs) => {
                lhs.borrow().contains_var(name) || rhs.borrow().contains_var(name)
            }
            Self::Product(lhs, rhs) => {
                lhs.borrow().contains_var(name) || rhs.borrow().contains_var(name)
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
            Self::Successor(inner) => inner.borrow_mut().replace(name, term),
            Self::Sum(lhs, rhs) => {
                lhs.borrow_mut().replace(name, term);
                rhs.borrow_mut().replace(name, term);
            }
            Self::Product(lhs, rhs) => {
                lhs.borrow_mut().replace(name, term);
                rhs.borrow_mut().replace(name, term);
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
            Self::Successor(inner) => inner.borrow_mut().rename_var(name, new_name),
            Self::Sum(lhs, rhs) => {
                lhs.borrow_mut().rename_var(name, new_name);
                rhs.borrow_mut().rename_var(name, new_name);
            }
            Self::Product(lhs, rhs) => {
                lhs.borrow_mut().rename_var(name, new_name);
                rhs.borrow_mut().rename_var(name, new_name);
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

    pub fn get_vars(&self, set: &mut IndexSet<String>) {
        match self {
            Self::Zero => (),
            Self::Variable(v) => {
                if !set.contains(v) {
                    set.insert(v.to_string());
                }
            }
            Self::Successor(inner) => inner.borrow().get_vars(set),
            Self::Sum(lhs, rhs) => {
                lhs.borrow().get_vars(set);
                rhs.borrow().get_vars(set);
            }
            Self::Product(lhs, rhs) => {
                lhs.borrow().get_vars(set);
                rhs.borrow().get_vars(set);
            }
        }
    }

    /// Produces the Term in its austere form. The lhsmost variable is renamed `a` in all appearances, the next is renamed `a'` and so on.
    pub fn austere(&self) -> Term {
        let mut out = self.clone();
        out.to_austere();
        out
    }

    pub fn to_austere(&mut self) {
        let vars = {
            let mut v = IndexSet::new();
            self.get_vars(&mut v);
            v
        };
        self.to_austere_with(&vars);
    }

    pub(crate) fn to_austere_with(&mut self, vars: &IndexSet<String>) {
        let mut mask = String::from("#");
        for v in vars.iter() {
            self.rename_var(v, &mask);
            mask.push('\'');
        }
        let mut mask = String::from("#");
        let mut a = String::from("a");
        for _ in vars.iter() {
            self.rename_var(&mask, &a);
            mask.push('\'');
            a.push('\'');
        }
    }

    /// Create the unique BigUint that characterizes the Term. This is done by converting the Term to its austere form and then reading the bytes as a bigendian number.
    pub fn arithmetize(&self) -> BigUint {
        let s = self.clone().austere().to_string();
        BigUint::from_bytes_be(s.as_bytes())
    }

    pub fn succ(term: &Term) -> Term {
        Term::Successor(Rc::new(RefCell::new(term.clone())))
    }

    pub fn sum(lhs: &Term, rhs: &Term) -> Term {
        Term::Sum(
            Rc::new(RefCell::new(lhs.clone())),
            Rc::new(RefCell::new(rhs.clone())),
        )
    }

    pub fn prod(lhs: &Term, rhs: &Term) -> Term {
        Term::Product(
            Rc::new(RefCell::new(lhs.clone())),
            Rc::new(RefCell::new(rhs.clone())),
        )
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::Variable(v) => write!(f, "{}", v),
            Self::Successor(inner) => write!(f, "S{}", inner.borrow()),
            Self::Sum(lhs, rhs) => write!(f, "({}+{})", lhs.borrow(), rhs.borrow()),
            Self::Product(lhs, rhs) => write!(f, "({}*{})", lhs.borrow(), rhs.borrow()),
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
            .austere();
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
