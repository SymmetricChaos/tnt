use indexmap::IndexSet;
use num::BigUint;
use std::{
    convert::TryFrom,
    fmt,
    fs::File,
    io::{Error, Write},
    slice::Iter,
};

use crate::{production::*, Formula, LogicError, Term};

/// All the rules of production.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Rule {
    Axiom,
    Specification,
    Generalization,
    Existence,
    Successor,
    Predecessor,
    InterchangeAE,
    InterchangeEA,
    Symmetry,
    Transitivity,
    Supposition,
    Implication,
    Induction,
}

/// Information tracked about each Formula
#[derive(Clone)]
pub struct TheoremFrame {
    pub formula: Formula,
    pub depth: usize,
    pub position: usize,
    pub annotation: String,
    pub rule: Rule,
    pub scope: usize,
}

impl TheoremFrame {
    pub fn new(
        formula: Formula,
        depth: usize,
        position: usize,
        annotation: String,
        rule: Rule,
        scope: usize,
    ) -> TheoremFrame {
        TheoremFrame {
            formula,
            depth,
            position,
            annotation,
            rule,
            scope,
        }
    }
}

/// Enforces valid use of deductive logic to produce proofs in Typographical Number Theory and outputs formatted results.
#[derive(Clone)]
pub struct Deduction {
    index: usize,
    scope_stack: Vec<usize>,
    scope_cur: usize,
    pub title: String,
    axioms: Vec<Formula>,
    theorems: Vec<TheoremFrame>,
}

// When 'true' forces the theorems to be printed every time they are added, helps with debugging
const NOISY: bool = false;

impl Deduction {
    pub fn custom(title: &str, axioms: Vec<Formula>) -> Deduction {
        Deduction {
            index: 0,
            scope_stack: vec![0],
            scope_cur: 0,
            title: title.to_string(),
            axioms,
            theorems: Vec::<TheoremFrame>::new(),
        }
    }

    /**
     * These are the axiomatic statements of the TNT formal system, they don't align strictly with the Peano Axioms but they define the same arithmetic properties for addition and multiplication. The axioms are as follows:
     *
     * Aa:~Sa=0                  for all a, it is false that (a + 1) is 0
     *
     * Aa:(a+0)=a                for all a, (a + 0) = a
     *
     * Aa:Ab:(a+Sb)=S(a+b)       for all a and b, (a + (b + 1)) = ((a + b) + 1)
     *
     * Aa:(a\*0)=0               for all a, (a × 0) = 0
     *
     * Aa:Ab:(a\*Sb)=((a\*b)+a)  for all a and b, (a × (b + 1)) = ((a × b) + a)
     */

    pub fn new(title: &str) -> Deduction {
        let peano_axioms = vec![
            Formula::try_from("Aa:~Sa=0").unwrap(),
            Formula::try_from("Aa:(a+0)=a").unwrap(),
            Formula::try_from("Aa:Ab:(a+Sb)=S(a+b)").unwrap(),
            Formula::try_from("Aa:(a*0)=0").unwrap(),
            Formula::try_from("Aa:Ab:(a*Sb)=((a*b)+a)").unwrap(),
        ];

        Deduction {
            index: 0,
            scope_stack: vec![0],
            scope_cur: 0,
            title: title.to_string(),
            axioms: peano_axioms,
            theorems: Vec::<TheoremFrame>::new(),
        }
    }

    // Internal methods
    // Get a theorem if it is in an accessible scope
    fn get_theorem(&self, n: usize) -> Result<&Formula, LogicError> {
        // Check the scope
        let tscope = self.theorems[n].scope;
        if tscope == self.scope_cur || self.scope_stack.contains(&tscope) {
            return Ok(&self.theorems[n].formula);
        }
        let msg = format!("Scope Error: position {n} is not in an accessible scope");
        Err(LogicError::new(msg))
    }

    // The last theorem on the list is always in an accessible scope.
    fn get_last_theorem(&self) -> &Formula {
        &self.theorems.last().unwrap().formula
    }

