//! Axioms for use with Deduction.

use lazy_static::lazy_static;

use crate::formula::Formula;

lazy_static! {
    /**  
    * These are the axiomatic statements of the TNT formal system, they don't align strictly with the Peano Axioms but they define the same arithmetic properties for addition and multiplication. The axioms are as follows:
    *
    * Aa:~Sa=0                for all a, it is false that (a + 1) is 0
    *
    * Aa:(a+0)=a              for all a, (a + 0) = a
    *
    * Aa:Ab:(a+Sb)=S(a+b)     for all a and b, (a + (b + 1)) = ((a + b) + 1)
    *
    * Aa:(a\*0) = 0            for all a, (a × 0) = 0
    *
    * Aa:Ab:(a\*Sb)=((a\*b)+a)  for all a and b, (a × (b + 1)) = ((a × b) + a)
    */
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