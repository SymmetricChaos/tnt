use std::{ fs::File, io::{Write,Error}};
use num::BigUint;

use crate::{string_manip::get_free_vars, types::Formula};
use crate::terms::{Term,Variable};
use crate::operations::production::*;
use crate::operations::construction::implies;
use crate::logic_errors::LogicError;

/// Information tracked about each Formula
#[derive(Clone)]
pub struct TheoremFrame {
    pub formula: Formula,
    pub comment: String,
    pub depth: usize,
    pub position: usize,
    pub annotation: String,
    pub rule_num: u8,
    pub scope: usize,
}


impl TheoremFrame {
    pub fn new(formula: Formula, comment: String, depth: usize, position: usize, annotation: String, rule_num: u8, scope: usize) -> TheoremFrame {
        TheoremFrame{ formula, comment, depth, position, annotation, rule_num, scope }
    }
}



/// Enforces valid use of deductive logic to produce proofs in Typographical Number Theory and outputs formatted results.
pub struct Deduction {
    index: usize,
    depth: usize,
    scope_stack: Vec<usize>,
    scope_cur: usize,
    title: String,
    axioms: Vec<Formula>,
    theorems: Vec<TheoremFrame>,
}

// When 'true' forces the theorems to be printed every time they are added, helps with debugging
const NOISY: bool = false;

impl Deduction {
    pub fn new(title: &str, axioms: Vec<Formula>) -> Deduction {
        Deduction{ index: 0, depth: 0, scope_stack: vec![0], scope_cur: 0, title: title.to_string(), axioms, theorems: Vec::<TheoremFrame>::new() }
    }


    // Internal methods
    // Get a theorem if it is in an accessible scope
    fn get_theorem(&self, n: usize) -> Result<&Formula,LogicError> {
        // Check the scope
        let tscope = self.theorems[n].scope;
        if tscope == self.scope_cur || self.scope_stack.contains(&tscope) {
            return Ok(&self.theorems[n].formula)
        }
        let msg = format!("Scope Error: position {} is not in an accessible scope", n);
        Err(LogicError::new(msg))
    }

    // The last theorem on the list is always in an accessible scope.
    fn get_last_theorem(&self) -> &Formula {
        &self.theorems.last().unwrap().formula
    }

    fn push_new(&mut self, theorem: Formula, comment: &str, annotation: String, rule_num: u8) {
        if NOISY {
            if rule_num == 10 {
                println!("{}begin supposition","   ".repeat(self.depth-1))
            } else if rule_num == 11 {
                println!("{}end supposition","   ".repeat(self.depth))
            }
            println!("{}{}) {} [{}]","   ".repeat(self.depth),self.index,theorem,annotation)
        }
        let t = TheoremFrame{ formula: theorem, 
                                          comment: comment.to_string(), 
                                          depth: self.depth, 
                                          position: self.index,
                                          annotation,
                                          rule_num,
                                          scope: self.scope_cur,
                                        };
        self.theorems.push( t );
        self.index += 1;
    }



    /// Return the Formula at index n.
    pub fn theorem(&self, n: usize) -> &Formula {
        &self.theorems[n].formula
    }

    /// Return the last Formula.
    pub fn last_theorem(&self) -> &Formula {
        &self.theorems.last().unwrap().formula
    }

    /// Return a vector of all the Formulas in the Deduction.
    pub fn all_theorems(&self) -> Vec<Formula> {
        let mut out: Vec<Formula> = Vec::new();
        for row in self.theorems.clone() {
            out.push(row.formula)
        }
        out
    }

    /// Dump the entire vector of tuples with all the attached information.
    pub fn all_theorems_raw(&self) -> Vec<TheoremFrame> {
        self.theorems.clone()
    }



    /// Print the Deduction as a nicely formatted as an ASCII representation.
    pub fn pretty_print(&self) {
        let mut prev_depth = 0;
        for (pos,t) in self.theorems.iter().enumerate() {
            if t.depth > prev_depth {
                println!("{}begin supposition","   ".repeat(prev_depth));
            } else if t.depth < prev_depth {
                println!("{}end supposition","   ".repeat(t.depth));
            } else {
            }
            println!("{}{}) {}", "   ".repeat(t.depth), pos, t.formula.to_string());
            prev_depth = t.depth;
        }
    }

