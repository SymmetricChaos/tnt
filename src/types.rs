use std::fmt;
use num::bigint::BigUint;
use onig::Regex;
use std::ops::{Add,Mul,Shl};

use crate::properties::{is_term,is_num,is_var,is_simple_formula,is_formula};
use crate::translate::{to_latex,to_english,arithmetize,dearithmetize};






#[derive(Clone,Debug,PartialEq)]
pub enum Formula {
    Simple(String),
    Complex(String),
}

impl Formula {
    pub fn new(input: &str) -> Formula {
        if is_simple_formula(input) {
            return Formula::Simple(input.to_owned())
        } else if is_formula(input) {
            return Formula::Complex(input.to_owned()) 
        } else {
            panic!("{} is not a well formed formula",input)
        }
    }

    // Fast creation of variants without checking
    pub fn new_simple(input: &str) -> Formula {
        return Formula::Simple(input.to_owned())
    }

    pub fn new_complex(input: &str) -> Formula {
        return Formula::Complex(input.to_owned())
    }

    // Pretty names
    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    pub fn english(&self) -> String {
        to_english(self.to_string())
    }

    pub fn arithmetize(&self) -> BigUint {
        arithmetize(self.to_string())
    }

    pub fn dearithmetize(number: &BigUint) -> Formula {
        Formula::new(&dearithmetize(number))
    }

