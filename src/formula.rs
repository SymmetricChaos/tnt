use std::fmt::{self, Display, Formatter};

use crate::terms::{succ, Term, ONE, ZERO};

use lazy_static::lazy_static;
lazy_static! {
    /**
    * These are the axiomatic statements of the TNT formal system, they don't align strictly with the Peano Axioms but they define the same arithmetic properties for addition and multiplication. The axioms are as follows:
    *
    * Aa:~Sa=0                for all a, it is false that (a + 1) is 0
    *
    * Aa:(a+0)=a              for all a, (a + 0) = a
    *
    * Aa:Ab:(a+Sb)=S(a+b)     for all a and b, (a + (b + 1)) = ((a + b) + 1)
    *
    * Aa:(a\*0) = 0            for all a, (a × 0) = 0
    *
    * Aa:Ab:(a\*Sb)=((a\*b)+a)  for all a and b, (a × (b + 1)) = ((a × b) + a)
    */

    pub static ref PEANO: Vec<Formula> = {

        let a = &Term::var("a");
        let b = &Term::var("b");

        let mut m = Vec::new();
        m.push(
            forall("a",&not(&eq(&succ(a),ZERO)))
        );

        m.push(
            forall("a",eq(a + ZERO, a))
        );

        m.push(
            forall("a",forall("b", &eq(a + &succ(b), &succ(a + b))))
        );

        m.push(
            forall("a",eq(a * ZERO, ZERO))
        );

        m.push(
            forall("a",forall("b",
                &eq(
                    &(a * &(b + ONE)),
                    &((a * b) + a))
                ))
        );

        m
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Formula {
    Equality(Term, Term),
    Universal(&'static str, Box<Formula>),
    Existential(&'static str, Box<Formula>),
    Negation(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
}

impl Formula {
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

    pub fn contains_var(&mut self, name: &str) -> bool {
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

    pub fn contains_var_bound(&mut self, name: &str) -> bool {
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
pub fn exists(v: &'static str, formula: &Formula) -> Formula {
    Formula::Existential(v, Box::new(formula.clone()))
}

/// Assert that all values of a Variable with the given name make the Formula true
pub fn forall(v: &'static str, formula: &Formula) -> Formula {
    Formula::Universal(v, Box::new(formula.clone()))
}
