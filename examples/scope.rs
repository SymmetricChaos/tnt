use tnt::{logic_errors::LogicError, formula::Formula};
use tnt::terms::{Variable,Number,Term};
use tnt::deduction::Deduction;
use tnt::axioms::PEANO;

fn main() -> Result<(),LogicError> {

    let a = &Variable::new("a");
    let zero = &Number::zero();

    let mut e = Deduction::new("Show Scope Error", PEANO.clone());
    e.add_axiom(&PEANO[3], "")?;
    e.specification(0, a, zero, "")?;
    e.supposition(Formula::new("Aa:a=Sa"), "")?;
    e.specification(2, a, zero, "")?;
    e.symmetry(3, "")?;
    e.implication("")?;
    e.supposition(Formula::new("0=SSb"), "")?;
    e.transitivity(1,6, "")?;
    e.transitivity(4,6, "")?;

    Ok(())
}