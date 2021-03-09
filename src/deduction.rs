
use crate::types::{TNT};

// The Peano Axioms
pub const AXIOMS: [&str;5] = ["∀a:~Sa=0",
                            "∀a:(a+0)=a",
                            "∀a:∀b:(a+Sb)=S(a+b)",
                            "∀a:(a⋅0)=0",
                            "∀a:∀b:(a⋅Sb)=((a⋅b)+a))"];

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