    // Pushes a new TheoremFrame and updates the index
    fn push_new(&mut self, theorem: Formula, annotation: String, rule: Rule) {
        if NOISY {
            if rule == Rule::Supposition {
                println!("{}begin supposition", "   ".repeat(self.depth() - 1))
            } else if rule == Rule::Implication {
                println!("{}end supposition", "   ".repeat(self.depth()))
            }
            println!(
                "{}{}) {} [{}]",
                "   ".repeat(self.depth()),
                self.index,
                theorem,
                annotation
            )
        }
        let depth = match rule {
            Rule::Supposition => self.depth() + 1,
            Rule::Implication => self.depth() - 1,
            _ => self.depth(),
        };
        let t = TheoremFrame {
            formula: theorem,
            depth: depth,
            position: self.index,
            annotation,
            rule,
            scope: self.scope_cur,
        };

        self.theorems.push(t);
        self.index += 1;
    }

    /// Current depth of the deduction.
    pub fn depth(&self) -> usize {
        if self.theorems.is_empty() {
            0
        } else {
            self.theorems.last().unwrap().depth
        }
    }

    /// Reference a TheoremFrame.
    pub fn theorem(&self, n: usize) -> &TheoremFrame {
        &self.theorems[n]
    }

    /// Return the last TheoremFrame.
    pub fn last_theorem(&self) -> &TheoremFrame {
        &self.theorems.last().unwrap()
    }

    /// Iterate over TheoremFrames of the Deduction.
    pub fn theorems(&self) -> Iter<TheoremFrame> {
        self.theorems.iter()
    }

    /// Write the Deduction.
    pub fn pretty_string(&self) -> String {
        let mut out = String::new();
        let mut prev_depth = 0;
        for (pos, t) in self.theorems.iter().enumerate() {
            if t.depth > prev_depth {
                let begin = format!("\n{}begin supposition", "   ".repeat(prev_depth));
                out.push_str(&begin);
            } else if t.depth < prev_depth {
                let end = format!("\n{}end supposition", "   ".repeat(t.depth));
                out.push_str(&end);
            } else {
            }
            let line = format!(
                "\n{}{}) {}",
                "   ".repeat(t.depth),
                pos,
                t.formula.pretty_string()
            );
            out.push_str(&line);
            prev_depth = t.depth;
        }
        out
    }

    /// Create an annotated LaTeX file the given file name that displays the Deduction.
    pub fn latex_file(&self, filename: &str) -> Result<(), Error> {
        let filename = format!("{}.tex", filename);
        let mut file = File::create(filename)?;

        let section_title = format!("\\section*{{{}}}\n", self.title);

        file.write(b"\\documentclass[fleqn,11pt]{article}\n")?;
        file.write(b"\\usepackage{amsmath}\n")?;
        file.write(b"\\allowdisplaybreaks\n")?;
        file.write(b"\\begin{document}\n")?;
        file.write(&section_title.into_bytes())?;
        file.write(b"\\begin{flalign*}\n")?;

        let mut prev_depth = 0;
        for (pos, t) in self.theorems.iter().enumerate() {
            if t.depth > prev_depth {
                let line = format!(
                    "&{}\\text{{begin supposition}}&\\\\\n",
                    "   ".repeat(prev_depth)
                )
                .into_bytes();
                file.write(&line)?;
            } else if t.depth < prev_depth {
                let line = format!("&{}\\text{{end supposition}}&\\\\\n", "   ".repeat(t.depth))
                    .into_bytes();
                file.write(&line)?;
            }

            let line = format!(
                "&\\hspace{{{}em}}{})\\hspace{{1em}}{}\\hspace{{2em}}\\textbf{{[{}]}}\\\\\n",
                t.depth * 2,
                pos,
                t.formula.to_latex(),
                t.annotation
            )
            .into_bytes();
            file.write(&line)?;

            prev_depth = t.depth;
        }

        file.write(b"\\end{flalign*}\n")?;
        file.write(b"\\end{document}")?;
        Ok(())
    }

    pub fn english(&self) -> String {
        let mut out = String::new();
        for t in self.theorems.iter() {
            out.push_str(&format!("\n{}) {}", t.position, t.formula.to_english()));
        }
        out
    }

    pub fn english_annotated(&self) -> String {
        let mut out = String::new();
        for t in self.theorems.iter() {
            out.push_str(&format!(
                "\n{}) {} [{}]",
                t.position,
                t.formula.to_english(),
                t.annotation
            ));
        }
        out
    }

