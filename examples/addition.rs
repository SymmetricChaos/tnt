use tnt::{Deduction, LogicError, ONE, PEANO, ZERO};

fn main() -> Result<(), LogicError> {
    let a = "a";
    let b = "b";

    let mut d = Deduction::peano("One Plus One Equals Two");
    d.add_axiom(&PEANO[2])?;
    d.specification(0, a, &ONE)?;
    d.specification(1, b, &ZERO)?;
    d.add_axiom(&PEANO[1])?;
    d.specification(3, a, &ONE)?;
    d.successor(4)?;
    d.transitivity(2, 5)?;

    d.pretty_print();

    println!("{}", d.arithmetize());

    match d.latex_file("addition") {
        Ok(_) => println!("\nSuccessfully created .tex file!"),
        Err(w) => println!("\nError: {}", w),
    };

    Ok(())
}
