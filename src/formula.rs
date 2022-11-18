use crate::parsing::parser::string_to_formula;
use crate::term::VARIABLE_NAME;
use crate::{LogicError, Term};
use indexmap::IndexSet;
use num::BigUint;
use std::str::from_utf8;
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

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
    /// Return a String explaining the Formula in English.
    pub fn to_english(&self) -> String {
        match self {
            Self::Equality(l, r) => format!("{} = {}", l.pretty_string(), r.pretty_string()),
            Self::Universal(var, inner) => format!("for all {var}, {}", inner.to_english()),
            Self::Existential(var, inner) => {
                format!("there exists {var} such that {}", inner.to_english())
            }
            Self::Negation(inner) => format!("it is false that {}", inner.to_english()),
            Self::And(l, r) => format!("[{} and {}]", l.to_english(), r.to_english()),
            Self::Or(l, r) => format!("[{} or {}]", l.to_english(), r.to_english()),
            Self::Implies(l, r) => format!("[{} implies {}]", l.to_english(), r.to_english()),
        }
    }

    /// Display the Formula with Unicode symbols and spaces.
    pub fn pretty_string(&self) -> String {
        match self {
            Self::Equality(lhs, rhs) => {
                format!("{} = {}", lhs.pretty_string(), rhs.pretty_string())
            }
            Self::Universal(name, formula) => format!("∀{name}:{}", formula.pretty_string()),
            Self::Existential(name, formula) => format!("∃{name}:{}", formula.pretty_string()),
            Self::Negation(formula) => format!("¬{}", formula.pretty_string()),
            Self::And(lhs, rhs) => format!("[{} ∧ {}]", lhs.pretty_string(), rhs.pretty_string()),
            Self::Or(lhs, rhs) => format!("[{} ∨ {}]", lhs.pretty_string(), rhs.pretty_string()),
            Self::Implies(lhs, rhs) => {
                format!("[{} ⇒ {}]", lhs.pretty_string(), rhs.pretty_string())
            }
        }
    }

    /// Return a String formatting the Formula in LaTeX with Hofstadter's original notation.
    pub fn to_latex(&self) -> String {
        match self {
            Self::Equality(l, r) => format!("{} = {}", l.to_latex(), r.to_latex()),
            Self::Universal(var, inner) => format!("\\forall {var}: {}", inner.to_latex()),
            Self::Existential(var, inner) => format!("\\exists {var}: {}", inner.to_latex()),
            Self::Negation(inner) => format!("\\neg {}", inner.to_latex()),
            Self::And(l, r) => format!(
                "\\langle {} \\wedge {} \\rangle",
                l.to_latex(),
                r.to_latex()
            ),
            Self::Or(l, r) => format!("\\langle {} \\vee {} \\rangle", l.to_latex(), r.to_latex()),
            Self::Implies(l, r) => format!(
                " \\langle{} \\rhsarrow {} \\rangle",
                l.to_latex(),
                r.to_latex()
            ),
        }
    }

    /// Eliminate all universal quantification of some Variable and then replace all instances of that variable with the provided Term
    pub fn specify<S: ToString>(&mut self, var_name: &S, term: &Term) {
        match self {
            Self::Equality(lhs, rhs) => {
                lhs.replace(var_name, term);
                rhs.replace(var_name, term);
            }
            Self::Universal(v, formula) => {
                if *v == var_name.to_string() {
                    let mut new_formula = formula.clone();
                    new_formula.specify(var_name, term);
                    *self = *new_formula;
                } else {
                    formula.specify(var_name, term)
                }
            }
            Self::Existential(_, formula) => formula.specify(var_name, term),
            Self::Negation(formula) => formula.specify(var_name, term),
            Self::And(lhs, rhs) | Self::Or(lhs, rhs) | Self::Implies(lhs, rhs) => {
                lhs.specify(var_name, term);
                rhs.specify(var_name, term);
            }
        }
    }

    // pub fn interchange(&mut self, n: usize) -> usize {
    //     if let Some((v, f)) = self.is_forall_not() {
    //         if n == 0 {
    //             *self = Self::Negation(Box::new(Self::Existential(v, Box::new(f))));
    //             return n;
    //         }
    //     }
    //     if let Some((v, f)) = self.is_not_exists() {
    //         if n == 0 {
    //             *self = Self::Universal(v, Box::new(Self::Negation(Box::new(f))));
    //             return n;
    //         }
    //     }

    //     match self {
    //         Formula::Equality(_, _) => n,
    //         Formula::Universal(_, f) => f.interchange(n - 1),
    //         Formula::Existential(_, f) => f.interchange(n - 1),
    //         Formula::Negation(f) => f.interchange(n),
    //         Formula::And(lhs, rhs) | Formula::Or(lhs, rhs) | Formula::Implies(lhs, rhs) => {
    //             let n = lhs.interchange(n);
    //             rhs.interchange(n)
    //         }
    //     }
    // }

    // Replace all free instances of the named Term::Variable with a Term
    pub(crate) fn replace_free<S: ToString>(&mut self, var_name: &S, term: &Term) {
        match self {
            Self::Equality(lhs, rhs) => {
                lhs.replace(var_name, term);
                rhs.replace(var_name, term);
            }
            Self::Universal(_, formula) => formula.replace_free(var_name, term),
            Self::Existential(_, formula) => formula.replace_free(var_name, term),
            Self::Negation(formula) => formula.replace_free(var_name, term),
            Self::And(lhs, rhs) | Self::Or(lhs, rhs) | Self::Implies(lhs, rhs) => {
                lhs.replace_free(var_name, term);
                rhs.replace_free(var_name, term);
            }
        }
    }

    // Rename a variable everywhere it appears.
    pub fn rename_var<S: ToString>(&mut self, name: &S, new_name: &S) {
        match self {
            Self::Equality(lhs, rhs) => {
                lhs.rename_var(name, new_name);
                rhs.rename_var(name, new_name);
            }
            Self::Universal(v, formula) => {
                if v == &name.to_string() {
                    *v = new_name.to_string();
                }
                formula.rename_var(name, new_name);
            }
            Self::Existential(v, formula) => {
                if v == &name.to_string() {
                    *v = new_name.to_string();
                }
                formula.rename_var(name, new_name);
            }
            Self::Negation(formula) => formula.rename_var(name, new_name),
            Self::And(lhs, rhs) | Self::Or(lhs, rhs) | Self::Implies(lhs, rhs) => {
                lhs.rename_var(name, new_name);
                rhs.rename_var(name, new_name);
            }
        }
    }

    /// Fills the provided IndexSet with the name of every variable in the Formula.
    pub fn get_vars(&self, var_names: &mut IndexSet<String>) {
        match self {
            Self::Equality(lhs, rhs) => {
                lhs.get_vars(var_names);
                rhs.get_vars(var_names);
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
            Self::And(lhs, rhs) | Self::Or(lhs, rhs) | Self::Implies(lhs, rhs) => {
                lhs.get_vars(var_names);
                rhs.get_vars(var_names);
            }
        }
    }

    /// Fills the provided IndexSet with the name of every free variable in the Formula.
    pub fn get_vars_free(&self, var_names: &mut IndexSet<String>) {
        let mut all_v = IndexSet::<String>::new();
        let mut bound_v = IndexSet::<String>::new();
        self.get_vars(&mut all_v);
        self.get_vars(&mut bound_v);
        for free_v in all_v.difference(&bound_v) {
            var_names.insert(free_v.clone());
        }
    }

    /// Fills the provided IndexSet with the name of every bound variable in the Formula.
    pub fn get_vars_bound(&self, var_names: &mut IndexSet<String>) {
        match self {
            Self::Equality(_, _) => (),
            Self::Universal(v, formula) => {
                var_names.insert(v.to_string());
                formula.get_vars_bound(var_names);
            }
            Self::Existential(v, formula) => {
                var_names.insert(v.to_string());
                formula.get_vars_bound(var_names);
            }
            Self::Negation(formula) => formula.get_vars_bound(var_names),
            Self::And(lhs, rhs) | Self::Or(lhs, rhs) | Self::Implies(lhs, rhs) => {
                lhs.get_vars_bound(var_names);
                rhs.get_vars_bound(var_names);
            }
        }
    }

    pub fn contains_var<S: ToString>(&self, var_name: &S) -> bool {
        match self {
            Self::Equality(lhs, rhs) => lhs.contains_var(var_name) || rhs.contains_var(var_name),
            Self::Universal(v, formula) | Self::Existential(v, formula) => {
                *v == var_name.to_string() || formula.contains_var(var_name)
            }
            Self::Negation(formula) => formula.contains_var(var_name),
            Self::And(lhs, rhs) | Self::Or(lhs, rhs) | Self::Implies(lhs, rhs) => {
                lhs.contains_var(var_name) || rhs.contains_var(var_name)
            }
        }
    }

    pub fn contains_var_bound<S: ToString>(&self, var_name: &S) -> bool {
        match self {
            Self::Equality(_, _) => false,
            Self::Universal(v, formula) | Self::Existential(v, formula) => {
                *v == var_name.to_string() || formula.contains_var_bound(var_name)
            }
            Self::Negation(formula) => formula.contains_var_bound(var_name),
            Self::And(lhs, rhs) | Self::Or(lhs, rhs) | Self::Implies(lhs, rhs) => {
                lhs.contains_var_bound(var_name) || rhs.contains_var_bound(var_name)
            }
        }
    }

    pub fn contains_var_bound_universal<S: ToString>(&self, var_name: &S) -> bool {
        match self {
            Self::Equality(_, _) => false,
            Self::Universal(v, formula) => {
                *v == var_name.to_string() || formula.contains_var_bound_universal(var_name)
            }
            Self::Existential(_, formula) => formula.contains_var_bound_universal(var_name),
            Self::Negation(formula) => formula.contains_var_bound_universal(var_name),
            Self::And(lhs, rhs) | Self::Or(lhs, rhs) | Self::Implies(lhs, rhs) => {
                lhs.contains_var_bound_universal(var_name)
                    || rhs.contains_var_bound_universal(var_name)
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
            Self::And(lhs, rhs) | Self::Or(lhs, rhs) | Self::Implies(lhs, rhs) => {
                lhs.contains_var_bound_existential(name) || rhs.contains_var_bound_existential(name)
            }
        }
    }

    // pub fn is_not_exists(&self) -> Option<(String, Formula)> {
    //     if let Self::Negation(inner) = self {
    //         if let Self::Existential(v, f) = &**inner {
    //             Some((v.clone(), *f.clone()))
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }

    // pub fn is_forall_not(&self) -> Option<(String, Formula)> {
    //     if let Self::Universal(v, inner) = self {
    //         if let Self::Negation(f) = &**inner {
    //             Some((v.clone(), *f.clone()))
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }

    /// Produces a clone of the Formula in its austere form. The leftmost variable is renamed `a` in all appearances, the next is renamed `a'` and so on.
    pub fn austere(&self) -> Formula {
        let mut out = self.clone();
        out.to_austere();
        out
    }

    /// As .austere() but mutates the Formula instead.
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

    /// Create the unique BigUint that characterizes the Formula. This is done by converting the Formula to its austere form and then reading the bytes as a bigendian number.
    pub fn arithmetize(&self) -> BigUint {
        let s = self.austere().to_string();
        BigUint::from_bytes_be(s.as_bytes())
    }

    // These are guaranteed to produce well-formed formulas of TNT. However they may produce false statements.

    /// Equality of two Terms
    pub fn eq(lhs: &Term, rhs: &Term) -> Formula {
        Formula::Equality(lhs.clone(), rhs.clone())
    }

    /// Negation of a Formula
    pub fn not(formula: &Formula) -> Formula {
        Formula::Negation(Box::new(formula.clone()))
    }

    /// Logical OR of two Formulas
    pub fn or(lhs: &Formula, rhs: &Formula) -> Formula {
        Formula::Or(Box::new(lhs.clone()), Box::new(rhs.clone()))
    }

    /// Logical AND of two Formulas
    pub fn and(lhs: &Formula, rhs: &Formula) -> Formula {
        Formula::And(Box::new(lhs.clone()), Box::new(rhs.clone()))
    }

    /// lhs Formula implies rhs Formula
    pub fn implies(lhs: &Formula, rhs: &Formula) -> Formula {
        Formula::Implies(Box::new(lhs.clone()), Box::new(rhs.clone()))
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
}

