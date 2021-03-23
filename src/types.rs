use std::fmt;
use num::bigint::BigUint;
use onig::Regex;
use std::ops::{Add,Mul,Shl};

use crate::properties::{is_equation,is_num,is_var,is_simple_formula,is_formula};
use crate::translate::{to_latex,to_english,arithmetize,dearithmetize};





/// A Formula is a well-formed formula, either Simple or Complex
#[derive(Clone,Debug,PartialEq)]
pub enum Formula {
    /// Formula::Simple consists of an equality of two terms
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

    /// Fast creation of Formula::Simple with no checks
    pub fn new_simple(input: &str) -> Formula {
        return Formula::Simple(input.to_owned())
    }

    /// Fast creation of Formula::Complex with no checks
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

    /// Replace every instance of a Variable with some Term
    pub fn replace_var<T: Term>(&self, v: &Variable, replacement: &T) -> Formula {
        Formula::new(&v.re.replace_all(&self.to_string(), &replacement.get_string()[..]))
    }

    /// Eliminate universal quantification of a Variable then replace every instance with some Term
    pub fn specify_var<T: Term>(&self, v: &Variable, replacement: &T) -> Formula {
        let out = self.to_string().replace(&format!("A{}:",v),"");
        Formula::new(&v.re.replace_all(&out, &replacement.get_string()[..]))
    }

    /// Does the Formula contain the Variable in question?
    pub fn contains_var(&self, v: &Variable) -> bool {
        v.re.find(&self.to_string()).is_some()
    }

    /// Does the Formula contain the Variable in within a quantification?
    pub fn contains_var_bound(&self, v: &Variable) -> bool {
        v.req.find(&self.to_string()).is_some()
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




/// Term is implemented for Variable, Number, and Equation the structs that hold valid pieces of TNT formulas.
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

    /// Get the contrained String
    fn get_string(&self) -> String;
}

/// Variable represents any valid variable of TNT, a lowercase English letter followed by zero or more apostophes. Besides the string itself it keeps two Regex with it representing the variable itself and quantification of the variable. This prevents the Regex from having to be built each time the Variable is searched for.
#[derive(Debug)]
pub struct Variable {
    string: String,
    re: Regex,
    req: Regex,
}

/// Number represents any valid number of TNT, 0 preceeded by zero or more S.
#[derive(Debug)]
pub struct Number {
    string: String,
}

/// Equation representd any valid equation of TNT, any addition, multiplication, or successor of Variable, Number, or Equation.
#[derive(Debug)]
pub struct Equation {
    string: String,
}

impl Term for Variable {
    fn new(input: &str) -> Variable {
        if is_var(input) {
            let p1 = format!("{}(?!')",input);
            let re = Regex::new(&p1).unwrap();
            let p2 = format!("[AE]{}:",input);
            let req = Regex::new(&p2).unwrap();
            let string = input.to_owned();
            return Variable{ string, re, req }
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

impl Number {
    pub fn zero() -> Number {
        Number{ string: "0".to_string() }
    }

    pub fn one() -> Number {
        Number{ string: "S0".to_string() }
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
        if is_equation(input) {
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





/// TNT consists of any valid statement
#[derive(Debug)]
pub enum TNT {
    Formula(Formula),
    Number(Number),
    Variable(Variable),
    Equation(Equation),
}

impl TNT {
    pub fn new(input: &str) -> TNT {
        if is_num(input) {
            return TNT::Number(Number::new(input))
        } else if is_var(input) {
            return TNT::Variable(Variable::new(input))
        } else if is_equation(input) {
            return TNT::Equation(Equation::new(input))
        } else if is_formula(input) {
            return TNT::Formula(Formula::new(input))
        } else {
            panic!()
        }
    }

    /// Translate the TNT to LaTeX representation
    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    /// Translate the TNT to relatively readable English
    pub fn english(&self) -> String {
        to_english(self.to_string())
    }
}

impl fmt::Display for TNT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TNT::Number(term) => write!(f, "{}", term),
            TNT::Variable(term) => write!(f, "{}", term),
            TNT::Equation(term) => write!(f, "{}", term),
            TNT::Formula(term) => write!(f, "{}", term),
        }
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









