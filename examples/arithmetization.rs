use std::convert::TryFrom;

use tnt::{LogicError, Term};

fn main() -> Result<(), LogicError> {
    println!("Every Term and Formula can be turned into a single BigUint which can (in principle) be represented in TNT. This arithmetization is simply done by writing out the austere representation of the type and then reinterpreting those an numbers according to their ASCII codes.");
    println!("Deductions can be also be arithmetized by a similar process. Each Formula is arithmetized and they are appended together seperated by the byte 00100000 (the ASCCI space).");
    // println!("Below are examples of Formulas and Terms being arithmetized.");

    // let f0 = Formula::try_from("Ax:x=x").expect("invalid formula");
    // let n0 = f0.arithmetize();
    // let d0 = Formula::dearithmetize(&n0);

    println!("Consider the Term S(z+Sb). It can be converted to a number using to .arithmetize() method to get a number that represents its form. Here is the result:");
    let t0 = Term::try_from("S(z+Sb)")?;
    let t0_n = t0.arithmetize();
    println!("{t0} -> {t0_n}");

    println!("Any Term with the same form will have the same number assigned to it.");
    let t1 = Term::try_from("S(m''+Sy)")?;
    let t1_n = t1.arithmetize();
    println!("{t1} -> {t1_n}");

    Ok(())
}