    /// Create a LaTeX file the given file name that displays the Deduction.
    pub fn latex_file(&self, filename: &str) -> Result<(), Error> {
        let filename = format!("{}.tex",filename);
        let mut file = File::create(filename)?;

        let section_title = format!("\\section*{{{}}}\n",self.title);

        file.write(b"\\documentclass[fleqn,11pt]{article}\n")?;
        file.write(b"\\usepackage{amsmath}\n")?;
        file.write(b"\\allowdisplaybreaks\n")?;
        file.write(b"\\begin{document}\n")?;
        file.write(&section_title.into_bytes())?;
        file.write(b"\\begin{flalign*}\n")?;

        let mut prev_depth = 0;
        for (pos,t) in self.theorems.iter().enumerate() {

            if t.depth > prev_depth {
                let line = format!("&{}\\text{{begin supposition}}&\\\\\n","   ".repeat(prev_depth)).into_bytes();
                file.write(&line)?;
            } else if t.depth < prev_depth {
                let line = format!("&{}\\text{{end supposition}}&\\\\\n","   ".repeat(t.depth)).into_bytes();
                file.write(&line)?;
            }

            if t.comment != "" {
                let line = format!("&\\hspace{{{}em}}{})\\hspace{{1em}}{}\\hspace{{2em}}\\textbf{{[{}]}}\\\\\n",t.depth*2,pos,t.formula.latex(),t.comment).into_bytes();
                file.write(&line)?;
            } else {
                let line = format!("&\\hspace{{{}em}}{})\\hspace{{1em}}{}\\\\\n",t.depth*2,pos,t.formula.latex()).into_bytes();
                file.write(&line)?;
            }

            prev_depth = t.depth;
        }

        file.write(b"\\end{flalign*}\n")?;
        file.write(b"\\end{document}")?;
        Ok(())
    }

    /// Create an annotated LaTeX file the given file name that displays the Deduction.
    pub fn latex_file_annotated(&self, filename: &str) -> Result<(), Error> {
        let filename = format!("{}.tex",filename);
        let mut file = File::create(filename)?;

        let section_title = format!("\\section*{{{}}}\n",self.title);

        file.write(b"\\documentclass[fleqn,11pt]{article}\n")?;
        file.write(b"\\usepackage{amsmath}\n")?;
        file.write(b"\\allowdisplaybreaks\n")?;
        file.write(b"\\begin{document}\n")?;
        file.write(&section_title.into_bytes())?;
        file.write(b"\\begin{flalign*}\n")?;

        let mut prev_depth = 0;
        for (pos,t) in self.theorems.iter().enumerate() {

            if t.depth > prev_depth {
                let line = format!("&{}\\text{{begin supposition}}&\\\\\n","   ".repeat(prev_depth)).into_bytes();
                file.write(&line)?;
            } else if t.depth < prev_depth {
                let line = format!("&{}\\text{{end supposition}}&\\\\\n","   ".repeat(t.depth)).into_bytes();
                file.write(&line)?;
            }

            let line = format!("&\\hspace{{{}em}}{})\\hspace{{1em}}{}\\hspace{{2em}}\\textbf{{[{}]}}\\\\\n",t.depth*2,pos,t.formula.latex(),t.annotation).into_bytes();
            file.write(&line)?;

            prev_depth = t.depth;
        }

        file.write(b"\\end{flalign*}\n")?;
        file.write(b"\\end{document}")?;
        Ok(())
    }

    pub fn english(&self) {
        for t in self.theorems.iter() {
            println!("{}) {} [{}]",t.position,t.formula.english(),t.annotation);
        }
    }

    /// Convert the Deduction to a (very large) integer.
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


    
    // Logical methods
    /// Push any axiom of the Deduction system into the theorems.
    pub fn add_axiom(&mut self, premise: &Formula, comment: &str) -> Result<(),LogicError> {
        if self.axioms.contains(&premise) {
            self.push_new( premise.clone(), comment, "axiom".to_string(), 0 );
        } else {
            let msg = format!("Axiom Error: {} is not a known axiom", premise);
            return Err(LogicError::new(msg))
        }
        Ok(())
    }

    /// Push a new theorem which replaces var with the provided Term in theorem n.
    pub fn specification<T: Term>(&mut self, n: usize, var: &Variable, replacement: &T, comment: &str) -> Result<(),LogicError> {
        let t = specification(self.get_theorem(n)?, &var, replacement);
        let r = format!("specification of {} to {} in theorem {}",var,replacement.get_string(),n);
        self.push_new( t?, comment, r, 1 );
        Ok(())
    }

    /// Push a new theorem that adds universal quantification of var in theorem n.
    pub fn generalization(&mut self, n: usize, var: &Variable, comment: &str) -> Result<(),LogicError> {
        if self.depth != 0 {
            let f = get_free_vars(&self.get_theorem(self.scope_cur)?.to_string());
            if f.contains(&var.to_string()) {
                let msg = format!("Generalization Error: the variable {} is free in the supposition {}",var,self.get_theorem(self.scope_cur)?);
                return Err(LogicError::new(msg))
            }
        }
        let t = generalization(self.get_theorem(n)?, &var);
        let r = format!("generalization of {} in theorem {}",var,n);
        self.push_new( t?, comment, r, 2 );
        Ok(())
    }

