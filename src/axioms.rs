use lazy_static::lazy_static;

use crate::types::Formula;

lazy_static! {
    /// The Peano Axioms
    pub static ref PEANO: Vec<Formula> = {
        let mut m = Vec::new();
        m.push(Formula::new("Aa:~Sa=0"));
        m.push(Formula::new("Aa:(a+0)=a"));
        m.push(Formula::new("Aa:Ab:(a+Sb)=S(a+b)"));
        m.push(Formula::new("Aa:(a*0)=0"));
        m.push(Formula::new("Aa:Ab:(a*Sb)=((a*b)+a)"));
        m
    };
}