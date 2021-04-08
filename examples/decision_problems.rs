use tnt::{logic_errors::LogicError};
use tnt::formula::Formula;

fn main() -> Result<(),LogicError> {
    println!("This example shows how decision problems can be presented as Formulas of TNT with free variables.");
    println!("To start with it is well known that \"a is even if and only if it is a multiple of two\". We can write this in TNT as follows.");
    let is_even = Formula::new("Eb:(b*SS0)=a");
    println!("\n{}\nwhich translates to\n{}\n",is_even,is_even.english());

    println!("Not every decision problem has such an obvious form. Consider primality. To represent this we first need to capture what it means for one number to be divisible by another. This is not too hard.");
    let is_factor = Formula::new("Ec:(b*c)=a");
    println!("\n{}\nwhich translates to\n{}\n",is_factor,is_factor.english());

    

    Ok(())
}