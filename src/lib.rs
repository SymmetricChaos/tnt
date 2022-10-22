//pub mod deduction;
pub mod formula;
pub use formula::{and, eq, exists, forall, implies, not, or, Formula, PEANO};
pub mod term;
pub use term::{prod, succ, sum, Term, ONE, ZERO};
//mod properties;
//pub mod axioms;
pub mod logic_errors;
pub mod production;
//mod translate;

pub mod parsing;
use parsing::parser::{string_to_formula, string_to_term};

#[macro_use]
extern crate pest_derive;
