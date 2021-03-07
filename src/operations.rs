use crate::types::{Term,Formula,Variable,Termlike};
use crate::string_manip::replace_var_in_string;


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