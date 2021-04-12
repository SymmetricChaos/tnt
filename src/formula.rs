use std::fmt;
use num::bigint::BigUint;

use crate::{properties::{is_simple_formula,is_formula}};
use crate::translate::{to_latex,to_english,arithmetize,dearithmetize,to_austere};
use crate::string_manip::replace_all_re;
use crate::random::{random_simple_formula};
use crate::terms::{Variable,Term};



/// A Formula is a well-formed formula, either Simple or Complex
#[derive(Clone,Debug)]
pub enum Formula {
    /// Formula::Simple consists of precisely an equality of two terms
    Simple(String),
    /// Formula::Complex consists of any well-formed formula
    Complex(String),
}

impl Formula {
    /// An &str is automatically converted to the correct variant, this requires potentially slow parsing of the &str
    pub fn new(input: &str) -> Formula {
        if is_simple_formula(input) {
            return Formula::Simple(input.to_owned())
        } else if is_formula(input) {
            return Formula::Complex(input.to_owned()) 
        } else {
            panic!("{} is not a well formed formula",input)
        }
    }

    /// Fast creation of Formula::Simple with no checks, may be deprecated soon
    pub fn new_simple(input: &str) -> Formula {
        return Formula::Simple(input.to_owned())
    }

    /// Fast creation of Formula::Complex with no checks, may be deprecated soon
    pub fn new_complex(input: &str) -> Formula {
        return Formula::Complex(input.to_owned())
    }

    /// Translate the Formula to LaTeX representation
    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    /// Translate the Formula to relatively readable English
    pub fn english(&self) -> String {
        to_english(self.to_string())
    }

    /// Return a BigUint that represents the Formula
    pub fn arithmetize(&self) -> BigUint {
        arithmetize(self.to_string())
    }

    /// Create a formula from a BigUint
    pub fn dearithmetize(number: &BigUint) -> Formula {
        Formula::new(&dearithmetize(number))
    }

    /// Return the Formula converted into its canonical austere form
    pub fn austere(&self) -> Formula {
        Formula::new(&to_austere(self.to_string()))
    }

    /// Replace every instance of a Variable in the Formula with some Term
    pub fn replace_var<T: Term>(&self, v: &Variable, replacement: &T) -> Formula {
        let st = replace_all_re(&self.to_string(), &v.re, &replacement.get_string()[..]);
        Formula::new(&st )
    }

    /// Eliminate universal quantification of a Variable in the Formula then replace every instance with some Term
    pub fn specify_var<T: Term>(&self, v: &Variable, replacement: &T) -> Formula {
        let mut st = self.to_string().replace(&format!("A{}:",v),"");
        st = replace_all_re(&st, &v.re, &replacement.get_string()[..]);
        Formula::new(&st )
    }

    /// Does the Formula contain the Variable in question?
    pub fn contains_var(&self, v: &Variable) -> bool {
        v.re.find(&self.to_string()).unwrap().is_some()
    }

    /// Does the Formula contain the Variable in a quantification?
    pub fn contains_var_bound(&self, v: &Variable) -> bool {
        v.req.find(&self.to_string()).unwrap().is_some()
    }

    pub fn random_simple() -> Formula {
        random_simple_formula()
    }

}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Formula::Simple(form) => write!(f, "{}", form),
            Formula::Complex(form) => write!(f, "{}", form),
        }
    }
}

/// Two formulas are considered equal if their austere versions are identical
impl PartialEq for Formula {
    fn eq(&self, other: &Self) -> bool {
        self.austere().to_string() == other.austere().to_string()
    }
}




#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_replace_var() {
        let a = &Variable::new("a");
        let b = &Variable::new("b");
        let f1 = Formula::new("Aa:Ea':a=Sa'");
        assert_eq!( f1.replace_var(a,b).to_string(), "Ab:Ea':b=Sa'".to_string() )
    }

    #[test]
    fn test_specify_var() {
        let a = &Variable::new("a");
        let b = &Variable::new("b");
        let f1 = Formula::new("Aa:Ea':a=Sa'");
        assert_eq!( f1.specify_var(a,b).to_string(), "Ea':b=Sa'".to_string() )
    }

}