#![allow(dead_code)]

mod types;
mod properties;
mod ops_production;
mod ops_construction;
mod string_manip;
mod deduction;
mod latex;

use crate::ops_production::{successor, specification, transitivity, generalization, induction};
use crate::ops_construction::{succ,implies};
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


    println!("\n\n");


    //prove 1 is the (left) multiplicative identity
    let t = &Formula::new("(S0*a)=a");
    let s0 = &Formula::new(AXIOMS[3]);
    let s1 = &specification(s0,a,one);
    let d0 = &Formula::new("Aa:(S0*a)=a");
    let d1 = &specification(d0, a, &succ(a));
    let s2 = &implies(d0,d1);
    let s3 = &specification(s2, a, a);
    let s4 = &generalization(s3, a);
    let s5 = &induction(t, a, s1, s4);

    println!("{}",s0.latex(0));
    println!("{}",s1.latex(0));
    println!("$begin supposition$\\\\");
    println!("{}",d0.latex(1));
    println!("{}",d1.latex(1));
    println!("$end supposition$\\\\");
    println!("{}",s2.latex(0));
    println!("{}",s3.latex(0));
    println!("{}",s4.latex(0));
    println!("{}",s5.latex(0))
}