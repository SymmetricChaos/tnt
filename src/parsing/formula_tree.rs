use std::fmt;

use num::BigUint;

use crate::translate::{arithmetize, to_english, to_latex};

use regex::Regex;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref VAR: Regex = Regex::new("[a-z]\'*").unwrap();
}


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variable {
    name: String
}

impl Variable {
    pub fn new(name: &str) -> Variable {
        if VAR.is_match(name) {
            Variable{name: name.to_string()}
        } else {
            panic!("invalid variable name: `{}`", name)
        }
    }

    pub fn replace_var(replacement: Term) -> Term {
        replacement
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}



#[derive(Clone,Debug)]
pub enum Term {
    Zero,
    Variable(Variable),
    Add(Box<Term>,Box<Term>),
    Mul(Box<Term>,Box<Term>),
    Successor(Box<Term>),
}

impl Term {
    pub fn replace_var(&self, v: &Variable, replacement: &Term) -> Term {
        match self {
            Term::Zero => Term::Zero,

            Term::Variable(var) => {
                if var == v {
                    replacement.clone()
                } else {
                    Term::Variable(var.clone())
                }
            },

            Term::Add(t1, t2) => Term::Add(Box::new(t1.replace_var(v,replacement)),
                                                                Box::new(t2.replace_var(v,replacement))
                                                            ),

            Term::Mul(t1, t2) => Term::Add(Box::new(t1.replace_var(v,replacement)),
                                                                Box::new(t2.replace_var(v,replacement))
                                                            ),

            Term::Successor(t) => Term::Successor(Box::new(t.replace_var(v, replacement)))

        }
    }

    pub fn contains_var(&self, v: &Variable) -> bool {
        match self {
            Term::Zero => false,
            Term::Variable(var) => var == v,
            Term::Add(lhs, rhs) => {
                lhs.contains_var(v) || rhs.contains_var(v)
            }
            Term::Mul(lhs, rhs) => {
                lhs.contains_var(v) || rhs.contains_var(v)
            }
            Term::Successor(t) => t.contains_var(v)
        }
    }

    pub fn get_vars(&self) -> Vec<Variable> {
        let mut vec = Vec::new();
        match self {
            Term::Zero => {},
            Term::Variable(v) => vec.push(v.clone()),
            Term::Add(lhs, rhs) => {
                vec.extend(lhs.get_vars());
                vec.extend(rhs.get_vars());
            },
            Term::Mul(lhs, rhs) => {
                vec.extend(lhs.get_vars());
                vec.extend(rhs.get_vars());
            },
            Term::Successor(term) => {
                vec.extend(term.get_vars());;
            },
        }
        vec.sort();
        vec.dedup();
        vec
    }

}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Term::Zero => write!(f, "0"),
            Term::Variable(v) => write!(f, "{}", v.name),
            Term::Add(t1, t2) => write!(f, "({}+{})", t1, t2),
            Term::Mul(t1, t2) =>  write!(f, "({}*{})", t1, t2),
            Term::Successor(t) => write!(f, "S{}", t),
        }
    }
}



#[derive(Clone,Debug)]
pub enum Formula {
    Equality(Term,Term), // the only Simple formula
    And(Box<Formula>,Box<Formula>),
    Or(Box<Formula>,Box<Formula>),
    Implies(Box<Formula>,Box<Formula>),
    Exists(Variable,Box<Formula>),
    ForAll(Variable,Box<Formula>),
    Negation(Box<Formula>)
}

impl Formula {
    /// An &str is automatically converted to the correct variant, this requires potentially slow parsing of the &str
/*     pub fn new_formula(&str) -> Formula {

    } */

    /// Fast creation of various formulae
    pub fn equality(t1: Term, t2: Term) -> Formula {
        return Formula::Equality(t1,t2)
    }

    pub fn and(t1: Formula, t2: Formula) -> Formula {
        return Formula::And(Box::new(t1),Box::new(t2))
    }

    pub fn or(t1: Formula, t2: Formula) -> Formula {
        return Formula::Or(Box::new(t1),Box::new(t2))
    }

    pub fn implies(t1: Formula, t2: Formula) -> Formula {
        return Formula::Implies(Box::new(t1),Box::new(t2))
    }

    pub fn exists(v: Variable, f: Formula) -> Formula {
        return Formula::Exists(v,Box::new(f))
    }

    pub fn forall(v: Variable, f: Formula) -> Formula {
        return Formula::ForAll(v,Box::new(f))
    }

    pub fn negation(f: Formula) -> Formula {
        return Formula::Negation(Box::new(f))
    }



