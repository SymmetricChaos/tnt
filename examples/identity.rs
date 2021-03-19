use tnt::types::{Term, Formula};
use tnt::deduction::{Deduction,PEANO};

fn main() {

    println!("A very simple Deduction showing that 1 is the left multiplicative identity");

    println!("
let a = &Term::new(\"a\");
let sa = &Term::new(\"Sa\");
let one = &Term::new(\"S0\");

let mut e = Deduction::new(\"Prove 1 is the Left Multiplicative Identity\", PEANO.clone());
e.add_axiom(PEANO[3].clone(), \"axiom of absorbtion\");
e.specification(0, a, one, \"specification of 0\");
e.supposition(Formula::new(\"Aa:(S0*a)=a\"), \"supposition\");
e.specification(2, a, sa, \"specification of 2\");
e.implication(\"implication of supposition block\");
e.specification(4, a, a, \"specification of 4\");
e.generalization(5, a, \"generalization of 5\");
e.induction(a, 1, 6, \"induction of a on 1 and 6\");
    ");

    let a = &Term::new("a");
    let sa = &Term::new("Sa");
    let one = &Term::new("S0");

    let t = &Formula::new("Aa:(S0*a)=a"); //The theorem to be proven
    let mut e = Deduction::new("Prove 1 is the Left Multiplicative Identity", PEANO.clone());
    e.add_axiom(PEANO[3].clone(), "axiom of absorbtion");
    e.specification(0, a, one, "specification of 0");
    e.supposition(Formula::new("Aa:(S0*a)=a"), "supposition");
    e.specification(2, a, sa, "specification of 2");
    e.implication("implication of supposition block");
    e.specification(4, a, a, "specification of 4");
    e.generalization(5, a, "generalization of 5");
    e.induction(a, 1, 6, "induction of a on 1 and 6");
    assert_eq!(e.theorem(7),t);
    
    println!("\n\nNow the pretty ASCII output");
    e.pretty_print();

    println!("\n\nA deduction can be arithmetized into a single (very large) number. In this case:");
    println!("{}",e.arithmetize());

}