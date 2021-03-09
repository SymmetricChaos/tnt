
use crate::types::{TNT};

// The Peano Axioms
pub const AXIOMS: [&str;5] = ["Aa:~Sa=0",
                            "Aa:(a+0)=a",
                            "Aa:Ab:(a+Sb)=S(a+b)",
                            "Aa:(a*0)=0",
                            "Aa:Ab:(a⋅Sb)=((a*b)+a))"];

pub struct Deduction {
    depth: usize,
    reality: Option<usize>,
    title: String,
    theorems: Vec<TNT>,
    descriptions: Vec<String>
}



impl Deduction  {
    fn new(title: String) -> Deduction {
        Deduction{ depth: 0, reality: None, title, theorems: Vec::<TNT>::new(), descriptions: Vec::<String>::new() }
    }

}