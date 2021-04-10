use std::str::from_utf8;
use rand::{Rng, prelude::ThreadRng};

use crate::formula::Formula;
use crate::terms::{Term,Variable,Number,Expression};
use crate::properties::is_expression;

/*
Backus-Naur Form for the TNT Language

<num> ::= { "0" | "S" <num> }
<var> ::= { <lowercase_letter> | <var> "'" }
<arith_op> ::= { "+" | "*" }
<expr> ::= { num | var | "(" <expr> <arith_op> <expr> ")" | "S" expr }
<quant> ::= { "A" <var> ":" | "E" <var> ":" | "~" <quant> }
<logical_op> ::= { "&" | "|" | ">" }
<formula> ::= {  <expr> "=" <expr> | "[" <formula> <logical_op> <formula> "]" | <quant> <formula> }
*/

pub fn random_variable_str() -> String {
    let mut rng = rand::thread_rng();
    // Get a u8 corresponding to an ASCII lowercase letter and make it into a String
    let n: u8 = rng.gen_range(97..123);
    let mut s = from_utf8(&[n]).unwrap().to_string();
    while rng.gen_range(0.0..1.0) > 0.75 {
        s.push('\'')
    }
    s
}

pub fn random_variable() -> Variable {
    Variable::new(&random_variable_str())
}

pub fn random_number_str() -> String {
    let mut rng = rand::thread_rng();
    let mut s = "".to_string();
    while rng.gen_range(0.0..1.0) > 0.6 {
        s.push('S')
    }
    s.push('0');
    s
}

pub fn random_number() -> Number {

    Number::new(&random_number_str())
}

pub fn random_expression() -> Expression {

    fn simple_expr(n: i32) -> String {
        let mut out = "$".to_string();
        if n < 2 {
            out.push_str(&random_variable_str())
        } else if n < 4 {
            return random_number_str()
        } else if n == 4 {
            out.push_str("(#+#)")
        } else if n == 5 {
            out.push_str("(#*#)")
        }
        out
    }

    fn simple_succ(n: i32) -> String {
        match n {
            0 => "S".to_string(),
            1 => "".to_string(),
            2 => "".to_string(),
            3 => "$S".to_string(),
            _ => unreachable!(),
        }
    }

    let mut rng = rand::thread_rng();
    let mut s = simple_expr(rng.gen_range(0..6));


    while !is_expression(&s) {
        let n1 = rng.gen_range(0..6);
        let n2 = rng.gen_range(0..4);
        s = s.replacen("#", &simple_expr(n1), 1);
        s = s.replacen("$", &simple_succ(n2), 1);
    }

    Expression::new(&s)
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_random_variable() {
        for _ in 0..10 {
            random_variable();
        }
    }

    #[test]
    fn test_random_number() {
        for _ in 0..10 {
            random_number();
        }
    }

    #[test]
    fn test_random_expression() {
        for _ in 0..10 {
            println!("{}",random_expression());
        }
    }

}