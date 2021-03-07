use crate::Types::{Term,Formula,Variable};
use crate::StringManipulation::{get_bound_vars};

pub fn add(x: &Term, y: &Term) -> Term {
    let new_s = format!("({}+{})",x,y);
    Term::new(&new_s)
}

pub fn mul(x: &Term, y: &Term) -> Term {
    let new_s = format!("({}·{})",x,y);
    Term::new(&new_s)
}

pub fn succ(x: &Term) -> Term {
    let new_s = format!("S{}",x);
    Term::new(&new_s)
}

pub fn eq(x: &Term, y: &Term) -> Formula {
    let new_s = format!("{}={}",x,y);
    Formula::new(&new_s)
}

pub fn or(x: &Formula, y: &Formula) -> Formula {
    let new_s = format!("<{}∨{}>",x,y);
    Formula::new(&new_s)
}

pub fn and(x: &Formula, y: &Formula) -> Formula {
    let new_s = format!("<{}∧{}>",x,y);
    Formula::new(&new_s)
}

pub fn implies(x: &Formula, y: &Formula) -> Formula {
    let new_s = format!("<{}⊃{}>",x,y);
    Formula::new(&new_s)
}

pub fn not(x: &Formula) -> Formula {
    let new_s = format!("~{}",x);
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