    /// Push a new theorem that adds existence quantification of var in theorem n.
    pub fn existence<T: Term>(&mut self, n: usize, term: &T, var: &Variable, comment: &str) -> Result<(),LogicError> {
        let t = existence(&self.theorems[n].formula.clone(), term, &var);
        let r = format!("existence of {} in theorem {}",var,n);
        self.push_new( t?, comment, r, 3 );
        Ok(())
    }

    /// Push a new theorem that applies the successor to each side of a theorem n.
    pub fn successor(&mut self, n: usize, comment: &str) -> Result<(),LogicError> {
        let t = successor(self.get_theorem(n)?);
        let r = format!("successor of theorem {}",n);
        self.push_new( t?, comment, r, 4 );
        Ok(())
    }

    /// Push a new theorem that strips the successor to each side of a theorem n.
    pub fn predecessor(&mut self, n: usize, comment: &str) -> Result<(),LogicError> {
        let t = predecessor(self.get_theorem(n)?);
        let r = format!("predecessor of theorem {}",n);
        self.push_new( t?, comment, r, 5 );
        Ok(())
    }

    /// Push a new theorem that takes theorem n and changes the negated existential quantifier at the given position to a universal quantifer followed by a negation.
    pub fn interchange_ea(&mut self, n: usize, v: &Variable, pos: usize, comment: &str) -> Result<(),LogicError> {
        let t = interchange_ea(self.get_theorem(n)?, v, pos);
        let r = format!("interchange ~E{}: for A{}:~ in theorem {}",v,v,n);
        self.push_new( t?, comment, r, 6 );
        Ok(())
    }

    /// Push a new theorem that takes theorem n and changes the universal quantifer followed by a negation at the given position with a negated existential quantifier.
    pub fn interchange_ae(&mut self, n: usize, v: &Variable, pos: usize, comment: &str) -> Result<(),LogicError> {
        let t = interchange_ae(self.get_theorem(n)?, v, pos);
        let r = format!("interchange A{}:~ for ~E{}: in theorem {}",v,v,n);
        self.push_new( t?, comment, r, 7 );
        Ok(())
    }

    /// Push a new theorem that flips the left and right sides of theorem n.
    pub fn symmetry(&mut self, n: usize, comment: &str) -> Result<(),LogicError> {
        let t = symmetry(self.get_theorem(n)?);
        let r = format!("symmetry of theorem {}",n);
        self.push_new( t?, comment, r, 8 );
        Ok(())
    }

    /// Push a new theorem that is an equality of the left term and right term of formula n1 and n2.
    pub fn transitivity(&mut self, n1: usize, n2: usize, comment: &str) -> Result<(),LogicError> {
        let t = transitivity(self.get_theorem(n1)?, self.get_theorem(n2)?);
        let r = format!("transitivity of theorem {} and theorem {}",n1,n2);
        self.push_new( t?, comment, r, 9 );
        Ok(())
    }
    
    /// Begin a supposition taking an arbitrary Formula as the premise.
    pub fn supposition(&mut self, premise: Formula, comment: &str) -> Result<(),LogicError> {
        // Push the current scope onto the stack and name the new scope after the index where it starts
        self.scope_stack.push(self.scope_cur);
        self.scope_cur = self.index;
        
        self.depth += 1;
        self.push_new( premise, comment, "supposition".to_string(), 10 );
        Ok(())
    }

    /// End a supposition and push a new theorem that the premise of the supposition implies the final theorem of the supposition.
    pub fn implication(&mut self, comment: &str) -> Result<(),LogicError> {

        // Create the formula and annotation
        let t = implies(self.get_theorem(self.scope_cur)?, self.get_last_theorem());
        let r = format!("implication of theorem {} and theorem {}",self.scope_cur,self.index);

        // Pop the top of the stack and make it the new scope
        self.scope_cur = self.scope_stack.pop().unwrap();
        self.depth -= 1;

        self.push_new( t, comment, r, 11 );
        Ok(())
    }

    /// Push a new theorem that is induction on given variable with base and general being the position of theorems that state the base case and general case.
    pub fn induction(&mut self, var: &Variable, base: usize, general: usize, comment: &str) -> Result<(),LogicError> {
        let t = induction(var,self.get_theorem(base)?,self.get_theorem(general)?);
        let r = format!("induction of {} on theorems {} and {}",var,base,general);
        self.push_new( t?, comment, r, 12 );
        Ok(())
    }
}