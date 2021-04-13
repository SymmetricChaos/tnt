use tnt::formula::Formula;

fn main() {
    println!("A well-formed formula of TNT is a statement that makes a logical assertion. To represent these we use the Formula enum. Its variables are Simple and Complex.");
    
    println!("\n\n\nA Formula::Simple is just two Expressions with `=` between them.");

    println!("\nSome selected Formula::Simple");
    println!("{}",Formula::new("S0=0"));
    println!("{}",Formula::new("(SSSx*S0)=SS((0*SSS0)+Sc'')"));
    println!("{}",Formula::new("Sa=Sa'"));

    println!("\n\n\nA Formula::Complex can have many additional features. To start with any two Formulas in brackets `[]` and seperated by one of `&`, `|`, or `>` is also a Formula. Further any Formula prepended by `~` is also a Formula. Additionally a Formula::Complex may be prepended by `E` or `A` followed by a Variable and then `:`.");

    println!("\nSome selected Formula::Complex");
    println!("{}",Formula::new("Aa:Ea':Sa=Sa'"));

    println!("The intro continues in:  cargo run --example intro_deductions")
}