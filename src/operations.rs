use crate::types::{Term,Formula,Atom,Variable,Termlike,Wellformed};
use crate::string_manip::replace_var_in_string;

// Arithmetic operations
pub fn succ(x: &Term) -> Term {
    let new_s = format!("S{}",x);
    Term::new(&new_s)
}

pub fn add(x: &Term, y: &Term) -> Term {
    let new_s = format!("({}+{})",x,y);
    Term::new(&new_s)
}

pub fn mul(x: &Term, y: &Term) -> Term {
    let new_s = format!("({}·{})",x,y);
    Term::new(&new_s)
}


// Logical operations
pub fn not(x: &Formula) -> Formula {
    let new_s = format!("~{}",x);
    Formula::new(&new_s)
}
pub fn eq<A: Termlike, B: Termlike>(x: &A, y: &B) -> Atom {
    let new_s = format!("{}={}",x.get_string(),y.get_string());
    Atom::new(&new_s)
}

pub fn or<T: Wellformed>(x: &T, y: &T) -> Formula {
    let new_s = format!("<{}∨{}>",x.get_string(),y.get_string());
    Formula::new(&new_s)
}

pub fn and<T: Wellformed>(x: &T, y: &T) -> Formula {
    let new_s = format!("<{}∧{}>",x.get_string(),y.get_string());
    Formula::new(&new_s)
}

pub fn implies<T: Wellformed>(x: &T, y: &T) -> Formula {
    let new_s = format!("<{}⊃{}>",x.get_string(),y.get_string());
    Formula::new(&new_s)
}


pub fn exists(v: &Variable, x: &Formula) -> Formula {
    if x.bound_vars.contains(&v.s) {
        panic!("{} is bound in {}",v,x)
    }
    let new_s = format!("∃{}:{}",v,x);
    Formula::new(&new_s)
}

pub fn forall(v: &Variable, x: &Formula) -> Formula {
    if x.bound_vars.contains(&v.s) {
        panic!("{} is bound in {}",v,x)
    }
    let new_s = format!("∀{}:{}",v,x);
    Formula::new(&new_s)
}


// Rules of production

pub fn specify(x: &Formula, v: &Variable, t: &Term) -> Formula {
    if x.s.contains(&format!("∀{}:",v)) {
        let mut new_s = x.s.clone().replace(&format!("∀{}:",v.s),"");
        if !x.bound_vars.contains(&v.s) {
            panic!("{} bound in {}",v,x)
        }
        new_s = replace_var_in_string(&new_s,&v.s,&t.s);
        return Formula::new(&new_s)
    } else {
        panic!("{} is not univerally quantified in {}",v,x)
    }
}

/*
pub fn generalize(x: &Formula, v: &Variable) -> Formula {

}

pub fn interchange_EA(x: &Formula, v: &Variable) -> Formula {

}

pub fn interchange_AE(x: &Formula, v: &Variable) -> Formula {

}

pub fn successor(a: &atom) -> Atom {

}

pub fn predecessor(a: &atom) -> Atom {

}

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