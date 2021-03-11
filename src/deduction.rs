
use crate::types::{Formula,Term};
use crate::ops_production::{specification,generalization,existence,successor,predecessor};

// The Peano Axioms
pub const AXIOMS: [&str;5] = ["Aa:~Sa=0",
                            "Aa:(a+0)=a",
                            "Aa:Ab:(a+Sb)=S(a+b)",
                            "Aa:(a*0)=0",
                            "Aa:Ab:(a⋅Sb)=((a*b)+a))"];

pub struct Deduction {
    depth: usize,
    parent: Option<Vec<Formula>>,
    title: String,
    theorems: Vec<Formula>,
    descriptions: Vec<String>,
}



impl Deduction {
    pub fn new(title: &str) -> Deduction {
        Deduction{ depth: 0, parent: None, title: title.to_string(), theorems: Vec::<Formula>::new(), descriptions: Vec::<String>::new() }
    }

    pub fn quick_print(&self) {
        for t in self.theorems.iter() {
            println!("{}",t)
        }
    }

    pub fn add_premise(&mut self, premise: Formula, comment: &str) {
        self.theorems.push(premise);
        self.descriptions.push(comment.to_string());
    }

    pub fn specification(&mut self, n: usize, var: &Term, replacement: &Term, comment: &str) {
        let t = specification(&self.theorems[n].clone(), &var, &replacement);
        self.theorems.push(t);
        self.descriptions.push(comment.to_string());
    }

    pub fn generalization(&mut self, n: usize, var: &Term, comment: &str) {
        let t = generalization(&self.theorems[n].clone(), &var);
        self.theorems.push(t);
        self.descriptions.push(comment.to_string());
    }

    pub fn existence(&mut self, n: usize, term: &Term, var: &Term, comment: &str) {
        let t = existence(&self.theorems[n].clone() , &term, &var);
        self.theorems.push(t);
        self.descriptions.push(comment.to_string());
    }

    pub fn successor(&mut self, n: usize, comment: &str) {
        let t = successor(&self.theorems[n].clone());
        self.theorems.push(t);
        self.descriptions.push(comment.to_string());
    }

    pub fn predecessor(&mut self, n: usize, comment: &str) {
        let t = predecessor(&self.theorems[n].clone());
        self.theorems.push(t);
        self.descriptions.push(comment.to_string());
    }

    /* 
    fn supposition(&self, premise: TNT, title: &str, comment: &str) -> Deduction {
        let d = Deduction{ depth: self.depth+1,
                                    parent: Some(self.theorems.clone()),
                                    title: title.to_string(),
                                    theorems: Vec::<TNT>::new(),
                                    descriptions: Vec::<String>::new()};
    
        d
    }
    */
}