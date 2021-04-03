use tnt::{logic_errors::LogicError, types::{Term, Variable, Number}};
use tnt::deduction::Deduction;
use tnt::axioms::PEANO;

fn main() -> Result<(),LogicError> {

    let a = &Variable::new("a");
    let b = &Variable::new("b");
    let zero = &Number::zero();
    let one = &Number::one();

    let mut d = Deduction::new("One Plus One Equals Two", PEANO.clone());
    d.add_axiom(&PEANO[2], "")?;
    d.specification(0, a, one, "")?;
    d.specification(1, b, zero, "")?;
    d.add_axiom(&PEANO[1], "")?;
    d.specification(3, a, one, "")?;
    d.successor(4, "")?;
    d.transitivity(2,5,"")?;

    d.english();

    println!("{}",d.arithmetize());

    Ok(())
}