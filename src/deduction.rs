use std::{fs::File, io::Write};
use num::BigUint;

use crate::{string_manip::get_free_vars, types::{Formula,Term,Variable}};
use crate::ops_production::*;
use crate::ops_construction::implies;

/// The Deduction struct enforces valid use of deductive logic to produce proofs in Typographical Number Theory and output LaTeX formatted proofs.
pub struct Deduction {
    depth: usize,
    tag_stack: Vec<usize>,
    title: String,
    axioms: Vec<Formula>,
    theorems: Vec<(Formula,String,usize,usize)>, // Forumla, comment, current depth, start of current supposition
}

// When 'true' forces the theorems to be printed every time they are added, helps with debugging
const NOISY: bool = false;

impl Deduction {
    pub fn new(title: &str, axioms: Vec<Formula>) -> Deduction {
        Deduction{ depth: 0, tag_stack: vec![0], title: title.to_string(), axioms, theorems: Vec::<(Formula,String,usize,usize)>::new()}
    }


    // Internal methods
    // This is correct only because nested supposition is forbidden in the .supposition() method
    // To allow nested supposition we need to track scope somehow
    fn get_theorem(&self, n: usize) -> &Formula {
        &self.theorems[n].0
    }

    fn get_last_theorem(&self) -> &Formula {
        &self.theorems.last().unwrap().0
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

    pub fn last_theorem(&self) -> &Formula {
        &self.theorems.last().unwrap().0
    }

    pub fn all_theorems(&self) -> Vec<Formula> {
        let mut out: Vec<Formula> = Vec::new();
        for row in self.theorems.clone() {
            out.push(row.0)
        }
        out
    }

    pub fn all_theorems_raw(&self) -> Vec<(Formula, String, usize, usize)> {
        self.theorems.clone()
    }



    // Printing methods
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

    pub fn latex_file(&self, filename: &str) -> std::io::Result<()>{
        let filename = format!("{}.tex",filename);
        let mut file = File::create(filename)?;

        let section_title = format!("\\section*{{{}}}\n",self.title);
        //let translate_last = format!("\\text{{{}}}",self.get_last_theorem().english());

        file.write(b"\\documentclass[fleqn,11pt]{article}\n")?;
        file.write(b"\\usepackage{amsmath}\n")?;
        file.write(b"\\allowdisplaybreaks\n")?;
        file.write(b"\\begin{document}\n")?;
        file.write(&section_title.into_bytes())?;
        file.write(b"\\begin{flalign*}\n")?;

        let mut prev_depth = 0;
        for (pos,t) in self.theorems.iter().enumerate() {

            if t.2 > prev_depth {
                let line = format!("&{}\\text{{begin supposition}}&\\\\&\n","   ".repeat(prev_depth)).into_bytes();
                file.write(&line)?;
            } else if t.2 < prev_depth {
                let line = format!("&{}\\text{{end supposition}}&\\\\&\n","   ".repeat(t.2)).into_bytes();
                file.write(&line)?;
            }

            if t.1 != "" {
                let line = format!("&\\hspace{{{}em}}{})\\hspace{{1em}}{}\\hspace{{2em}}\\textbf{{[{}]}}\\\\&\n",t.2*2,pos,t.0.latex(),t.1).into_bytes();
                file.write(&line)?;
            } else {
                let line = format!("&\\hspace{{{}em}}{})\\hspace{{1em}}{}\\\\&\n",t.2*2,pos,t.0.latex()).into_bytes();
                file.write(&line)?;
            }

            prev_depth = t.2;
        }

        file.write(b"\\end{flalign*}\n")?;
        file.write(b"\\end{document}")?;
        Ok(())
    }

    pub fn arithmetize(&self) -> BigUint {
        let mut n: Vec<u8> = Vec::new();
        let mut th = self.all_theorems();
        th.reverse();
        for t in th {
            n.extend( t.to_string().into_bytes().iter() );
            n.push(32);
        }
        BigUint::from_bytes_be(&n)
    }

    // TODO: Dearithmetize? Probably needs to track more information


    
    // Logical methods
    pub fn add_axiom(&mut self, premise: Formula, comment: &str) {
        if self.axioms.contains(&premise) {
            self.push_new( premise, comment );
        } else {
            panic!("{} is not a known axiom", premise);
        }
    }

    pub fn specification<T: Term>(&mut self, n: usize, var: &Variable, replacement: &T, comment: &str) {
        let t = specification(self.get_theorem(n), &var, replacement);
        self.push_new( t, comment );
    }

    pub fn generalization(&mut self, n: usize, var: &Variable, comment: &str) {
        if self.depth != 0 {
            let f = get_free_vars(&self.get_theorem(*self.tag_stack.last().unwrap()).to_string());
            if f.contains(&var.to_string()) {
                panic!("Generalization Error: the variable {} is free in the supposition {}",var,self.get_theorem(*self.tag_stack.last().unwrap()))
            }
        }
        let t = generalization(self.get_theorem(n), &var);
        self.push_new( t, comment );
    }

    pub fn existence<T: Term>(&mut self, n: usize, term: &T, var: &Variable, comment: &str) {
        let t = existence(&self.theorems[n].0.clone(), term, &var);
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

    pub fn interchange_ea(&mut self, n: usize, v: &Variable, pos: usize, comment: &str) {
        let t = interchange_ea(self.get_theorem(n), v, pos);
        self.push_new( t, comment );
    }

    pub fn interchange_ae(&mut self, n: usize, v: &Variable, pos: usize, comment: &str) {
        let t = interchange_ae(self.get_theorem(n), v, pos);
        self.push_new( t, comment );
    }

    pub fn symmetry(&mut self, n: usize, comment: &str) {
        let t = symmetry(self.get_theorem(n));
        self.push_new( t, comment );
    }

    pub fn transitivity(&mut self, n1: usize, n2: usize, comment: &str) {
        let t = transitivity(self.get_theorem(n1), self.get_theorem(n2));
        self.push_new( t, comment );
    }
    
    pub fn supposition(&mut self, premise: Formula, comment: &str) {
        if self.depth == 1 {
            panic!("Nested supposition not currently supported")
        }
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

    pub fn induction(&mut self, var: &Variable, base: usize, general: usize, comment: &str) {
        let t = induction(var,self.get_theorem(base),self.get_theorem(general));
        self.push_new( t, comment );
    }
    
}