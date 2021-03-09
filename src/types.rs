use std::fmt;
use onig::Regex;

use crate::properties::{is_term,is_well_formed_formula,is_var,is_atom};
use crate::string_manip::{get_bound_vars};

#[derive(Debug)]
pub struct Variable {
    pub s: String,
    pub regex: Regex,
}


#[derive(Debug)]
pub struct Term {
    pub s: String,
}

#[derive(Debug)]
pub struct Atom {
    pub s: String,
}

#[derive(Debug)]
pub struct Formula {
    pub s: String,
    pub bound_vars: Vec<String>
}




// Every Variable is a Term but not every Term is an Variable
pub trait Termlike {
    fn get_string(&self) -> &str;
    fn to_var(self) -> Option<Variable>;
}
impl Termlike for Term {
    fn get_string(&self) -> &str {
        &self.s
    }
    fn to_var(self) -> Option<Variable> {
        if is_var(&self.s) {
            return Some(Variable::new(&self.s))
        } else {
            return None
        }
    }
}
impl Termlike for Variable {
    fn get_string(&self) -> &str {
        &self.s
    }
    fn to_var(self) -> Option<Variable> {
        Some(self)
    }
}

// Every Atom is a Formula but not every Forumula is an Atom
pub trait Wellformed {
    fn get_string(&self) -> &str;
    fn well_formed(&self) -> bool;
    fn bound_vars(&self) -> Vec<String>;
}

impl Wellformed for Formula {
    fn get_string(&self) -> &str {
        &self.s
    }
    
    fn well_formed(&self) -> bool {
        is_well_formed_formula(&self.s)
    }

    fn bound_vars(&self) -> Vec<String> {
        self.bound_vars.clone()
    }
}
impl Wellformed for Atom {
    fn get_string(&self) -> &str {
        &self.s
    }

    fn well_formed(&self) -> bool {
        is_well_formed_formula(&self.s)
    }

    fn bound_vars(&self) -> Vec<String> {
        Vec::<String>::new()
    }
}



impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Variable {
    pub fn new(input: &str) -> Variable {
        if is_var(input) {
            let s = input.to_owned();
            let regex = Regex::new(&format!("{}(?!')",input)).unwrap();
            return Variable{ s, regex }
        } else {
            panic!("{} is not a variable",input)
        }
    }
}



impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Term {
    pub fn new(input: &str) -> Term {
        if is_term(input) {
            return Term{ s: input.to_owned() }
        } else {
            panic!("{} is not a term",input)
        }
    }
}



impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Atom {
    pub fn new(input: &str) -> Atom {
        return Atom{ s: input.to_owned() }
    }
}


impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Formula {
    pub fn new(input: &str) -> Formula {
        return Formula{ s: input.to_owned(), bound_vars: get_bound_vars(input) }
    }

    pub fn to_atom(self) -> Option<Atom> {
        if is_atom(&self.s) {
            return Some(Atom::new(&self.s))
        } else {
            return None
        }
    }
}




// Utility Functions for working with types

// Quickly generate a bunch of variables
pub fn variables(names: Vec<&str>) -> Vec<Variable> {
    let mut out = Vec::new();
    for n in names {
        out.push(Variable::new(n));
    }
    out
}

pub fn number(n: usize) -> Term {
    let start = "S".repeat(n);
    let new_s = format!("{}0",start);
    Term::new(&new_s)
}

pub fn var_in_string(v: &Variable, s: &str) -> bool {
    if v.regex.find(s).is_some() {
        return true
    } else {
        return false
    }
}