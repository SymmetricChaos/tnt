use crate::types::{Term,Formula,Atom,Variable,Termlike,Wellformed};
use crate::string_manip::{replace_var_in_string,split_eq};

// Rules of construction. 
// These rules do not check any internal constraits.

// Arithmetic
pub fn succ<T: Termlike>(x: &T) -> Term {
    let new_s = format!("S{}",x.get_string());
    Term::new(&new_s)
}

pub fn add<A: Termlike, B: Termlike>(x: &A, y: &B) -> Term {
    let new_s = format!("({}+{})",x.get_string(),y.get_string());
    Term::new(&new_s)
}

pub fn mul<A: Termlike, B: Termlike>(x: &A, y: &B) -> Term {
    let new_s = format!("({}·{})",x.get_string(),y.get_string());
    Term::new(&new_s)
}


// Logical
pub fn not<T: Wellformed>(x: &T) -> Formula {
    let new_s = format!("~{}",x.get_string());
    Formula::new(&new_s)
}
pub fn eq<A: Termlike, B: Termlike>(x: &A, y: &B) -> Atom {
    let new_s = format!("{}={}",x.get_string(),y.get_string());
    Atom::new(&new_s)
}

pub fn or<A: Wellformed, B: Wellformed>(x: &A, y: &B) -> Formula {
    let new_s = format!("<{}∨{}>",x.get_string(),y.get_string());
    Formula::new(&new_s)
}

pub fn and<A: Wellformed, B: Wellformed>(x: &A, y: &B) -> Formula {
    let new_s = format!("<{}∧{}>",x.get_string(),y.get_string());
    Formula::new(&new_s)
}

pub fn implies<A: Wellformed, B: Wellformed>(x: &A, y: &B) -> Formula {
    let new_s = format!("<{}⊃{}>",x.get_string(),y.get_string());
    Formula::new(&new_s)
}


// Quantification
pub fn exists<F: Wellformed>(v: &Variable, x: &F) -> Formula {
    let new_s = format!("∃{}:{}",v.get_string(),x.get_string());
    Formula::new(&new_s)
}

pub fn forall<F: Wellformed>(v: &Variable, x: &F) -> Formula {
    let new_s = format!("∀{}:{}",v.get_string(),x.get_string());
    Formula::new(&new_s)
}


// Rules of production
// These may check for additional internal contraints and will panic on failure

pub fn specify(x: &Formula, v: &Variable, t: &Term) -> Formula {
    if x.s.contains(&format!("∀{}:",v)) {
        let mut new_s = x.s.clone().replace(&format!("∀{}:",v.s),"");
        if !x.bound_vars.contains(&v.s) {
            panic!("{} is bound in {}",v,x)
        }
        new_s = replace_var_in_string(&new_s,&v.s,&t.s);
        return Formula::new(&new_s)
    } else {
        panic!("{} is not univerally quantified in {}",v,x)
    }
}


pub fn generalize(x: &Formula, v: &Variable) -> Formula {
    if x.s.contains(&format!("∀{}:",v)) {
        return forall(v,x)
    } else {
        panic!("{} is bound in {}",v,x)
    }
}

/*
pub fn interchange_EA(x: &Formula, v: &Variable) -> Formula {

}

pub fn interchange_AE(x: &Formula, v: &Variable) -> Formula {

}
*/


pub fn successor(a: &Atom) -> Atom {
    if let Some((l,r)) = split_eq(&a.s) {
        let lt = Term::new(&format!("S{}",l));
        let rt = Term::new(&format!("S{}",r));
        return eq(&lt,&rt)
    } else {
        panic!("unable to split {} to apply the successor rule",a)
    }
}

pub fn predecessor(a: &Atom) -> Atom {
    if let Some((l,r)) = split_eq(&a.s) {
        if l.starts_with("S") && r.starts_with("S") {
            let lt = Term::new(&l.strip_prefix("S").unwrap());
            let rt = Term::new(&r.strip_prefix("S").unwrap());
            return eq(&lt,&rt)
        };
    }
    panic!("unable to split {} to apply the predecessor rule",a)

}

/*
pub fn existence(x: &Formula, v: &Variable, t: &Term) {

}

pub fn symmetry(a: &Atom) {

}

pub fn transitivity(a1: &Atom, a2: &Atom) {

}

pub fn induction() {

}
*/


#[test]
fn test_specify() {
    let a = Variable::new("a");
    let one = Term::new("S0");
    let formula1 = Formula::new("∀a:a=a");
    let formula2 = Formula::new("∃a':∀a:<a=a∧a'=a'>");
    assert_eq!(specify(&formula2,&a,&one).s,"∃a':<S0=S0∧a'=a'>");
}

#[test]
fn test_add() {
    let a = Term::new("a");
    let b = Term::new("b");
    assert_eq!(add(&a,&b).s,"(a+b)");
    assert_eq!(add(&a,&add(&b,&b)).s,"(a+(b+b))");
}