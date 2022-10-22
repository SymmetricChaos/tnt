use crate::parsing::parser::{string_to_formula, Rule};
use crate::Term;
use lazy_static::lazy_static;
use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
    ops::{BitAnd, BitOr},
};

lazy_static! {
    /**
    * These are the axiomatic statements of the TNT formal system, they don't align strictly with the Peano Axioms but they define the same arithmetic properties for addition and multiplication. The axioms are as follows:
    *
    * Aa:~Sa=0                  for all a, it is false that (a + 1) is 0
    *
    * Aa:(a+0)=a                for all a, (a + 0) = a
    *
    * Aa:Ab:(a+Sb)=S(a+b)       for all a and b, (a + (b + 1)) = ((a + b) + 1)
    *
    * Aa:(a\*0)=0             for all a, (a × 0) = 0
    *
    * Aa:Ab:(a\*Sb)=((a\*b)+a)  for all a and b, (a × (b + 1)) = ((a × b) + a)
    */

    pub static ref PEANO: Vec<Formula> = {


        let axioms = vec![
            Formula::from_string("Aa:~Sa=0").unwrap(),
            Formula::from_string("Aa:(a+0)=a").unwrap(),
            Formula::from_string("Aa:Ab:(a+Sb)=S(a+b)").unwrap(),
            Formula::from_string("Aa:(a*0)=0").unwrap(),
            Formula::from_string("Aa:Ab:(a*Sb)=((a*b)+a)").unwrap(),
        ];

        axioms
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Formula {
    Equality(Term, Term),
    Universal(String, Box<Formula>),
    Existential(String, Box<Formula>),
    Negation(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
}

impl Formula {
    pub fn from_string(s: &str) -> Result<Formula, pest::error::Error<Rule>> {
        string_to_formula(s)
    }

    // Eliminate all universal quantification of some Variable and then replace all instances of that variable with the provided Term
    pub fn specify(&mut self, name: &str, term: &Term) {
        match self {
            Self::Equality(left, right) => {
                left.replace(name, term);
                right.replace(name, term);
            }
            Self::Universal(v, formula) => {
                if *v == name {
                    *self = *formula.clone();
                    self.specify(name, term);
                }
            }
            Self::Existential(_, formula) => formula.specify(name, term),
            Self::Negation(formula) => formula.specify(name, term),
            Self::And(left, right) => {
                left.specify(name, term);
                right.specify(name, term);
            }
            Self::Or(left, right) => {
                left.specify(name, term);
                right.specify(name, term);
            }
            Self::Implies(left, right) => {
                left.specify(name, term);
                right.specify(name, term);
            }
        }
    }

    pub fn get_vars(&self, var_names: &mut HashSet<String>) {
        match self {
            Self::Equality(left, right) => {
                left.get_vars(var_names);
                right.get_vars(var_names);
            }
            Self::Universal(v, formula) => {
                var_names.insert(v.to_string());
                formula.get_vars(var_names)
            }
            Self::Existential(v, formula) => {
                var_names.insert(v.to_string());
                formula.get_vars(var_names)
            }
            Self::Negation(formula) => formula.get_vars(var_names),
            Self::And(left, right) => {
                left.get_vars(var_names);
                right.get_vars(var_names)
            }
            Self::Or(left, right) => {
                left.get_vars(var_names);
                right.get_vars(var_names)
            }
            Self::Implies(left, right) => {
                left.get_vars(var_names);
                right.get_vars(var_names)
            }
        }
    }

    pub fn get_vars_bound(&self, var_names: &mut HashSet<String>) {
        match self {
            Self::Equality(_, _) => (),
            Self::Universal(v, formula) => {
                var_names.insert(v.to_string());
                formula.get_vars(var_names);
            }
            Self::Existential(v, formula) => {
                var_names.insert(v.to_string());
                formula.get_vars(var_names);
            }
            Self::Negation(formula) => formula.get_vars(var_names),
            Self::And(left, right) => {
                left.get_vars(var_names);
                right.get_vars(var_names);
            }
            Self::Or(left, right) => {
                left.get_vars(var_names);
                right.get_vars(var_names);
            }
            Self::Implies(left, right) => {
                left.get_vars(var_names);
                right.get_vars(var_names);
            }
        }
    }

    pub fn contains_var(&self, name: &str) -> bool {
        match self {
            Self::Equality(left, right) => left.contains_var(name) || right.contains_var(name),
            Self::Universal(v, formula) => *v == name || formula.contains_var(name),
            Self::Existential(v, formula) => *v == name || formula.contains_var(name),
            Self::Negation(formula) => formula.contains_var(name),
            Self::And(left, right) => left.contains_var(name) || right.contains_var(name),
            Self::Or(left, right) => left.contains_var(name) || right.contains_var(name),
            Self::Implies(left, right) => left.contains_var(name) || right.contains_var(name),
        }
    }

    pub fn contains_var_bound(&self, name: &str) -> bool {
        match self {
            Self::Equality(_, _) => false,
            Self::Universal(v, formula) => *v == name || formula.contains_var_bound(name),
            Self::Existential(v, formula) => *v == name || formula.contains_var_bound(name),
            Self::Negation(formula) => formula.contains_var_bound(name),
            Self::And(left, right) => {
                left.contains_var_bound(name) || right.contains_var_bound(name)
            }
            Self::Or(left, right) => {
                left.contains_var_bound(name) || right.contains_var_bound(name)
            }
            Self::Implies(left, right) => {
                left.contains_var_bound(name) || right.contains_var_bound(name)
            }
        }
    }

    pub fn contains_var_bound_universal(&self, name: &str) -> bool {
        match self {
            Self::Equality(_, _) => false,
            Self::Universal(v, formula) => *v == name || formula.contains_var_bound_universal(name),
            Self::Existential(_, formula) => formula.contains_var_bound_universal(name),
            Self::Negation(formula) => formula.contains_var_bound_universal(name),
            Self::And(left, right) => {
                left.contains_var_bound_universal(name) || right.contains_var_bound_universal(name)
            }
            Self::Or(left, right) => {
                left.contains_var_bound_universal(name) || right.contains_var_bound_universal(name)
            }
            Self::Implies(left, right) => {
                left.contains_var_bound_universal(name) || right.contains_var_bound_universal(name)
            }
        }
    }

    pub fn contains_var_bound_existential(&self, name: &str) -> bool {
        match self {
            Self::Equality(_, _) => false,
            Self::Universal(_, formula) => formula.contains_var_bound_existential(name),
            Self::Existential(v, formula) => {
                *v == name || formula.contains_var_bound_existential(name)
            }
            Self::Negation(formula) => formula.contains_var_bound_existential(name),
            Self::And(left, right) => {
                left.contains_var_bound_existential(name)
                    || right.contains_var_bound_existential(name)
            }
            Self::Or(left, right) => {
                left.contains_var_bound_existential(name)
                    || right.contains_var_bound_existential(name)
            }
            Self::Implies(left, right) => {
                left.contains_var_bound_existential(name)
                    || right.contains_var_bound_existential(name)
            }
        }
    }
}

impl Display for Formula {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Equality(left, right) => write!(f, "{left}={right}"),
            Self::Universal(name, formula) => write!(f, "A{name}:{formula}"),
            Self::Existential(name, formula) => write!(f, "E{name}:{formula}"),
            Self::Negation(formula) => write!(f, "~{formula}"),
            Self::And(left, right) => write!(f, "[{left}&{right}]"),
            Self::Or(left, right) => write!(f, "[{left}|{right}]"),
            Self::Implies(left, right) => write!(f, "[{left}>{right}]"),
        }
    }
}

// These are guaranteed to produce well-formed formulas of TNT. However they may produce false statements.

/// Equality of two Terms
pub fn eq(left: &Term, right: &Term) -> Formula {
    Formula::Equality(left.clone(), right.clone())
}

/// Negation of a Formula
pub fn not(formula: &Formula) -> Formula {
    Formula::Negation(Box::new(formula.clone()))
}

/// Logical OR of two Formulas
pub fn or(left: &Formula, right: &Formula) -> Formula {
    Formula::Or(Box::new(left.clone()), Box::new(right.clone()))
}

/// Logical AND of two Formulas
pub fn and(left: &Formula, right: &Formula) -> Formula {
    Formula::And(Box::new(left.clone()), Box::new(right.clone()))
}

/// Left Formula implies right Formula
pub fn implies(left: &Formula, right: &Formula) -> Formula {
    Formula::Implies(Box::new(left.clone()), Box::new(right.clone()))
}

/// Assert some values for a Variable with the given name makes the Forumla true
pub fn exists<S: ToString>(var_name: S, formula: &Formula) -> Formula {
    Formula::Existential(var_name.to_string(), Box::new(formula.clone()))
}

/// Assert that all values of a Variable with the given name make the Formula true
pub fn forall<S: ToString>(var_name: S, formula: &Formula) -> Formula {
    Formula::Universal(var_name.to_string(), Box::new(formula.clone()))
}

impl<'a, 'b> BitAnd<&'b Formula> for &'a Formula {
    type Output = Formula;

    fn bitand(self, other: &'b Formula) -> Formula {
        and(self, other)
    }
}

impl<'a, 'b> BitOr<&'b Formula> for &'a Formula {
    type Output = Formula;

    fn bitor(self, other: &'b Formula) -> Formula {
        or(self, other)
    }
}
