#![allow(dead_code)]

mod types;
mod properties;
mod ops_production;
mod ops_construction;
mod string_manip;
mod deduction;
mod translate;

use crate::types::{Term, Formula};
use crate::deduction::{Deduction,PEANO};
use crate::translate::{arithmetize,dearithmetize};

// symbols used: 0AESabcdefghijkmnopqrstuwxyz'~+*=&|()[]>: with # as a terminal character used only to arithmetize a proof

fn main() {

    let a = &Term::new("a");
    let sa = &Term::new("Sa");
    let zero = &Term::new("0");
    let one = &Term::new("S0");
    let zpz = &Term::new("(0+0)");

    let t1 = &Formula::new("Ea:a=0"); //The theorem to be proven
    let mut d = Deduction::new("Prove 0 is a Natural Number", PEANO.clone());
    d.add_premise(PEANO[1].clone(), "");
    d.specification(0, a, zero, "");
    d.existence(1, zpz, a, "");
    assert_eq!(d.theorem(2),t1);
    d.pretty_print();

    println!("\n\n");

    let t2 = &Formula::new("Aa:(S0*a)=a"); //The theorem to be proven
    let mut e = Deduction::new("Prove 1 is the Left Multiplicative Identity", PEANO.clone());
    e.add_premise(PEANO[3].clone(), "axiom of absorbtion");
    e.specification(0, a, one, "specification of 0, base case");
    e.supposition(Formula::new("Aa:(S0*a)=a"), "assume desired theorem");
    e.specification(2, a, sa, "specification of 2");
    e.implication("implication of supposition block");
    e.specification(4, a, a, "specification of 4");
    e.generalization(5, a, "generalization of 5, general case");
    e.induction(&Formula::new("(S0*a)=a"), a, 1, 6, "induction on 1 and 6");
    assert_eq!(e.theorem(7),t2);
    e.latex_print();

    let num = arithmetize(e.theorem(7).to_string());
    let word = dearithmetize(num.clone());
    println!("{}",num);
    println!("{}",word);
}