use tnt::formula::Formula;

fn main() {
    println!("A well-formed formula of TNT is a statement that makes a logical assertion. To represent these we use the Formula enum. Its variables are Simple and Complex.");
    
    println!("\n\n\nA Formula::Simple is just two Expressions with `=` between them.");

    println!("\nSome random Formula::Simple");
    for _ in 0..3 {
        let mut f = Formula::random_simple();
        while f.to_string().len() > 35 {
            f = Formula::random_open();
        }
        print!("{}  ",f);
    }

    println!("\n\n\nA Formula::Complex can have many additional features. To start with any two Formulas in brackets `[]` and seperated by one of `&`, `|`, or `>` is also a Formula. Further any Formula prepended by `~` is also a Formula. Additionally a Formula::Complex may be prepended by `E` or `A` followed by a Variable and then `:`.");

    println!("\nSome random Formula::Complex without quantifications");
    for _ in 0..3 {
        let mut f = Formula::random_open().to_string();
        while f.len() > 25 || !f.contains("[") {
            f = Formula::random_open().to_string();
        }
        print!("{}  ",f);
    }

    println!("The intro continues in: cargo example --intro_deductions")
}