use crate::types::{Term,Formula,Atom,Variable,Termlike,Wellformed};
use crate::ops_construction::*;
use crate::string_manip::{replace_var_in_string,split_eq};

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




// TODO: test pathalogical inputs
#[test]
fn test_specification() {
    let a = Variable::new("a");
    let one = Term::new("S0");
    let formula1 = Formula::new("∀a:a=a");
    let formula2 = Formula::new("∃a':∀a:<a=a∧a'=a'>");
    assert_eq!(specification(&formula1,&a,&one).s,"S0=S0");
    assert_eq!(specification(&formula2,&a,&one).s,"∃a':<S0=S0∧a'=a'>");
}

#[test]
fn test_symmetry() {
    let atom1 = Atom::new("a=b");
    let atom2 = Atom::new("b=S(a+S0)");
    assert_eq!(symmetry(&atom1).s,"b=a");
    assert_eq!(symmetry(&atom2).s,"S(a+S0)=b");
}

#[test]
fn test_transitivity() {
    let atom1 = Atom::new("a=b");
    let atom2 = Atom::new("b=S(a+S0)");
    assert_eq!(transitivity(&atom1,&atom2).s,"a=S(a+S0)");
}

#[test]
fn test_predecessor() {
    let atom = Atom::new("Sm''=SSu");
    assert_eq!(predecessor(&atom).s,"m''=Su");
}

#[test]
fn test_successor() {
    let atom = Atom::new("Sm''=SSu");
    assert_eq!(successor(&atom).s,"SSm''=SSSu");
}


#[test]
fn test_interchange_ea() {
    let formula1 = Formula::new("∀a:~∃u':(a+u')=Sa");
    let formula2 = Formula::new("<∀a:~∃u':(a+u')=Sa∧~∃u':u'=SS0");
    let variable = Variable::new("u'");
    assert_eq!(interchange_ea(&formula1,&variable,0).s,"∀a:∀u':~(a+u')=Sa");
    assert_eq!(interchange_ea(&formula2,&variable,1).s,"<∀a:~∃u':(a+u')=Sa∧∀u':~u'=SS0");
}

#[test]
fn test_interchange_ae() {
    let formula1 = Formula::new("∀a:∀u':~(a+u')=Sa");
    let formula2 = Formula::new("<∀a:~∃u':(a+u')=Sa∧∀u':~u'=SS0");
    let variable = Variable::new("u'");
    assert_eq!(interchange_ae(&formula1,&variable,0).s,"∀a:~∃u':(a+u')=Sa");
    assert_eq!(interchange_ae(&formula2,&variable,0).s,"<∀a:~∃u':(a+u')=Sa∧~∃u':u'=SS0");
}

#[test]
fn test_induction() {
    let theorem = Formula::new("v=v");
    let v = Variable::new("v");
    let base = Formula::new("0=0");
    let gen = Formula::new("Sv=Sv");
    assert_eq!(induction(&theorem,&v,&base,&gen).s,"∀v:v=v");
}
