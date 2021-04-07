use tnt::{logic_errors::LogicError, formula::Formula};
use tnt::terms::{Term,Variable,Number};
use tnt::deduction::Deduction;
use tnt::axioms::PEANO;

fn main() -> Result<(),LogicError> {

    let a = &Variable::new("a");
    let b = &Variable::new("b");
    let c = &Variable::new("c");
    let d = &Variable::new("d");
    let sc = &c.succ();
    let sd = &d.succ();
    let zero = &Number::new("0");

    let t = &Formula::new("Ad:Ac:(c+d)=(d+c)");
    let mut e = Deduction::new("Prove That Addition Commutes", PEANO.clone());
    e.add_axiom(&PEANO[2])?;
    e.specification(0, a, d)?;
    e.specification(1, b,sc)?;
    e.specification(0, a, sd)?;
    e.specification(3, b, c)?;
    e.symmetry(4)?;

    e.supposition(Formula::new("Ad:(d+Sc)=(Sd+c)"))?;
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
    e.specification(15,a, sd)?;
    e.symmetry(19)?;
    e.transitivity(18, 20)?;
    e.generalization(21, d)?;
    e.induction( c, 22, 13)?;
    e.specification(0, a, c)?;
    e.specification( 24, b, d)?;
    e.specification(0, a, d)?;
    e.specification( 26, b, c)?;
    e.symmetry(27)?;
    e.specification(23, c, c)?;
    e.specification(29, d, d)?;

    e.supposition(Formula::new("Ac:(c+d)=(d+c)"))?;
    e.specification(31, c, c)?;
    e.successor(32)?;
    e.transitivity(25, 33)?;
    e.transitivity(34, 28)?;
    e.transitivity(35, 30)?;
    e.generalization(36, c)?;
    e.implication()?;

    e.generalization(38, d)?;
    e.specification(15,a, c)?;
    e.specification(0, a, zero)?;
    e.specification(41, b,b)?;

    e.supposition(Formula::new("(0+b)=b"))?;
    e.successor(43)?;
    e.transitivity(42, 44)?;
    e.implication()?;

    e.generalization(46, b)?;
    e.specification(15, a, zero)?;
    e.induction( b, 48, 47)?;
    e.specification(49, b, c)?;
    e.symmetry(50)?;
    e.transitivity(40, 51)?;
    e.generalization(52, c)?;
    e.induction( d, 53, 39)?;

    assert_eq!(e.last_theorem(),t);

    e.pretty_print();

    match e.latex_file_annotated("commutativity") {
        Ok(_) => println!("\nSuccessfully created .tex file!"),
        Err(w) => println!("\nError: {}",w)
    };

    Ok(())
}