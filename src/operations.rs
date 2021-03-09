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
pub fn specification(x: &Formula, v: &Variable, t: &Term) -> Formula {
    if x.s.contains(&format!("∀{}:",v)) {
        let mut new_s = x.s.clone().replace(&format!("∀{}:",v.s),"");
        if !x.bound_vars.contains(&v.s) {
            panic!("Specification Error: {} is bound in {}",v,x)
        }
        new_s = replace_var_in_string(&new_s,&v.s,&t.s);
        return Formula::new(&new_s)
    } else {
        panic!("Specification Error: {} is not univerally quantified in {}",v,x)
    }
}

pub fn generalization(x: &Formula, v: &Variable) -> Formula {
    if x.bound_vars.contains(&v.s) {
        panic!("Generalization Error: {} is bound in {}",v,x)
    } else {
        return forall(v,x)
    }
}

pub fn existence<T: Termlike>(x: &Formula, t: &T, v: &Variable) -> Formula {
    if x.bound_vars.contains(&v.s) {
        panic!("Existence Error: {} is bound in {}",v,x)
    } else {
        Formula::new(&replace_var_in_string(&x.s, &t.get_string(), &v.s))
    }
}

pub fn interchange_ea(x: &Formula, v: &Variable, n: usize) -> Formula {
    let e = format!("~∃{}:",v);
    let a = format!("∀{}:~",v);
    let mut new_s = x.s.clone();
    let qs = x.s.match_indices(&e);
    for (pos,q) in qs.enumerate() {
        if pos == n {
            new_s.replace_range(q.0..q.0+q.1.len(), &a);
            break
        }
    }
    Formula::new(&new_s)
}

pub fn interchange_ae(x: &Formula, v: &Variable, n: usize) -> Formula {
    let e = format!("~∃{}:",v);
    let a = format!("∀{}:~",v);
    let mut new_s = x.s.clone();
    let qs = x.s.match_indices(&a);
    for (pos,q) in qs.enumerate() {
        if pos == n {
            new_s.replace_range(q.0..q.0+q.1.len(), &e);
            break
        }
    }
    Formula::new(&new_s)
}


pub fn induction<A: Wellformed, B: Wellformed, C: Wellformed>(theorem: &A, var: &Variable, base: &B, general: &C) -> Formula {
    if theorem.bound_vars().contains(&var.s) {
        panic!("Induction Error: {} is already bound in {}",var,theorem.get_string())
    } else {
        let xs = replace_var_in_string(&theorem.get_string(), &var.s, &format!("S{}",var));
        let x0 = replace_var_in_string(&theorem.get_string(), &var.s, "0");
        if x0 != base.get_string() {
            panic!("Induction Error: base case must be {}",x0)
        }
        if xs != general.get_string() {
            panic!("Induction Error: general case must be {}",xs)
        }
        forall(var,theorem)
    }
}

pub fn successor(a: &Atom) -> Atom {
    if let Some((l,r)) = split_eq(&a.s) {
        let lt = Term::new(&format!("S{}",l));
        let rt = Term::new(&format!("S{}",r));
        return eq(&lt,&rt)
    } else {
        unreachable!("Successor Error: unable to split {}",a)
    }
}

pub fn predecessor(a: &Atom) -> Atom {
    if let Some((l,r)) = split_eq(&a.s) {
        if l.starts_with("S") && r.starts_with("S") {
            let lt = Term::new(&l.strip_prefix("S").unwrap());
            let rt = Term::new(&r.strip_prefix("S").unwrap());
            return eq(&lt,&rt)
        } else {
            panic!("Predecessor Error: both terms of {} must begin with S",a)
        }
    }
    unreachable!("Predecessor Error: unable to split {}",a)

}


pub fn symmetry(a: &Atom) -> Atom {
    if let Some((l,r)) = split_eq(&a.s) {
        let lt = Term::new(&l);
        let rt = Term::new(&r);
        return eq(&rt,&lt)
    } else {
        unreachable!("Symmetry Error: unable to split {}",a)
    }
}


pub fn transitivity(a1: &Atom, a2: &Atom) -> Atom {
    if let Some((l,_)) = split_eq(&a1.s) {
        if let Some((_,r)) = split_eq(&a2.s) {
            let lt = Term::new(&l);
            let rt = Term::new(&r);
            return eq(&lt,&rt)
        } else {
            unreachable!("Symmetry Error: unable to split {}",a2)
        }
    } else {
        unreachable!("Symmetry Error: unable to split {}",a1)
    }
}




