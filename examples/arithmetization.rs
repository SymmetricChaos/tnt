use tnt::formula::Formula;
use tnt::term::{Number, Term};

fn main() {
    println!("Every Term, Formula, and Deduction of tnt can be turned into a BigUint.");
    println!("Because only ASCII symbols are used and they are simply read as a sequence of bytes then provided to BigUint.");
    println!("In the specific case of arithmetizing a Deduction the byte '00100000' (the ASCII space) is used to seperate formulas.");
    println!("Below are examples of Formulas and Terms being arithmetized. To see a deductiction arithmetized try: cargo run --example identity\n\n");

    let f0 = Formula::new("Ax:x=x");
    let n0 = f0.arithmetize();
    let d0 = Formula::dearithmetize(&n0);

    let f1 = Number::new("SS0");
    let n1 = f1.arithmetize();
    let d1 = Number::dearithmetize(&n1);

    println!("{} -> {} -> {}", f0, n0, d0);
    println!("{} -> {} -> {}", f1, n1, d1);
}