    pub fn replace_var(&self, v: &Variable, replacement: &Term) -> Formula {
        match self {

            Formula::Equality(t1, t2) => Formula::Equality(
                t1.replace_var(v,replacement),
                t2.replace_var(v,replacement)
            ),

            Formula::And(f1, f2) => Formula::And(
                Box::new(f1.replace_var(v,replacement)),
                Box::new(f2.replace_var(v,replacement))
            ),

            Formula::Or(f1, f2) => Formula::Or(
                Box::new(f1.replace_var(v,replacement)),
                Box::new(f2.replace_var(v,replacement))
            ),

            Formula::Implies(f1, f2) => Formula::Implies(
                Box::new(f1.replace_var(v,replacement)),
                Box::new(f2.replace_var(v,replacement))
            ),

            Formula::Exists(v, f) => Formula::Exists(
                v.clone(),
                Box::new(f.replace_var(v,replacement))
            ),

            Formula::ForAll(v, f) => Formula::ForAll(
                v.clone(),
                Box::new(f.replace_var(v,replacement))
            ),

            Formula::Negation(f) => Formula::Negation(
                Box::new(f.replace_var(v,replacement))
            ),

        }
    }



    // Translation of the Formula to a different representation
    pub fn english(&self) -> String {
        to_english(self.to_string())
    }

    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    pub fn arithmetize(&self) -> BigUint {
        arithmetize(self.to_string())
    }



    /// Does the Formula contain the Variable in question?
    pub fn contains_var(&self, v: &Variable) -> bool {
        match self {
            Formula::Equality(lhs, rhs) => {
                lhs.contains_var(v) || rhs.contains_var(v)
            }
            Formula::And(lhs, rhs) => {
                lhs.contains_var(v) || rhs.contains_var(v)
            }
            Formula::Or(lhs, rhs) => {
                lhs.contains_var(v) || rhs.contains_var(v)
            }
            Formula::Implies(lhs, rhs) => {
                lhs.contains_var(v) || rhs.contains_var(v)
            }
            Formula::Exists(var, f) => {
                var == v || f.contains_var(v)
            }
            Formula::ForAll(var, f) => {
                var == v || f.contains_var(v)
            }
            Formula::Negation(f) => f.contains_var(v)
        }
    }

    // All variables in the Formula
    pub fn get_vars(&self) -> Vec<Variable> {
        let mut vec = Vec::new();
        match self {
            Formula::Equality(lhs, rhs) => {
                vec.extend(lhs.get_vars());
                vec.extend(rhs.get_vars());
            },
            Formula::And(lhs, rhs) => {
                vec.extend(lhs.get_vars());
                vec.extend(rhs.get_vars());
            },
            Formula::Or(lhs, rhs) => {
                vec.extend(lhs.get_vars());
                vec.extend(rhs.get_vars());
            },
            Formula::Implies(lhs, rhs) => {
                vec.extend(lhs.get_vars());
                vec.extend(rhs.get_vars());
            },
            Formula::Exists(v, form) => {
                vec.push(v.clone());
                vec.extend(form.get_vars());
            },
            Formula::ForAll(v, form) => {
                vec.push(v.clone());
                vec.extend(form.get_vars());
            },
            Formula::Negation(form) => {
                vec.extend(form.get_vars());
            },
        }
        vec.sort();
        vec.dedup();
        vec
    }

/*
    pub fn dearithmetize(number: &BigUint) -> Formula {
        Formula::new(&dearithmetize(number))
    }

    /// Return the Formula converted into its canonical austere form
    pub fn austere(&self) -> Formula {
        Formula::new(&to_austere(self.to_string()))
    }

    /// Replace every instance of a Variable in the Formula with some Term


    /// Eliminate universal quantification of a Variable in the Formula then replace every instance with some Term
    pub fn specify_var<T: Term>(&self, v: &Variable, replacement: &T) -> Formula {
        let mut st = self.to_string().replace(&format!("A{}:",v),"");
        st = replace_all_re(&st, &v.re, &replacement.get_string()[..]);
        Formula::new(&st )
    }



    /// Does the Formula contain the Variable in a quantification?
    pub fn contains_var_bound(&self, v: &Variable) -> bool {
        v.req.find(&self.to_string()).unwrap().is_some()
    } */

}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Formula::Equality(t1, t2) => write!(f, "{}={}", t1, t2),
            Formula::And(f1, f2) => write!(f, "[{}&{}]", f1, f2),
            Formula::Or(f1, f2) => write!(f, "[{}|{}]", f1, f2),
            Formula::Implies(f1, f2) => write!(f, "[{}>{}]", f1, f2),
            Formula::Exists(v, form) => write!(f, "E{}:{}", v, form),
            Formula::ForAll(v, form) => write!(f, "A{}:{}", v, form),
            Formula::Negation(form) => write!(f, "~{}", form),
        }
    }
}
/* 
/// Two formulas are considered equal if their austere versions are identical
impl PartialEq for Formula {
    fn eq(&self, other: &Self) -> bool {
        self.austere().to_string() == other.austere().to_string()
    }
} */




#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn create_variable() {
        let a = &Variable::new("A");
        let b = &Variable::new("b''");
        println!("{}",a);
        println!("{}",b);
    }



}