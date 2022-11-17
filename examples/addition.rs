use tnt::{Deduction, LogicError, Term};

fn main() -> Result<(), LogicError> {
    let a = "a";
    let b = "b";
    let zero = Term::Zero;
    let one = Term::one();

    let mut d = Deduction::new("One Plus One Equals Two");
    d.add_axiom(2)?;
    d.specification(0, a, &one)?;
    d.specification(1, b, &zero)?;
    d.add_axiom(1)?;
    d.specification(3, a, &one)?;
    d.successor(4)?;
    d.transitivity(2, 5)?;

    println!("{}", d.pretty_string());

    //println!("{}", d.arithmetize());

    match d.latex_file("addition") {
        Ok(_) => println!("\nSuccessfully created .tex file!"),
        Err(w) => println!("\nError: {}", w),
    };

    Ok(())
}
