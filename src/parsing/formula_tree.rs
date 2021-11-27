use std::fmt;


#[derive(Clone,Debug,PartialEq, Eq)]
pub struct Variable {
    name: String
}

impl Variable {
    pub fn replace_var(replacement: Term) -> Term {
        replacement
    }
}



#[derive(Clone,Debug)]
pub enum Term {
    Zero,
    Variable(Variable),
    Add(Box<Term>,Box<Term>),
    Mul(Box<Term>,Box<Term>),
    Successor(Box<Term>),
}

impl Term {
    pub fn replace_var(&self, v: &Variable, replacement: &Term) -> Term {
        match self {
            Term::Zero => Term::Zero,

            Term::Variable(var) => {
                if var == v {
                    replacement.clone()
                } else {
                    Term::Variable(var.clone())
                }
            },

            Term::Add(t1, t2) => Term::Add(Box::new(t1.replace_var(v,replacement)),
                                                                Box::new(t2.replace_var(v,replacement))
                                                            ),

            Term::Mul(t1, t2) => Term::Add(Box::new(t1.replace_var(v,replacement)),
                                                                Box::new(t2.replace_var(v,replacement))
                                                            ),

            Term::Successor(t) => Term::Successor(Box::new(t.replace_var(v, replacement)))

        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Term::Zero => write!(f, "0"),
            Term::Variable(v) => write!(f, "{}", v.name),
            Term::Add(t1, t2) => write!(f, "{}+{}", t1, t2),
            Term::Mul(t1, t2) =>  write!(f, "{}*{}", t1, t2),
            Term::Successor(t) => write!(f, "S{}", t),
        }
    }
}



#[derive(Clone,Debug)]
pub enum Formula {
    Equality(Term,Term), // the only Simple formula
    And(Box<Formula>,Box<Formula>),
    Or(Box<Formula>,Box<Formula>),
    Implies(Box<Formula>,Box<Formula>),
    Exists(Variable,Box<Formula>),
    ForAll(Variable,Box<Formula>),
    Negation(Box<Formula>)
}

impl Formula {
    /// An &str is automatically converted to the correct variant, this requires potentially slow parsing of the &str
/*     pub fn new_formula(&str) -> Formula {

    } */

    /// Fast creation of various formulae
    pub fn equality(t1: Term, t2: Term) -> Formula {
        return Formula::Equality(t1,t2)
    }

    pub fn and(t1: Formula, t2: Formula) -> Formula {
        return Formula::And(Box::new(t1),Box::new(t2))
    }

    pub fn or(t1: Formula, t2: Formula) -> Formula {
        return Formula::Or(Box::new(t1),Box::new(t2))
    }

    pub fn implies(t1: Formula, t2: Formula) -> Formula {
        return Formula::Implies(Box::new(t1),Box::new(t2))
    }

    pub fn exists(v: Variable, f: Formula) -> Formula {
        return Formula::Exists(v,Box::new(f))
    }

    pub fn forall(v: Variable, f: Formula) -> Formula {
        return Formula::ForAll(v,Box::new(f))
    }

    pub fn negation(f: Formula) -> Formula {
        return Formula::Negation(Box::new(f))
    }



    pub fn replace_var(&self, v: &Variable, replacement: &Term) -> Formula {
        match self {

            Formula::Equality(t1, t2) => Formula::Equality(
                t1.replace_var(v,replacement),
                t2.replace_var(v,replacement)
            ),

            Formula::And(f1, f2) => Formula::And(
                Box::new(f1.replace_var(v,replacement)),
                Box::new(f2.replace_var(v,replacement))
            ),

            Formula::Or(f1, f2) => Formula::Or(
                Box::new(f1.replace_var(v,replacement)),
                Box::new(f2.replace_var(v,replacement))
            ),

            Formula::Implies(f1, f2) => Formula::Implies(
                Box::new(f1.replace_var(v,replacement)),
                Box::new(f2.replace_var(v,replacement))
            ),

            Formula::Exists(v, f) => Formula::Exists(
                v.clone(),
                Box::new(f.replace_var(v,replacement))
            ),

            Formula::ForAll(v, f) => Formula::ForAll(
                v.clone(),
                Box::new(f.replace_var(v,replacement))
            ),

            Formula::Negation(f) => Formula::Negation(
                Box::new(f.replace_var(v,replacement))
            ),
            
        }
    }

    // Translation of the Formula to a different representation
/*     

    pub fn english(&self) -> String {
        to_english(self.to_string())
    }

    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    pub fn arithmetize(&self) -> BigUint {
        arithmetize(self.to_string())
    } */

/*
    pub fn dearithmetize(number: &BigUint) -> Formula {
        Formula::new(&dearithmetize(number))
    }

    /// Return the Formula converted into its canonical austere form
    pub fn austere(&self) -> Formula {
        Formula::new(&to_austere(self.to_string()))
    }

    /// Replace every instance of a Variable in the Formula with some Term


    /// Eliminate universal quantification of a Variable in the Formula then replace every instance with some Term
    pub fn specify_var<T: Term>(&self, v: &Variable, replacement: &T) -> Formula {
        let mut st = self.to_string().replace(&format!("A{}:",v),"");
        st = replace_all_re(&st, &v.re, &replacement.get_string()[..]);
        Formula::new(&st )
    }

    /// Does the Formula contain the Variable in question?
    pub fn contains_var(&self, v: &Variable) -> bool {
        v.re.find(&self.to_string()).unwrap().is_some()
    }

    /// Does the Formula contain the Variable in a quantification?
    pub fn contains_var_bound(&self, v: &Variable) -> bool {
        v.req.find(&self.to_string()).unwrap().is_some()
    } */

}

/* impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Formula::Equality(t1, t2) => write!(f, "{}={}", t1,t2),
            Formula::Complex(form) => write!(f, "{}", form),
        }
    }
} */
/* 
/// Two formulas are considered equal if their austere versions are identical
impl PartialEq for Formula {
    fn eq(&self, other: &Self) -> bool {
        self.austere().to_string() == other.austere().to_string()
    }
} */




/* #[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_replace_var() {
        let a = &Variable::new("a");
        let b = &Variable::new("b");
        let f1 = Formula::new("Aa:Ea':a=Sa'");
        assert_eq!( f1.replace_var(a,b).to_string(), "Ab:Ea':b=Sa'".to_string() )
    }

    #[test]
    fn test_specify_var() {
        let a = &Variable::new("a");
        let b = &Variable::new("b");
        let f1 = Formula::new("Aa:Ea':a=Sa'");
        assert_eq!( f1.specify_var(a,b).to_string(), "Ea':b=Sa'".to_string() )
    }

} */