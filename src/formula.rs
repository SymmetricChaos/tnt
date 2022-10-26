use crate::parsing::parser::string_to_formula;
use crate::term::VARIABLE_NAME;
use crate::{LogicError, Term};
use lazy_static::lazy_static;
use std::{
    collections::HashSet,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
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
            Formula::try_from("Aa:~Sa=0").unwrap(),
            Formula::try_from("Aa:(a+0)=a").unwrap(),
            Formula::try_from("Aa:Ab:(a+Sb)=S(a+b)").unwrap(),
            Formula::try_from("Aa:(a*0)=0").unwrap(),
            Formula::try_from("Aa:Ab:(a*Sb)=((a*b)+a)").unwrap(),
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
    pub fn to_english(&self) -> String {
        match self {
            Formula::Equality(l, r) => format!("{l} equals {r}"),
            Formula::Universal(var, inner) => format!("for all {var} it is the case that {inner}"),
            Formula::Existential(var, inner) => format!("there exists {var} such that {inner}"),
            Formula::Negation(inner) => format!("it is false that {inner}"),
            Formula::And(l, r) => format!("[{l} and {r}]"),
            Formula::Or(l, r) => format!("[{l} or {r}]"),
            Formula::Implies(l, r) => format!("[{l} implies {r}]"),
        }
    }

    pub fn to_latex(&self) -> String {
        match self {
            Formula::Equality(l, r) => format!("{} = {}", l.to_latex(), r.to_latex()),
            Formula::Universal(var, inner) => format!("\\forall {var}: {inner}"),
            Formula::Existential(var, inner) => format!("\\exists {var}: {inner}"),
            Formula::Negation(inner) => format!("\\neg {inner}"),
            Formula::And(l, r) => format!("\\langle {l} \\wedge {r} \\rangle"),
            Formula::Or(l, r) => format!("\\langle {l} \\vee {r} \\rangle"),
            Formula::Implies(l, r) => format!(" \\langle{l} \\Rightarrow {r} \\rangle"),
        }
    }

    // Eliminate all universal quantification of some Variable and then replace all instances of that variable with the provided Term
    pub fn specify<S: ToString>(&mut self, name: &S, term: &Term) {
        match self {
            Self::Equality(left, right) => {
                left.replace(name, term);
                right.replace(name, term);
            }
            Self::Universal(v, formula) => {
                if *v == name.to_string() {
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

    /// Replace all free instances of the named Term::Variable with a Term
    pub fn replace_free<S: ToString>(&mut self, name: &S, term: &Term) {
        match self {
            Self::Equality(left, right) => {
                left.replace(name, term);
                right.replace(name, term);
            }
            Self::Universal(_, formula) => formula.replace_free(name, term),
            Self::Existential(_, formula) => formula.replace_free(name, term),
            Self::Negation(formula) => formula.replace_free(name, term),
            Self::And(left, right) => {
                left.replace_free(name, term);
                right.replace_free(name, term);
            }
            Self::Or(left, right) => {
                left.replace_free(name, term);
                right.replace_free(name, term);
            }
            Self::Implies(left, right) => {
                left.replace_free(name, term);
                right.replace_free(name, term);
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

    pub fn get_vars_free(&self, var_names: &mut HashSet<String>) {
        let mut all_v = HashSet::<String>::new();
        let mut bound_v = HashSet::<String>::new();
        self.get_vars(&mut all_v);
        self.get_vars(&mut bound_v);
        for free_v in all_v.difference(&bound_v) {
            var_names.insert(free_v.clone());
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

    pub fn contains_var<S: ToString>(&self, name: &S) -> bool {
        match self {
            Self::Equality(left, right) => left.contains_var(name) || right.contains_var(name),
            Self::Universal(v, formula) => *v == name.to_string() || formula.contains_var(name),
            Self::Existential(v, formula) => *v == name.to_string() || formula.contains_var(name),
            Self::Negation(formula) => formula.contains_var(name),
            Self::And(left, right) => left.contains_var(name) || right.contains_var(name),
            Self::Or(left, right) => left.contains_var(name) || right.contains_var(name),
            Self::Implies(left, right) => left.contains_var(name) || right.contains_var(name),
        }
    }

    pub fn contains_var_bound<S: ToString>(&self, name: &S) -> bool {
        match self {
            Self::Equality(_, _) => false,
            Self::Universal(v, formula) => {
                *v == name.to_string() || formula.contains_var_bound(name)
            }
            Self::Existential(v, formula) => {
                *v == name.to_string() || formula.contains_var_bound(name)
            }
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

    pub fn contains_var_bound_universal<S: ToString>(&self, name: &S) -> bool {
        match self {
            Self::Equality(_, _) => false,
            Self::Universal(v, formula) => {
                *v == name.to_string() || formula.contains_var_bound_universal(name)
            }
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

    pub fn contains_var_bound_existential<S: ToString>(&self, name: &S) -> bool {
        match self {
            Self::Equality(_, _) => false,
            Self::Universal(_, formula) => formula.contains_var_bound_existential(name),
            Self::Existential(v, formula) => {
                *v == name.to_string() || formula.contains_var_bound_existential(name)
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

    pub fn matches_variant(&self, formula: &Formula) -> bool {
        match (self, formula) {
            (Formula::Equality(_, _), Formula::Equality(_, _)) => true,
            (Formula::Universal(_, _), Formula::Universal(_, _)) => true,
            (Formula::Existential(_, _), Formula::Existential(_, _)) => true,
            (Formula::Negation(_), Formula::Negation(_)) => true,
            (Formula::And(_, _), Formula::And(_, _)) => true,
            (Formula::Or(_, _), Formula::Or(_, _)) => true,
            (Formula::Implies(_, _), Formula::Implies(_, _)) => true,
            _ => false,
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

/// Assert some values for a Variable with the given name makes the Formula true
pub fn exists<S: ToString>(var_name: S, formula: &Formula) -> Formula {
    if VARIABLE_NAME.is_match(&var_name.to_string()) {
        Formula::Existential(var_name.to_string(), Box::new(formula.clone()))
    } else {
        panic!("invalid Variable name")
    }
}

/// Assert that all values of a Variable with the given name make the Formula true
pub fn forall<S: ToString>(var_name: S, formula: &Formula) -> Formula {
    if VARIABLE_NAME.is_match(&var_name.to_string()) {
        Formula::Universal(var_name.to_string(), Box::new(formula.clone()))
    } else {
        panic!("invalid Variable name")
    }
}

impl TryFrom<&str> for Formula {
    type Error = LogicError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match string_to_formula(value) {
            Ok(f) => Ok(f),
            Err(s) => Err(LogicError(s.to_string())),
        }
    }
}

impl TryFrom<String> for Formula {
    type Error = LogicError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match string_to_formula(&value) {
            Ok(f) => Ok(f),
            Err(s) => Err(LogicError(s.to_string())),
        }
    }
}
