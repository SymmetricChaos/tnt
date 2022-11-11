pub mod deduction;
pub use deduction::Deduction;
pub mod formula;
pub use formula::{Formula, PEANO};
pub mod term;
pub use term::{Term, ONE, ZERO};
pub mod logic_errors;
pub use logic_errors::LogicError;
pub mod production;
pub use production::*;
pub mod parsing;

#[macro_use]
extern crate pest_derive;
