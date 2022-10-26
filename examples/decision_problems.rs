use std::convert::TryFrom;

use tnt::formula::Formula;
use tnt::logic_errors::LogicError;

fn main() -> Result<(), LogicError> {
    println!("This example shows how decision problems can be presented as Formulas of TNT with free variables.");
    println!("To start with it is well known that \"a is even if and only if it is a multiple of two\". We can write this in TNT as follows.");
    let is_even = Formula::try_from("Eb:(b*SS0)=a").expect("invalid formula");
    println!(
        "\n{}\nwhich translates to\n{}\n",
        is_even,
        is_even.to_english()
    );

    println!("Likewise to represent the idea that a is divisible by b we can use the following.");
    let is_factor = Formula::try_from("Ec:(b*c)=a").expect("invalid formula");
    println!(
        "\n{}\nwhich translates to\n{}\n",
        is_factor,
        is_factor.to_english()
    );

    println!("We can slightly adapt this to represent primality.");
    let is_prime = Formula::try_from("Ab:Ac:[(SSb*c)=a>c=S0]").expect("invalid formula");
    println!(
        "\n{}\nwhich translates to\n{}\n",
        is_prime,
        is_prime.to_english()
    );

    println!("A few more statements equivalent to some property of the variable a:");
    let is_pow_two = Formula::try_from("Ab:[Ec:(b*c)=a>Ed:(d*SS0)=b]").expect("invalid formula");
    let is_square = Formula::try_from("Eb:(b*b)=a").expect("invalid formula");
    let is_factorial = Formula::try_from("Ab:[~Ec:(b+Sc)>Ed:(b*d)=a]").expect("invalid formula");
    println!("\n{}", is_pow_two);
    println!("\n{}", is_square);
    println!("\n{}", is_factorial);

    Ok(())
}
