#![allow(dead_code)]

mod types;
mod properties;
mod ops_production;
mod ops_construction;
mod string_manip;
mod deduction;
mod translate;

use crate::types::{Term, Formula};
use crate::deduction::{Deduction};


fn main() {

    // The Peano Axioms
    let axioms = vec![Formula::new("Aa:~Sa=0"),
                              Formula::new("Aa:(a+0)=a"),
                              Formula::new("Aa:Ab:(a+Sb)=S(a+b)"),
                              Formula::new( "Aa:(a*0)=0"),
                              Formula::new("Aa:Ab:(a*Sb)=((a*b)+a)")];


    let a = &Term::new("a");
    let sa = &Term::new("Sa");
    let zero = &Term::new("0");
    let one = &Term::new("S0");
    let zpz = &Term::new("(0+0)");

    let t1 = &Formula::new("Ea:a=0"); //The theorem to be proven
    let mut d = Deduction::new("Prove 0 is a Natural Number");
    d.add_premise(axioms[1].clone(), "");
    d.specification(0, a, zero, "");
    d.existence(1, zpz, a, "");
    assert_eq!(d.theorem(2),t1);
    d.pretty_print();

    println!("\n\n");

    let t2 = &Formula::new("Aa:(S0*a)=a"); //The theorem to be proven
    let mut e = Deduction::new("Prove 1 is the Left Multiplicative Identity");
    e.add_premise(axioms[3].clone(), "");
    e.specification(0, a, one, "");
    e.supposition(Formula::new("Aa:(S0*a)=a"), "");
    e.specification(2, a, sa, "comment");
    e.implication("");
    e.specification(4, a, a, "");
    e.generalization(5, a, "");
    e.induction(&Formula::new("(S0*a)=a"), a, 1, 6, "");
    assert_eq!(e.theorem(7),t2);
    e.pretty_print();
    e.latex_print();
}