    // Logical methods
    /// Push any axiom of the Deduction system into the theorems.
    pub fn add_axiom(&mut self, premise: usize) -> Result<(), LogicError> {
        if let Some(axiom) = self.axioms.get(premise) {
            self.push_new(axiom.clone(), "axiom".to_string(), Rule::Axiom);
            Ok(())
        } else {
            Err(LogicError(format!(
                "Axiom Error: there is no axiom #{premise}"
            )))
        }
    }

    /// Push a new theorem which replaces var with the provided Term in theorem n.
    pub fn specification(
        &mut self,
        n: usize,
        var_name: &'static str,
        term: &Term,
    ) -> Result<(), LogicError> {
        let t = specification(self.get_theorem(n)?, &var_name, term)?;
        let r = format!("specification of {var_name} to {term} in theorem {n}");
        self.push_new(t, r, Rule::Specification);
        Ok(())
    }

    /// Push a new theorem that adds universal quantification of var in theorem n.
    pub fn generalization(&mut self, n: usize, var_name: &'static str) -> Result<(), LogicError> {
        if self.depth() != 0 {
            let mut free_vars = IndexSet::<String>::new();
            self.get_theorem(self.scope_cur)?
                .get_vars_free(&mut free_vars);
            if free_vars.contains(&var_name.to_string()) {
                let msg = format!(
                    "Generalization Error: the variable {var_name} is free in the supposition {}",
                    self.get_theorem(self.scope_cur)?
                );
                return Err(LogicError::new(msg));
            }
        }
        let t = generalization(self.get_theorem(n)?, var_name);
        let r = format!("generalization of {var_name} in theorem {n}");
        self.push_new(t?, r, Rule::Generalization);
        Ok(())
    }

    /// Push a new theorem that adds existence quantification of var in theorem n.
    pub fn existence(&mut self, n: usize, var_name: &str) -> Result<(), LogicError> {
        let t = Formula::exists(var_name, &self.theorems[n].formula.clone());
        let r = format!("existence of {var_name} in theorem {n}");
        self.push_new(t, r, Rule::Existence);
        Ok(())
    }

    /// Push a new theorem that applies the successor to each side of a theorem n.
    pub fn successor(&mut self, n: usize) -> Result<(), LogicError> {
        let t = successor(self.get_theorem(n)?);
        let r = format!("successor of theorem {n}");
        self.push_new(t?, r, Rule::Successor);
        Ok(())
    }

    /// Push a new theorem that strips the successor to each side of a theorem n.
    pub fn predecessor(&mut self, n: usize) -> Result<(), LogicError> {
        let t = predecessor(self.get_theorem(n)?);
        let r = format!("predecessor of theorem {n}");
        self.push_new(t?, r, Rule::Predecessor);
        Ok(())
    }

    /// Push a new theorem that takes theorem n and changes the negated existential quantifier at the given position to a universal quantifer followed by a negation.
    pub fn interchange_ea(
        &mut self,
        n: usize,
        var_name: &str,
        pos: usize,
    ) -> Result<(), LogicError> {
        let t = interchange_ea(self.get_theorem(n)?, var_name, pos);
        let r = format!("interchange ~E{var_name}: for A{var_name}:~ in theorem {n}");
        self.push_new(t?, r, Rule::InterchangeEA);
        Ok(())
    }

    /// Push a new theorem that takes theorem n and changes the universal quantifer followed by a negation at the given position with a negated existential quantifier.
    pub fn interchange_ae(
        &mut self,
        n: usize,
        var_name: &str,
        pos: usize,
    ) -> Result<(), LogicError> {
        let t = interchange_ae(self.get_theorem(n)?, var_name, pos);
        let r = format!("interchange A{var_name}:~ for ~E{var_name}: in theorem {n}");
        self.push_new(t?, r, Rule::InterchangeAE);
        Ok(())
    }

    /// Push a new theorem that flips the left and right sides of theorem n.
    pub fn symmetry(&mut self, n: usize) -> Result<(), LogicError> {
        let t = symmetry(self.get_theorem(n)?);
        let r = format!("symmetry of theorem {n}");
        self.push_new(t?, r, Rule::Symmetry);
        Ok(())
    }

