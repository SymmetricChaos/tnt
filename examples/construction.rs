use tnt::types::{Term};
use tnt::ops_construction::*;

fn main() {

    let a = &Term::new("a");
    let b = &Term::new("b");
    let c = &Term::new("c");
    let f1 = forall(&a, &exists(b, &exists(c,&eq(&add(a,b),&mul(a,c)))));

    println!("Some valid statements of tnt:");
    println!("{}\n{}",f1,f1.english());


}