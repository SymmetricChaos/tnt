use tnt::{Deduction, LogicError, ONE, PEANO, ZERO};
fn main() -> Result<(), LogicError> {
    let a = "a";
    let b = "b";

    let mut d = Deduction::new("One Plus One Equals Two", PEANO.clone());
    d.add_axiom(&PEANO[2])?;
    d.specification(0, a, &ZERO)?;
    d.specification(1, b, &ZERO)?;
    d.add_axiom(&PEANO[1])?;
    d.specification(3, a, &ONE)?;
    d.successor(4)?;
    d.transitivity(2, 5)?;

    d.english();

    println!("{}", d.arithmetize());

    Ok(())
}
