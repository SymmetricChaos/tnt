#![allow(dead_code)]

mod types;
mod properties;
mod operations;
mod ops_production;
mod ops_construction;
mod string_manip;
mod deduction;

use crate::ops_production::{successor, specification, transitivity};
use crate::types::{number, variables, Formula};
use crate::deduction::{AXIOMS};

// Nonstandard symbols used: ∀∃∧∨⊃·

fn main() {

    //prove 1+1=2
    let vars = variables(vec!["a","b"]);
    let (a,b) = (&vars[0],&vars[1]);
    let zero = &number(0);
    let one = &number(1);

    let s0 = &Formula::new(AXIOMS[2]);
    println!("{}",s0);
    let s1 = &specification(s0, a, one);
    println!("{}",s1);
    let s2 = &specification(s1, b, zero).to_atom().unwrap();
    println!("{}",s2);
    let s3 = &Formula::new(AXIOMS[1]);
    println!("{}",s3);
    let s4 = &specification(s3, a, one).to_atom().unwrap();
    println!("{}",s4);
    let s5 = &successor(&s4);
    println!("{}",s5);
    let s6 = &transitivity(s2,s5);
    println!("{}",s6);
}
