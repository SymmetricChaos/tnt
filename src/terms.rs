
use std::fmt;
use num::bigint::BigUint;
use fancy_regex::Regex;
use std::ops::{Add,Mul};

use crate::properties::{is_expression,is_num,is_var};
use crate::translate::{to_latex,to_english,arithmetize,dearithmetize};

/// Term is implemented for the three structs that hold valid pieces of unquantified TNT formulas: Variable, Number, and Expression. Besides the traits bound to Term the also implement the + and * operations.
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
    pub string: String,
    pub re: Regex,
    pub req: Regex,
}

/// Number represents any valid number of TNT which is a 0 preceeded by zero or more S.
#[derive(Debug)]
pub struct Number {
    pub string: String,
}

/// Expression represents any valid expression of TNT which is any combination of Variables, Number, and Expressions using addition, multiplication, and the successor operation.
#[derive(Debug)]
pub struct Expression {
    pub string: String,
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

    fn english(&self) -> String {
        self.string.clone()
    }

    fn arithmetize(&self) -> BigUint {
        arithmetize(self.string.clone())
    }

    fn dearithmetize(number: &BigUint) -> Variable {
        Variable::new(&dearithmetize(number))
    }

    fn get_string(&self) -> String {
        self.string.clone()
    }
}

impl Variable {
    // Single successor
    pub fn succ(&self) -> Expression {
        let new = format!("S{}", self.get_string());
        Expression{ string: new }
    }

    // Quick multiple successor
    pub fn succs(&self, n: usize) -> Expression {
        let s = "S".repeat(n);
        let new = format!("{}{}", s, self.get_string());
        Expression{ string: new }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<'a, 'b> Add<&'b Variable> for &'a Variable {
    type Output = Expression;

    fn add(self, other: &'b Variable) -> Expression {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Add<&'b Number> for &'a Variable {
    type Output = Expression;

    fn add(self, other: &'b Number) -> Expression {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Add<&'b Expression> for &'a Variable {
    type Output = Expression;

    fn add(self, other: &'b Expression) -> Expression {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Variable> for &'a Variable {
    type Output = Expression;

    fn mul(self, other: &'b Variable) -> Expression {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Number> for &'a Variable {
    type Output = Expression;

    fn mul(self, other: &'b Number) -> Expression {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Expression> for &'a Variable {
    type Output = Expression;

    fn mul(self, other: &'b Expression) -> Expression {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Expression{ string: new }
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

    fn english(&self) -> String {
        to_english(self.string.clone())
    }

    fn arithmetize(&self) -> BigUint {
        arithmetize(self.string.clone())
    }

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

    // Single successor
    pub fn succ(&self) -> Number {
        let new = format!("S{}", self.get_string());
        Number{ string: new }
    }

    // Quick multiple successor
    pub fn succs(&self, n: usize) -> Number {
        let s = "S".repeat(n);
        let new = format!("{}{}", s, self.get_string());
        Number{ string: new }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<'a, 'b> Add<&'b Variable> for &'a Number {
    type Output = Expression;

    fn add(self, other: &'b Variable) -> Expression {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Add<&'b Number> for &'a Number {
    type Output = Expression;

    fn add(self, other: &'b Number) -> Expression {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Add<&'b Expression> for &'a Number {
    type Output = Expression;

    fn add(self, other: &'b Expression) -> Expression {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Variable> for &'a Number {
    type Output = Expression;

    fn mul(self, other: &'b Variable) -> Expression {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Number> for &'a Number {
    type Output = Expression;

    fn mul(self, other: &'b Number) -> Expression {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Expression> for &'a Number {
    type Output = Expression;

    fn mul(self, other: &'b Expression) -> Expression {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}





impl Term for Expression {
    fn new(input: &str) -> Expression {
        if is_expression(input) {
            let string = input.to_owned();
            return Expression{ string }
        } else {
            panic!("{} is not a valid Term",input)
        }
    }

    fn latex(&self) -> String {
        to_latex(self.string.clone())
    }

    fn english(&self) -> String {
        to_english(self.string.clone())
    }

    fn arithmetize(&self) -> BigUint {
        arithmetize(self.string.clone())
    }

    fn dearithmetize(number: &BigUint) -> Expression {
        Expression::new(&dearithmetize(number))
    }

    fn get_string(&self) -> String {
        self.string.clone()
    }
}

impl Expression {
    // Single successor
    pub fn succ(&self) -> Expression {
        let new = format!("S{}", self.get_string());
        Expression{ string: new }
    }

    // Quick multiple successor
    pub fn succs(&self, n: usize) -> Expression {
        let s = "S".repeat(n);
        let new = format!("{}{}", s, self.get_string());
        Expression{ string: new }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<'a, 'b> Add<&'b Variable> for &'a Expression {
    type Output = Expression;

    fn add(self, other: &'b Variable) -> Expression {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Add<&'b Number> for &'a Expression {
    type Output = Expression;

    fn add(self, other: &'b Number) -> Expression {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Add<&'b Expression> for &'a Expression {
    type Output = Expression;

    fn add(self, other: &'b Expression) -> Expression {
        let new = format!("({}+{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Variable> for &'a Expression {
    type Output = Expression;

    fn mul(self, other: &'b Variable) -> Expression {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Number> for &'a Expression {
    type Output = Expression;

    fn mul(self, other: &'b Number) -> Expression {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}

impl<'a, 'b> Mul<&'b Expression> for &'a Expression {
    type Output = Expression;

    fn mul(self, other: &'b Expression) -> Expression {
        let new = format!("({}*{})", self.get_string(), other.get_string());
        Expression{ string: new }
    }
}





#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_variable() {
        let v1 = &Variable::new("a");
        let v2 = &Variable::new("b");
        let n1 = &Number::new("0");
        let e1 = &Expression::new("(x'+SS0)");
    
        assert_eq!((v1 + v2).get_string(),"(a+b)");
        assert_eq!((v1 * v2).get_string(),"(a*b)");
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
        let e1 = &Expression::new("(x'+SS0)");
    
        assert_eq!((n1 + n2).get_string(),"(0+S0)");
        assert_eq!((n1 * n2).get_string(),"(0*S0)");
        assert_eq!((n1 + v1).get_string(),"(0+a)");
        assert_eq!((n1 * v1).get_string(),"(0*a)");
        assert_eq!((n1 + e1).get_string(),"(0+(x'+SS0))");
        assert_eq!((n1 * e1).get_string(),"(0*(x'+SS0))");
    }
    
    #[test]
    fn test_expression() {
        let v1 = &Variable::new("a");
        let n1 = &Number::new("0");
        let e1 = &Expression::new("(x'+SS0)");
        let e2 = &Expression::new("S(u*v)");
    
        assert_eq!((e1 + e2).get_string(),"((x'+SS0)+S(u*v))");
        assert_eq!((e1 * e2).get_string(),"((x'+SS0)*S(u*v))");
        assert_eq!((e1 + v1).get_string(),"((x'+SS0)+a)");
        assert_eq!((e1 * v1).get_string(),"((x'+SS0)*a)");
        assert_eq!((e1 + n1).get_string(),"((x'+SS0)+0)");
        assert_eq!((e1 * n1).get_string(),"((x'+SS0)*0)");
    }

}