impl Display for Formula {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Equality(lhs, rhs) => write!(f, "{lhs}={rhs}"),
            Self::Universal(name, formula) => write!(f, "A{name}:{formula}"),
            Self::Existential(name, formula) => write!(f, "E{name}:{formula}"),
            Self::Negation(formula) => write!(f, "~{formula}"),
            Self::And(lhs, rhs) => write!(f, "[{lhs}&{rhs}]"),
            Self::Or(lhs, rhs) => write!(f, "[{lhs}|{rhs}]"),
            Self::Implies(lhs, rhs) => write!(f, "[{lhs}>{rhs}]"),
        }
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

impl TryFrom<BigUint> for Formula {
    type Error = LogicError;

    fn try_from(value: BigUint) -> Result<Self, Self::Error> {
        match from_utf8(&value.to_bytes_be()) {
            Ok(s) => Formula::try_from(s),
            Err(e) => Err(LogicError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn english() {
        let f = Formula::try_from("Ea':Aa:[a=a&a'=Sb]").unwrap();
        assert_eq!(
            f.to_english(),
            "there exists a' such that for all a, [a = a and a' = Sb]"
        )
    }

    #[test]
    fn austere() {
        let t0 = Formula::try_from("Ab:[Ea:(Sb+Sa)=S(a*b)|Ec:SSS0=(Sc*Sa)]")
            .unwrap()
            .austere();
        let t1 = Formula::try_from("Aa:[Ea':(Sa+Sa')=S(a'*a)|Ea'':SSS0=(Sa''*Sa')]").unwrap();
        assert_eq!(t0, t1);
    }

    #[test]
    fn from_big_uint() {
        let t0 = Formula::try_from("Ea:Aa':[a'=a'&a=Sa'']").unwrap();
        let t1 = Formula::try_from(t0.arithmetize()).unwrap();
        assert_eq!(t0, t1);
    }
}
