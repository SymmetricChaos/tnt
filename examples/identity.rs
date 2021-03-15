use tnt::types::{Term, Formula};
use tnt::deduction::{Deduction,PEANO};

fn main() {

    let a = &Term::new("a");
    let sa = &Term::new("Sa");
    let one = &Term::new("S0");

    let t = &Formula::new("Aa:(S0*a)=a"); //The theorem to be proven
    let mut e = Deduction::new("Prove 1 is the Left Multiplicative Identity", PEANO.clone());
    e.add_premise(PEANO[3].clone(), "axiom of absorbtion");
    e.specification(0, a, one, "specification of 0, base case");
    e.supposition(Formula::new("Aa:(S0*a)=a"), "assume desired theorem");
    e.specification(2, a, sa, "specification of 2");
    e.implication("implication of supposition block");
    e.specification(4, a, a, "specification of 4");
    e.generalization(5, a, "generalization of 5, general case");
    e.induction(&Formula::new("(S0*a)=a"), a, 1, 6, "induction on 1 and 6");
    assert_eq!(e.theorem(7),t);
    e.pretty_print();

    match e.latex_file("identity") {
        Ok(_) => println!("Successfully created .tex file!"),
        Err(w) => println!("Error: {}",w)
    };
}