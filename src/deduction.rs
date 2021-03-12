
use crate::types::{Formula,Term};
use crate::ops_production::{specification,generalization,existence,successor,predecessor,induction};
use crate::ops_construction::{implies};

// The Peano Axioms
pub const AXIOMS: [&str;5] = ["Aa:~Sa=0",
                            "Aa:(a+0)=a",
                            "Aa:Ab:(a+Sb)=S(a+b)",
                            "Aa:(a*0)=0",
                            "Aa:Ab:(a⋅Sb)=((a*b)+a))"];

pub struct Deduction {
    depth: usize,
    tag_stack: Vec<usize>,
    parent: Option<Vec<Formula>>,
    title: String,
    theorems: Vec<(Formula,String,usize,usize)>, // Forumla, comment, current depth, tag for supposition block
}



impl Deduction {
    pub fn new(title: &str) -> Deduction {
        Deduction{ depth: 0, tag_stack: vec![0], parent: None, title: title.to_string(), theorems: Vec::<(Formula,String,usize,usize)>::new() }
    }

    pub fn quick_print(&self) {
        println!("{}",self.title);
        for t in self.theorems.iter() {
            println!("{}",t.0)
        }
    }

    pub fn pretty_print(&self) {
        for (pos,t) in self.theorems.iter().enumerate() {
            println!("{}) {}{}",pos,"   ".repeat(t.2), t.0.to_string());
        }
    }

    pub fn latex_print(&self) {
        for t in self.theorems.iter() {
            println!("{}",t.0.latex(t.2));
        }
    }

    pub fn add_premise(&mut self, premise: Formula, comment: &str) {
        self.theorems.push( (premise,comment.to_string(),self.depth,*self.tag_stack.last().unwrap()) );
    }

    pub fn specification(&mut self, n: usize, var: &Term, replacement: &Term, comment: &str) {
        let t = specification(&self.theorems[n].0.clone(), &var, &replacement);
        self.theorems.push( (t,comment.to_string(),self.depth,*self.tag_stack.last().unwrap()) );
    }

    pub fn generalization(&mut self, n: usize, var: &Term, comment: &str) {
        let t = generalization(&self.theorems[n].0.clone(), &var);
        self.theorems.push( (t,comment.to_string(),self.depth,*self.tag_stack.last().unwrap()) );
    }

    pub fn existence(&mut self, n: usize, term: &Term, var: &Term, comment: &str) {
        let t = existence(&self.theorems[n].0.clone() , &term, &var);
        self.theorems.push( (t,comment.to_string(),self.depth,*self.tag_stack.last().unwrap()) );
    }

    pub fn successor(&mut self, n: usize, comment: &str) {
        let t = successor(&self.theorems[n].0.clone());
        self.theorems.push( (t,comment.to_string(),self.depth,*self.tag_stack.last().unwrap()) );
    }

    pub fn predecessor(&mut self, n: usize, comment: &str) {
        let t = predecessor(&self.theorems[n].0.clone());
        self.theorems.push( (t,comment.to_string(),self.depth,*self.tag_stack.last().unwrap()) );
    }

    
    pub fn supposition(&mut self, premise: Formula, comment: &str) {
        self.depth += 1;
        self.tag_stack.push(self.theorems.len());
        self.theorems.push( (premise,comment.to_string(),self.depth,*self.tag_stack.last().unwrap()) );
    }
    
    pub fn implication(&mut self, comment: &str) {
        self.depth -= 1;
        let first_premise = self.tag_stack.pop().unwrap();
        let t= implies(&self.theorems[first_premise].0, &self.theorems.last().unwrap().0);
        self.theorems.push( (t,comment.to_string(),self.depth,*self.tag_stack.last().unwrap()) );
    }

    pub fn induction(&mut self, theorem: &Formula, var: &Term, base: usize, general: usize, comment: &str) {
        let t= induction(theorem,var,&self.theorems[base].0,&self.theorems[general].0);
        self.theorems.push( (t,comment.to_string(),self.depth,*self.tag_stack.last().unwrap()) );
    }
    
}