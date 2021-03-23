use tnt::types::{Variable,Number,Term};

fn main() {

    let a = &Variable::new("a");
    let b = &Variable::new("b");
    let zero = &Number::new("0");

    println!("Because all types that implement Term have an arithmetic interpretation they also implement addition and multiplication.");
    println!("a + b = {}",a + b);
    println!("&(a + b) + zero = {}",&(a+b)+zero);
}