#![allow(dead_code)]

mod types;
mod properties;
mod operations;
mod string_manip;

use crate::operations::{add,eq,forall};
use crate::types::{number, variables};
// Nonstandard symbols used: ∀∃∧∨⊃·

fn main() {
    let vars = variables(vec!["a","b","c"]);
    let (a,b,c) = (&vars[0],&vars[1],&vars[2]);
    let zero = number(0);
    let one = number(1);
    

    println!("{}",one);
    println!("{}",add(a,b));
    println!("{}",add(a,&add(&zero,c)));
    println!("{}",eq(a,a));
    println!("{}",forall(a,&eq(a,a)));

}
