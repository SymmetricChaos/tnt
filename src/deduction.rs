
use crate::types::{Formula,Term};
use crate::ops_production::*;
use crate::ops_construction::{implies};
use lazy_static::lazy_static;

pub struct Deduction {
    depth: usize,
    tag_stack: Vec<usize>,
    parent: Option<Vec<Formula>>,
    title: String,
    axioms: Vec<Formula>,
    theorems: Vec<(Formula,String,usize,usize)>, // Forumla, comment, current depth, tag for supposition block
}

// When 'true' forces the theorems to be printed every time they are added, helps with debugging
const NOISY: bool = false;

lazy_static! {
    pub static ref PEANO: Vec<Formula> = {
        let mut m = Vec::new();
        m.push(Formula::new("Aa:~Sa=0"));
        m.push(Formula::new("Aa:(a+0)=a"));
        m.push(Formula::new("Aa:Ab:(a+Sb)=S(a+b)"));
        m.push(Formula::new("Aa:(a*0)=0"));
        m.push(Formula::new("Aa:Ab:(a*Sb)=((a*b)+a)"));
        m
    };
}


impl Deduction {
    pub fn new(title: &str, axioms: Vec<Formula>) -> Deduction {
        Deduction{ depth: 0, tag_stack: vec![0], parent: None, title: title.to_string(), axioms, theorems: Vec::<(Formula,String,usize,usize)>::new()}
    }


    // Internal methods
    fn get_theorem(&self, n: usize) -> &Formula {
        let t = &self.theorems[n];
        if t.2 > self.depth && n < *self.tag_stack.last().unwrap() {
            panic!("Cannot get theorem from within a higher supposition")
        }
        &t.0
    }

    fn get_last_theorem(&self) -> &Formula {
        let t = &self.theorems.last().unwrap();
        &t.0
    }

    fn push_new(&mut self, theorem: Formula, comment: &str) {
        if NOISY { 
            println!("{}",theorem)
        }
        self.theorems.push( (theorem,comment.to_string(),self.depth,*self.tag_stack.last().unwrap()) );
    }


    // Access methods
    pub fn theorem(&self, n: usize) -> &Formula {
        &self.theorems[n].0
    }

    pub fn theorems(&self) -> Vec<Formula> {
        let mut out: Vec<Formula> = Vec::new();
        for row in self.theorems.clone() {
            out.push(row.0)
        }
        out
    }

    pub fn theorems_raw(&self) -> Vec<(Formula, String, usize, usize)> {
        self.theorems.clone()
    }


    // Printing methods
    pub fn quick_print(&self) {
        println!("{}",self.title);
        for t in self.theorems.iter() {
            if t.1 != "" {
                println!("{} #{}",t.0,t.1)
            } else {
                println!("{}",t.0)
            }
        }
    }

    pub fn pretty_print(&self) {
        let mut prev_depth = 0;
        for (pos,t) in self.theorems.iter().enumerate() {
            if t.2 > prev_depth {
                println!("{}begin supposition","   ".repeat(prev_depth));
            } else if t.2 < prev_depth {
                println!("{}end supposition","   ".repeat(t.2));
            } else {
            }
            println!("{}{}) {}", "   ".repeat(t.2), pos, t.0.to_string());
            prev_depth = t.2;
        }
    }

    pub fn latex_print(&self) {
        println!("\\chapter{{{}}}",self.title);
        println!("\\begin{{align*}}");
        let mut prev_depth = 0;
        for (pos,t) in self.theorems.iter().enumerate() {
            if t.2 > prev_depth {
                println!("&{}\\text{{begin supposition}}&\\\\","   ".repeat(prev_depth));
            } else if t.2 < prev_depth {
                println!("&{}\\text{{end supposition}}&\\\\","   ".repeat(t.2));
            } else {
            }

            if t.1 != "" {
                println!("&{}) {}\\hspace{{1em}}&\\text{{[{}]}}\\\\",pos,t.0.latex(t.2),t.1);
            } else {
                println!("&{}) {}&\\\\",pos,t.0.latex(t.2));
            }
            prev_depth = t.2;
        }
        println!("\\end{{align*}}");
        println!("\\text{{{}}}",self.get_last_theorem().english())
    }

    // Logical methods
    pub fn add_premise(&mut self, premise: Formula, comment: &str) {
        if self.depth == 0 {
            if !self.axioms.contains(&premise) {
                panic!("At depth 0 only an axiom can be taken as a premise.")
            }
        }
        self.push_new( premise, comment );
    }

    pub fn specification(&mut self, n: usize, var: &Term, replacement: &Term, comment: &str) {
        let t = specification(self.get_theorem(n), &var, &replacement);
        self.push_new( t, comment );
    }

    pub fn generalization(&mut self, n: usize, var: &Term, comment: &str) {
        let t = generalization(self.get_theorem(n), &var);
        self.push_new( t, comment );
    }

    pub fn existence(&mut self, n: usize, term: &Term, var: &Term, comment: &str) {
        let t = existence(&self.theorems[n].0.clone() , &term, &var);
        self.push_new( t, comment );
    }

    pub fn successor(&mut self, n: usize, comment: &str) {
        let t = successor(self.get_theorem(n));
        self.push_new( t, comment );
    }

    pub fn predecessor(&mut self, n: usize, comment: &str) {
        let t = predecessor(self.get_theorem(n));
        self.push_new( t, comment );
    }

    pub fn interchange_ea(&mut self, n: usize, v: &Term, pos: usize, comment: &str) {
        let t = interchange_ea(self.get_theorem(n), v, pos);
        self.push_new( t, comment );
    }

    pub fn interchange_ae(&mut self, n: usize, v: &Term, pos: usize, comment: &str) {
        let t = interchange_ae(self.get_theorem(n), v, pos);
        self.push_new( t, comment );
    }
    
    pub fn supposition(&mut self, premise: Formula, comment: &str) {
        self.depth += 1;
        self.tag_stack.push(self.theorems.len());
        self.push_new( premise, comment );
    }

    pub fn implication(&mut self, comment: &str) {
        self.depth -= 1;
        let first_premise = self.tag_stack.pop().unwrap();
        let t = implies(self.get_theorem(first_premise), self.get_last_theorem());
        self.push_new( t, comment );
    }

    pub fn induction(&mut self, theorem: &Formula, var: &Term, base: usize, general: usize, comment: &str) {
        let t = induction(theorem,var,self.get_theorem(base),self.get_theorem(general));
        self.push_new( t, comment );
    }
    
}