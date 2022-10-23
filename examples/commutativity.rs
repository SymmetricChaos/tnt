use std::convert::TryFrom;

use tnt::{Deduction, Formula, LogicError, Term, PEANO, ZERO};

fn main() -> Result<(), LogicError> {
    let a = &Term::var("a");
    let b = &Term::var("b");
    let c = &Term::var("c");
    let d = &Term::var("d");
    let sc = &Term::try_from("Sc").expect("invalid term");
    let sd = &Term::try_from("Sd").expect("invalid term");
    let zero = &ZERO;

    let t = &Formula::try_from("Ad:Ac:(c+d)=(d+c)").expect("invalid formula");
    let mut e = Deduction::new("Prove That Addition Commutes", PEANO.clone());
    e.add_axiom(&PEANO[2])?;
    e.specification(0, a, d)?;
    e.specification(1, b, sc)?;
    e.specification(0, a, sd)?;
    e.specification(3, b, c)?;
    e.symmetry(4)?;

    e.supposition(Formula::try_from("Ad:(d+Sc)=(Sd+c)").expect("invalid formula"))?;
    e.specification(6, d, d)?;
    e.successor(7)?;
    e.transitivity(2, 8)?;
    e.transitivity(9, 5)?;
    e.generalization(10, d)?;
    e.implication()?;

    e.generalization(12, c)?;
    e.specification(1, b, zero)?;
    e.add_axiom(&PEANO[1])?;
    e.specification(15, a, d)?;
    e.successor(16)?;
    e.transitivity(14, 17)?;
    e.specification(15, a, sd)?;
    e.symmetry(19)?;
    e.transitivity(18, 20)?;
    e.generalization(21, d)?;
    e.induction(c, 22, 13)?;
    e.specification(0, a, c)?;
    e.specification(24, b, d)?;
    e.specification(0, a, d)?;
    e.specification(26, b, c)?;
    e.symmetry(27)?;
    e.specification(23, c, c)?;
    e.specification(29, d, d)?;

    e.supposition(Formula::try_from("Ac:(c+d)=(d+c)").expect("invalid formula"))?;
    e.specification(31, c, c)?;
    e.successor(32)?;
    e.transitivity(25, 33)?;
    e.transitivity(34, 28)?;
    e.transitivity(35, 30)?;
    e.generalization(36, c)?;
    e.implication()?;

    e.generalization(38, d)?;
    e.specification(15, a, c)?;
    e.specification(0, a, zero)?;
    e.specification(41, b, b)?;

    e.supposition(Formula::try_from("(0+b)=b").expect("invalid formula"))?;
    e.successor(43)?;
    e.transitivity(42, 44)?;
    e.implication()?;

    e.generalization(46, b)?;
    e.specification(15, a, zero)?;
    e.induction(b, 48, 47)?;
    e.specification(49, b, c)?;
    e.symmetry(50)?;
    e.transitivity(40, 51)?;
    e.generalization(52, c)?;
    e.induction(d, 53, 39)?;

    assert_eq!(e.last_theorem().formula, *t);

    e.pretty_print();

    match e.latex_file_annotated("commutativity") {
        Ok(_) => println!("\nSuccessfully created .tex file!"),
        Err(w) => println!("\nError: {}", w),
    };

    Ok(())
}
