use tnt::{logic_errors::LogicError};
use tnt::formula::Formula;

fn main() -> Result<(),LogicError> {
    println!("This example shows how decision problems can be presented as Formulas of TNT with free variables.");
    println!("To start with it is well known that \"a is even if and only if it is a multiple of two\". We can write this in TNT as follows.");
    let is_even = Formula::new("Eb:(b*SS0)=a");
    println!("\n{}\nwhich translates to\n{}\n",is_even,is_even.english());

    println!("Likewise to represent the idea that a is divisible by b we can use the following.");
    let is_factor = Formula::new("Ec:(b*c)=a");
    println!("\n{}\nwhich translates to\n{}\n",is_factor,is_factor.english());

    println!("We can slightly adapt this to represent primality.");
    let is_prime = Formula::new("Ab:Ac:[(SSb*c)=a>c=S0]");
    println!("\n{}\nwhich translates to\n{}\n",is_prime,is_prime.english());

    println!("A few more statements equivalent to some property of the variable a:");
    let is_pow_two = Formula::new("Ab:[Ec:(b*c)=a>Ed:(d*SS0)=b]");
    println!("\n{}\nwhich translates to\n{}\n",is_pow_two,is_pow_two.english());

    Ok(())
}