    /// Push a new theorem that is an equality of the left term and right term of formula n1 and n2.
    pub fn transitivity(&mut self, n1: usize, n2: usize) -> Result<(), LogicError> {
        let t = transitivity(self.get_theorem(n1)?, self.get_theorem(n2)?);
        let r = format!("transitivity of theorem {n1} and theorem {n2}");
        self.push_new(t?, r, Rule::Transitivity);
        Ok(())
    }

    /// Begin a supposition taking an arbitrary Formula as the premise.
    pub fn supposition(&mut self, premise: Formula) -> Result<(), LogicError> {
        // Push the current scope onto the stack and name the new scope after the index where it starts
        self.scope_stack.push(self.scope_cur);
        self.scope_cur = self.index;
        self.push_new(premise, "supposition".to_string(), Rule::Supposition);
        Ok(())
    }

    /// End a supposition and push a new theorem that the premise of the supposition implies the final theorem of the supposition.
    pub fn implication(&mut self) -> Result<(), LogicError> {
        // Create the formula and annotation
        let t = Formula::implies(self.get_theorem(self.scope_cur)?, self.get_last_theorem());
        let r = format!(
            "implication of theorem {} and theorem {}",
            self.scope_cur, self.index
        );

        // Pop the top of the stack and make it the new scope
        self.scope_cur = self.scope_stack.pop().unwrap();
        self.push_new(t, r, Rule::Implication);
        Ok(())
    }

    /// Push a new theorem that is induction on given variable with base and general being the position of theorems that state the base case and general case.
    pub fn induction(
        &mut self,
        var_name: &str,
        base: usize,
        general: usize,
    ) -> Result<(), LogicError> {
        let t = induction(
            var_name,
            self.get_theorem(base)?,
            self.get_theorem(general)?,
        );
        let r = format!("induction of {var_name} on theorems {base} and {general}");
        self.push_new(t?, r, Rule::Induction);
        Ok(())
    }

    fn vars_in_order(&self) -> IndexSet<String> {
        let mut vars = IndexSet::new();
        for theorem in self.theorems.iter() {
            theorem.formula.get_vars(&mut vars)
        }
        vars
    }

    /// Produce a Deduction of identical form with all Formulas in austere form.
    pub fn austere(&self) -> Deduction {
        let vars = self.vars_in_order();
        let mut out = self.clone();
        for theorem in out.theorems.iter_mut() {
            theorem.formula.to_austere_with(&vars);
        }
        out
    }

    /// Change all Formulas in the Deduction to their austere form.
    pub fn to_austere(&mut self) {
        let vars = self.vars_in_order();
        for theorem in self.theorems.iter_mut() {
            theorem.formula.to_austere_with(&vars);
        }
    }

    /// Convert the Deduction to a (very large) integer. Formulas are separated by the the ASCII null symbol.
    pub fn arithmetize(&self) -> BigUint {
        let austere = self.austere();
        let mut n: Vec<u8> = Vec::new();
        for t in austere.theorems().rev() {
            n.extend(t.formula.to_string().into_bytes().iter());
            n.push(0);
        }
        BigUint::from_bytes_be(&n)
    }
}

impl fmt::Display for Deduction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        let mut prev_depth = 0;
        for (pos, t) in self.theorems.iter().enumerate() {
            if t.depth > prev_depth {
                let begin = format!("\n{}begin supposition", "   ".repeat(prev_depth));
                out.push_str(&begin);
            } else if t.depth < prev_depth {
                let end = format!("\n{}end supposition", "   ".repeat(t.depth));
                out.push_str(&end);
            } else {
            }
            let line = format!(
                "\n{}{}) {}",
                "   ".repeat(t.depth),
                pos,
                t.formula.to_string()
            );
            out.push_str(&line);
            prev_depth = t.depth;
        }
        write!(f, "{}", out)
    }
}

// impl TryFrom<BigUint> for Deduction {
//     type Error = LogicError;

//     fn try_from(value: BigUint) -> Result<Self, Self::Error> {
//         match from_utf8(&value.to_bytes_be()) {
//             Ok(s) => Formula::try_from(s),
//             Err(e) => Err(LogicError(e.to_string())),
//         }
//     }
// }
