use tnt::types::{Term, Formula};
use tnt::deduction::{Deduction,PEANO};

fn main() {

    let a = &Term::new("a");
    let b = &Term::new("b");
    let c = &Term::new("c");
    let d = &Term::new("d");
    let sa = &Term::new("Sa");
    let sb = &Term::new("Sb");
    let sc = &Term::new("Sc");
    let sd = &Term::new("Sd");
    let one = &Term::new("S0");
    let zero = &Term::new("0");

    let t = &Formula::new("Ad:Ac:(c+d)=(d+c)"); //The theorem to be proven
    let mut e = Deduction::new("Prove That Addition Commutes", PEANO.clone());
    e.add_premise(PEANO[2].clone(), "axiom");
    e.specification(0, a, d, "");
    e.specification(1, b,sc, "");
    e.specification(0, a, sd, "");
    e.specification(3, b, c, "");
    e.symmetry(4, "");

    e.supposition(Formula::new("Ad:(d+Sc)=(Sd+c)"),"");
    e.specification(6, d, d, "");
    e.successor(7, "");
    e.transitivity(2, 8, "");
    e.transitivity(9, 5, "");
    e.generalization(10, d, "");
    e.implication("");

    e.generalization(12, c, "");
    e.specification(1, b, zero, "");
    e.add_premise(PEANO[1].clone(), "axiom");
    e.specification(15, a, d, "");
    e.successor(16, "");
    e.transitivity(14, 17, "");
    e.specification(15,a, sd, "");
    e.symmetry(19, "");
    e.transitivity(18, 20, "");
    e.generalization(21, d, "");
    e.induction(&Formula::new("Ad:(d+Sc)=(Sd+c)"), c, 22, 13, "");
    e.specification(0, a, c, "");
    e.specification( 24, b, d, "");
    e.specification(0, a, d, "");
    e.specification( 26, b, c, "");
    e.symmetry(27, "");
    e.specification(23, c, c, "");
    e.specification(29, d, d, "");

    e.supposition(Formula::new("Ac:(c+d)=(d+c)"), "");
    e.specification(31, c, c, "");
    e.successor(32, "");
    e.transitivity(25, 33, "");
    e.transitivity(34, 28, "");
    e.transitivity(35, 30, "");
    e.generalization(36, c, "");
    e.implication("");

    e.generalization(38, d, "");
    e.specification(15,a, c, "");
    e.specification(0, a, zero, "");
    e.specification(41, b,b, "");

    e.supposition(Formula::new("(0+b)=b"), "");
    e.successor(43, "");
    e.transitivity(42, 44, "");
    e.implication("");

    e.generalization(46, b, "");
    e.specification(15, a, zero, "");
    e.induction(&Formula::new("(0+b)=b"), b, 48, 47, "");
    e.specification(49, b, c, "");
    e.symmetry(50, "");
    e.transitivity(40, 51, "");
    e.generalization(52, c, "");
    e.induction(&Formula::new("Ac:(c+d)=(d+c)"), d, 53, 39, "");

    e.pretty_print();

    match e.latex_file("commutativity") {
        Ok(_) => println!("\nSuccessfully created .tex file!"),
        Err(w) => println!("\nError: {}",w)
    };
    
}