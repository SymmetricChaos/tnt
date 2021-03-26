use tnt::{errors::LogicError, types::{Term, Formula, Variable, Number}};
use tnt::deduction::Deduction;
use tnt::axioms::PEANO;

fn main() -> Result<(),LogicError> {

    let a = &Variable::new("a");
    let b = &Variable::new("b");
    let c = &Variable::new("c");
    let d = &Variable::new("d");
    let sc = &(c << 1);
    let sd = &(d << 1);
    let zero = &Number::new("0");

    let t = &Formula::new("Ad:Ac:(c+d)=(d+c)");
    let mut e = Deduction::new("Prove That Addition Commutes", PEANO.clone());
    e.add_axiom(PEANO[2].clone(), "axiom");
    e.specification(0, a, d, "specification of 0, a replaced by d")?;
    e.specification(1, b,sc, "specification of 1, b replaced by Sc")?;
    e.specification(0, a, sd, "specification of 0, a replaced by Sd")?;
    e.specification(3, b, c, "specification of 3, b replaced by c")?;
    e.symmetry(4, "symmetry of 4");

    e.supposition(Formula::new("Ad:(d+Sc)=(Sd+c)"),"supposition");
    e.specification(6, d, d, "specification of 6, d replaced by d")?;
    e.successor(7, "successor of 7");
    e.transitivity(2, 8, "transitivity of 2 and 8");
    e.transitivity(9, 5, "transitivty of 9 and 5");
    e.generalization(10, d, "generalization of 10");
    e.implication("implication");

    e.generalization(12, c, "generalization of 12");
    e.specification(1, b, zero, "specification of 1, b replaced by 0")?;
    e.add_axiom(PEANO[1].clone(), "axiom");
    e.specification(15, a, d, "specification of 15, a replaced by d")?;
    e.successor(16, "successor of 16");
    e.transitivity(14, 17, "transitivity of 14 and 17");
    e.specification(15,a, sd, "specification of 15, a replaced by Sd")?;
    e.symmetry(19, "symmetry of 19");
    e.transitivity(18, 20, "transitivity of 18 and 20");
    e.generalization(21, d, "generalization of 21");
    e.induction( c, 22, 13, "induction of c on 22 and 13");
    e.specification(0, a, c, "specification of 0, a replaced by c")?;
    e.specification( 24, b, d, "specification of 24, b replaced by d")?;
    e.specification(0, a, d, "specification of 0, a replaced by d")?;
    e.specification( 26, b, c, "specification of 26, b replaced by c")?;
    e.symmetry(27, "symmetry of 27");
    e.specification(23, c, c, "specification of 23, c replaced by c")?;
    e.specification(29, d, d, "specification of 29, d replaced by d")?;

    e.supposition(Formula::new("Ac:(c+d)=(d+c)"), "supposition");
    e.specification(31, c, c, "specificationf of 31, c replaced by c")?;
    e.successor(32, "successor of 32");
    e.transitivity(25, 33, "transitivity of 25 and 33");
    e.transitivity(34, 28, "transitivity of 34 and 28");
    e.transitivity(35, 30, "transitivity of 35 and 30");
    e.generalization(36, c, "generalization of 36");
    e.implication("implication");

    e.generalization(38, d, "generalization of 38");
    e.specification(15,a, c, "specification of 15, a replaced by c")?;
    e.specification(0, a, zero, "specification of 0, a replaced by 0")?;
    e.specification(41, b,b, "specification of 41. b replaced by b")?;

    e.supposition(Formula::new("(0+b)=b"), "supposition");
    e.successor(43, "successor of 43");
    e.transitivity(42, 44, "transitivity of 42 and 44");
    e.implication("implication");

    e.generalization(46, b, "generalization of  46");
    e.specification(15, a, zero, "specification of 15, a replaced by 0")?;
    e.induction( b, 48, 47, "induction of b on 48 and 47");
    e.specification(49, b, c, "specification of 49, b replaced by c")?;
    e.symmetry(50, "symmetry of 50");
    e.transitivity(40, 51, "transitivity of 40 and 51");
    e.generalization(52, c, "generalization of 52");
    e.induction( d, 53, 39, "induction of d on 53 and 39");

    assert_eq!(e.last_theorem(),t);

    e.pretty_print();

    match e.latex_file("commutativity") {
        Ok(_) => println!("\nSuccessfully created .tex file!"),
        Err(w) => println!("\nError: {}",w)
    };

    Ok(())
}