    pub fn replace_var<T: Term>(&self, v: &Variable, replacement: &T) -> Formula {
        let out = v.re.replace_all(&self.to_string(), &replacement.get_string()[..]);
        Formula::new(&out)
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





pub trait Term {
    /// An &str is automatically converted to the correct variant, this requires potentially slow parsing of the &str
    fn new(input: &str) -> Self;

    /// Translate the Term to LaTeX representation
    fn latex(&self) -> String;

    /// Translate the Term to relatively readable English
    fn english(&self) -> String;

    /// Return a BigUint that represents the Term
    fn arithmetize(&self) -> BigUint;

    /// Create a Term from a BigUint
    fn dearithmetize(number: &BigUint) -> Self;

    fn get_string(&self) -> String;
}

pub struct Variable {
    string: String,
    re: Regex,
}

pub struct Number {
    string: String,
}

pub struct Equation {
    string: String,
}

impl Term for Variable {
    fn new(input: &str) -> Variable {
        if is_var(input) {
            let p = format!("{}(?!')",input);
            let re = Regex::new(&p).unwrap();
            let string = input.to_owned();
            return Variable{ string, re }
        } else {
            panic!("{} is not a valid Variable",input)
        }
    }

    fn latex(&self) -> String {
        to_latex(self.string.clone())
    }

    /// Translate the Term to relatively readable English
    fn english(&self) -> String {
        to_english(self.string.clone())
    }

    /// Return a BigUint that represents the Term
    fn arithmetize(&self) -> BigUint {
        arithmetize(self.string.clone())
    }

    /// Create a Term from a BigUint
    fn dearithmetize(number: &BigUint) -> Variable {
        Variable::new(&dearithmetize(number))
    }

    fn get_string(&self) -> String {
        self.string.clone()
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<'a, 'b> Add<&'b Variable> for &'a Variable {
    type Output = Equation;

    fn add(self, other: &'b Variable) -> Equation {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Add<&'b Number> for &'a Variable {
    type Output = Equation;

    fn add(self, other: &'b Number) -> Equation {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Add<&'b Equation> for &'a Variable {
    type Output = Equation;

    fn add(self, other: &'b Equation) -> Equation {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Variable> for &'a Variable {
    type Output = Equation;

    fn mul(self, other: &'b Variable) -> Equation {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Number> for &'a Variable {
    type Output = Equation;

    fn mul(self, other: &'b Number) -> Equation {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Equation> for &'a Variable {
    type Output = Equation;

    fn mul(self, other: &'b Equation) -> Equation {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a> Shl<usize> for &'a Variable {
    type Output = Equation;

    fn shl(self, other: usize) -> Equation {
        let s = "S".repeat(other);
        let new = format!("{}{}", s, self.get_string());
        Equation{ string: new }
    }
}







impl Term for Number {
    fn new(input: &str) -> Number {
        if is_num(input) {
            let string = input.to_owned();
            return Number{ string }
        } else {
            panic!("{} is not a valid Number",input)
        }
    }

    fn latex(&self) -> String {
        to_latex(self.string.clone())
    }

    /// Translate the Term to relatively readable English
    fn english(&self) -> String {
        to_english(self.string.clone())
    }

    /// Return a BigUint that represents the Term
    fn arithmetize(&self) -> BigUint {
        arithmetize(self.string.clone())
    }

    /// Create a Term from a BigUint
    fn dearithmetize(number: &BigUint) -> Number {
        Number::new(&dearithmetize(number))
    }

    fn get_string(&self) -> String {
        self.string.clone()
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<'a, 'b> Add<&'b Variable> for &'a Number {
    type Output = Equation;

    fn add(self, other: &'b Variable) -> Equation {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Add<&'b Number> for &'a Number {
    type Output = Equation;

    fn add(self, other: &'b Number) -> Equation {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Add<&'b Equation> for &'a Number {
    type Output = Equation;

    fn add(self, other: &'b Equation) -> Equation {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Variable> for &'a Number {
    type Output = Equation;

    fn mul(self, other: &'b Variable) -> Equation {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Number> for &'a Number {
    type Output = Equation;

    fn mul(self, other: &'b Number) -> Equation {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Equation> for &'a Number {
    type Output = Equation;

    fn mul(self, other: &'b Equation) -> Equation {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a> Shl<usize> for &'a Number {
    type Output = Number;

    fn shl(self, other: usize) -> Number {
        let s = "S".repeat(other);
        let new = format!("{}{}", s, self.get_string());
        Number{ string: new }
    }
}





impl Term for Equation {
    fn new(input: &str) -> Equation {
        if is_term(input) {
            let string = input.to_owned();
            return Equation{ string }
        } else {
            panic!("{} is not a valid Term",input)
        }
    }

    fn latex(&self) -> String {
        to_latex(self.string.clone())
    }

    /// Translate the Term to relatively readable English
    fn english(&self) -> String {
        to_english(self.string.clone())
    }

    /// Return a BigUint that represents the Term
    fn arithmetize(&self) -> BigUint {
        arithmetize(self.string.clone())
    }

    /// Create a Term from a BigUint
    fn dearithmetize(number: &BigUint) -> Equation {
        Equation::new(&dearithmetize(number))
    }

    fn get_string(&self) -> String {
        self.string.clone()
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<'a, 'b> Add<&'b Variable> for &'a Equation {
    type Output = Equation;

    fn add(self, other: &'b Variable) -> Equation {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Add<&'b Number> for &'a Equation {
    type Output = Equation;

    fn add(self, other: &'b Number) -> Equation {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Add<&'b Equation> for &'a Equation {
    type Output = Equation;

    fn add(self, other: &'b Equation) -> Equation {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Variable> for &'a Equation {
    type Output = Equation;

    fn mul(self, other: &'b Variable) -> Equation {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Number> for &'a Equation {
    type Output = Equation;

    fn mul(self, other: &'b Number) -> Equation {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Equation> for &'a Equation {
    type Output = Equation;

    fn mul(self, other: &'b Equation) -> Equation {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Equation{ string: new }
    }
}

impl<'a> Shl<usize> for &'a Equation {
    type Output = Equation;

    fn shl(self, other: usize) -> Equation {
        let s = "S".repeat(other);
        let new = format!("{}{}", s, self.get_string());
        Equation{ string: new }
    }
}





#[test]
fn test_variable() {
    let v1 = &Variable::new("a");
    let v2 = &Variable::new("b");
    let n1 = &Number::new("0");
    let e1 = &Equation::new("(x'+SS0)");

    assert_eq!((v1 + v2).get_string(),"(a+b)");
    assert_eq!((v1 * v2).get_string(),"(a*b)");
    assert_eq!((v1 << 2 ).get_string(),"SSa");
    assert_eq!((v1 + n1).get_string(),"(a+0)");
    assert_eq!((v1 * n1).get_string(),"(a*0)");
    assert_eq!((v1 + e1).get_string(),"(a+(x'+SS0))");
    assert_eq!((v1 * e1).get_string(),"(a*(x'+SS0))");
}

#[test]
fn test_number() {
    let v1 = &Variable::new("a");
    let n1 = &Number::new("0");
    let n2 = &Number::new("S0");
    let e1 = &Equation::new("(x'+SS0)");

    assert_eq!((n1 + n2).get_string(),"(0+S0)");
    assert_eq!((n1 * n2).get_string(),"(0*S0)");
    assert_eq!((n1 << 2 ).get_string(),"SS0");
    assert_eq!((n1 + v1).get_string(),"(0+a)");
    assert_eq!((n1 * v1).get_string(),"(0*a)");
    assert_eq!((n1 + e1).get_string(),"(0+(x'+SS0))");
    assert_eq!((n1 * e1).get_string(),"(0*(x'+SS0))");

}

#[test]
fn test_equation() {
    let v1 = &Variable::new("a");
    let n1 = &Number::new("0");
    let e1 = &Equation::new("(x'+SS0)");
    let e2 = &Equation::new("S(u*v)");

    assert_eq!((e1 + e2).get_string(),"((x'+SS0)+S(u*v))");
    assert_eq!((e1 * e2).get_string(),"((x'+SS0)*S(u*v))");
    assert_eq!((e1 << 2 ).get_string(),"SS(x'+SS0)");
    assert_eq!((e1 + v1).get_string(),"((x'+SS0)+a)");
    assert_eq!((e1 * v1).get_string(),"((x'+SS0)*a)");
    assert_eq!((e1 + n1).get_string(),"((x'+SS0)+0)");
    assert_eq!((e1 * n1).get_string(),"((x'+SS0)*0)");


}











// All types used are accounted for here
// This will allow us to parse a string into a type
/*
#[derive(Clone,Debug,PartialEq)]
pub enum TNT {
    Term(Term),
    Formula(Formula)
}

impl TNT {
    pub fn new(input: &str) -> TNT {
        if is_equation(input) {
            return TNT::Term(Term::new(input))
        } else if is_formula(input) {
            return TNT::Formula(Formula::new(input))
        } else {
            panic!()
        }
    }

    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    pub fn english(&self) -> String {
        to_english(self.to_string())
    }
}

impl TryFrom<Term> for TNT {
    type Error = &'static str;

    fn try_from(value: Term) -> Result<Self, Self::Error> {
        Ok(TNT::Term(value))
    }
}

impl TryFrom<Formula> for TNT {
    type Error = &'static str;

    fn try_from(value: Formula) -> Result<Self, Self::Error> {
        Ok(TNT::Formula(value))
    }
}

impl fmt::Display for TNT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TNT::Term(term) => write!(f, "{}", term),
            TNT::Formula(term) => write!(f, "{}", term),
        }
    }
}
 */