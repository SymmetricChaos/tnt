use tnt::term::{Term, ZERO};

fn main() {
    let a = &Term::var("a");
    let b = &Term::var("b");

    println!("Because all types that implement Term have an arithmetic interpretation they also implement addition and multiplication.");
    println!(
        "    
let a = &Variable::new(\"a\");
let b = &Variable::new(\"b\");
let zero = &Number::zero();
"
    );
    println!("a + b = {:?}", a + b);
    println!("&(a + b) + zero = {:?}", &(a + b) + &ZERO);
}
