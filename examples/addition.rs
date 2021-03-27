use tnt::{errors::LogicError, types::{Term, Variable, Number}};
use tnt::deduction::Deduction;
use tnt::axioms::PEANO;

fn main() -> Result<(),LogicError> {

    let a = &Variable::new("a");
    let b = &Variable::new("b");
    let zero = &Number::zero();
    let one = &Number::one();

    let mut d = Deduction::new("One Plus One Equals Two", PEANO.clone());
    d.add_axiom(PEANO[2].clone(), "")?;
    d.specification(0, a, one, "")?;
    d.specification(1, b, zero, "")?;
    d.add_axiom(PEANO[1].clone(), "")?;
    d.specification(3, a, one, "")?;
    d.successor(4, "")?;
    d.transitivity(2,5,"")?;


    d.pretty_print();
    match d.latex_file("addition") {
        Ok(_) => println!("\nSuccessfully created .tex file!"),
        Err(w) => println!("\nError: {}",w)
    };

    Ok(())
}