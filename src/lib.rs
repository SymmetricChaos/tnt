pub mod deduction;
pub use deduction::Deduction;
pub mod formula;
pub use formula::{and, eq, exists, forall, implies, not, or, Formula, PEANO};
pub mod term;
pub use term::{prod, succ, sum, Term, ONE, ZERO};
//mod properties;
//pub mod axioms;
pub mod logic_errors;
pub use logic_errors::LogicError;
pub mod production;
//mod translate;

pub mod parsing;

#[macro_use]
extern crate pest_derive;
