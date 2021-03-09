#![allow(dead_code)]

mod types;
mod properties;
mod ops_production;
mod ops_construction;
mod string_manip;
mod deduction;
mod alt_type_system;

use crate::ops_production::{successor, specification, transitivity};
use crate::types::{Term, Formula};
use crate::deduction::{AXIOMS};

fn main() {

    //prove 1+1=2
    let a = &Term::new("a");
    let b = &Term::new("b");
    let zero = &Term::new("0");
    let one = &Term::new("S0");

    let s0 = &Formula::new(AXIOMS[2]);
    println!("{}",s0);
    let s1 = &specification(s0, a, one);
    println!("{}",s1);
    let s2 = &specification(s1, b, zero);
    println!("{}",s2);
    let s3 = &Formula::new(AXIOMS[1]);
    println!("{}",s3);
    let s4 = &specification(s3, a, one);
    println!("{}",s4);
    let s5 = &successor(&s4);
    println!("{}",s5);
    let s6 = &transitivity(s2,s5);
    println!("{}",s6);
}
