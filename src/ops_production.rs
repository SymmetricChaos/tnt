use crate::types::{Formula,Variable,Term,Number,Equation};
use crate::ops_construction::*;
use crate::string_manip::{replace_var_in_string, split_eq, get_bound_vars, left_implies};

// Rules of production
// These may check for additional internal contraints and will panic on failure
pub fn specification<T: Term>(x: &Formula, v: &Variable, t: &T) -> Formula {
    if x.to_string().contains(&format!("A{}:",v)) {
        let mut new_s = x.to_string().clone().replace(&format!("A{}:",v.to_string()),"");
        if get_bound_vars(&x.to_string()).contains(&v.to_string()) {
            new_s = replace_var_in_string(&new_s,&v.to_string(),&t.get_string());
            return Formula::new(&new_s)
        } else {
            panic!("Specification Error: {} is not bound in {}",v,x)
        }
    } else {
        panic!("Specification Error: {} is not univerally quantified in {}",v,x)
    }
}

pub fn generalization(x: &Formula, v: &Variable) -> Formula {
    if !get_bound_vars(&x.to_string()).contains(&v.to_string()) {
        return forall(v,x)
    } else {
        panic!("Generalization Error: {} is bound in {}",v,x)
    }
}

pub fn existence<T: Term>(x: &Formula, t: &T, v: &Variable) -> Formula {
    if !get_bound_vars(&x.to_string()).contains(&v.to_string()) {
        let out = exists(v,x);
        return Formula::new(&out.to_string().replace(&t.get_string(), &v.to_string()))
        
    } else {
        panic!("Existence Error: {} is bound in {}",v,x)
    }
}

// Should panic or warn if quantification not present
pub fn interchange_ea(x: &Formula, v: &Variable, nth: usize) -> Formula {
    let e = format!("~E{}:",v);
    let a = format!("A{}:~",v);
    let mut new_s = x.to_string().clone();
    let xs = x.to_string();
    let qs = xs.match_indices(&e);
    for (n,q) in qs.enumerate() {
        println!("{:?} {:?}",n,q);
        if n == nth {
            new_s.replace_range(q.0..q.0+q.1.len(), &a);
            break
        }
    }
    Formula::new_complex(&new_s)
}

// Should panic or warn if quantification not present
pub fn interchange_ae(x: &Formula, v: &Variable, n: usize) -> Formula {
    let e = format!("~E{}:",v);
    let a = format!("A{}:~",v);
    let mut new_s = x.to_string().clone();
    let xs = &x.to_string();
    let qs = xs.match_indices(&a);
    for (pos,q) in qs.enumerate() {
        if pos == n {
            new_s.replace_range(q.0..q.0+q.1.len(), &e);
            break
        }
    }
    Formula::new_complex(&new_s)
}

pub fn induction(v: &Variable, base: &Formula, general: &Formula) -> Formula {
    // The theorem we need to generalize is the outermost, leftmost implication of the general case
    let theorem = Formula::new(left_implies(&general.to_string()).unwrap());

    if get_bound_vars(&theorem.to_string()).contains(&v.to_string()) {
        panic!("Induction Error: {} is already bound in {}",v,theorem.to_string())
    } else {
        let xs = replace_var_in_string(&theorem.to_string(), &v.to_string(), &format!("S{}",v));
        let x0 = replace_var_in_string(&theorem.to_string(), &v.to_string(), "0");
        if x0 != base.to_string() {
            panic!("Induction Error: base case must be {}",x0)
        }
        if general.to_string() != format!("A{}:[{}>{}]",v,theorem,xs) {
            panic!("Induction Error: general case must be A{}:[{}>{}]",v,theorem,xs)
        }
        forall(v,&theorem)
    }
}

pub fn successor(a: &Formula) -> Formula {
    if let Formula::Simple(_) = a {
        if let Some((l,r)) = split_eq(&a.to_string()) {
            let lt = Equation::new(&format!("S{}",l));
            let rt = Equation::new(&format!("S{}",r));
            return eq(&lt,&rt)
        } else {
            unreachable!("Successor Error: unable to split {}",a)
        }
    } else {
        panic!("Successor Error: {} is not a Formula::Simple which is required in order to split it",a)
    }
}

pub fn predecessor(a: &Formula) -> Formula {
    if let Formula::Simple(_) = a {
        if let Some((l,r)) = split_eq(&a.to_string()) {
            if l.starts_with("S") && r.starts_with("S") {
                let lt = Equation::new(&l.strip_prefix("S").unwrap());
                let rt = Equation::new(&r.strip_prefix("S").unwrap());
                return eq(&lt,&rt)
            } else {
                panic!("Predecessor Error: both terms of {} must begin with S",a)
            }
        }
        unreachable!("Predecessor Error: unable to split {}",a)
    } else {
        panic!("Successor Error: {} is not a Formula::Simple which is required in order to split it",a)
    }
}

// Can be simplified
pub fn symmetry(a: &Formula) -> Formula {
    if let Formula::Simple(_) = a {
        if let Some((l,r)) = split_eq(&a.to_string()) {
            let lt = Equation::new(&l);
            let rt = Equation::new(&r);
            return eq(&rt,&lt)
        } else {
            unreachable!("Symmetry Error: unable to split {}",a)
        }
    } else {
        panic!("Successor Error: {} is not a Formula::Simple which is required in order to split it",a)
    }
}


pub fn transitivity(a1: &Formula, a2: &Formula) -> Formula {
    if let Formula::Simple(_) = a1 {
        if let Formula::Simple(_) = a2 {
            if let Some((l1,r1)) = split_eq(&a1.to_string()) {
                if let Some((l2,r2)) = split_eq(&a2.to_string()) {
                    if r1 != l2 {
                        panic!("Transitivity Error: The right term of {} does not match the left term of {}",a1,a2)
                    }
                    let lt = Equation::new(&l1);
                    let rt = Equation::new(&r2);
                    return eq(&lt,&rt)
                } else {
                    unreachable!("Transitivity Error: unable to split {}",a2)
                }
            } else {
                unreachable!("Transitivity Error: unable to split {}",a1)
            }
        } else {
            panic!("Transitivity Error: {} is not a Formula::Simple which is required in order to split it",a2)
        }
    } else {
        panic!("Transitivity Error: {} is not a Formula::Simple which is required in order to split it",a1)
    }
}




// TODO: test pathalogical inputs for all of these
// TODO: test panic modes for all of these

#[test]
fn test_specification() {
    let a = Variable::new("a");
    let one = Number::new("S0");
    let formula1 = Formula::new("Aa:a=a");
    let formula2 = Formula::new("Ea':Aa:[a=a&a'=a']");
    assert_eq!(specification(&formula1,&a,&one).to_string(),"S0=S0");
    assert_eq!(specification(&formula2,&a,&one).to_string(),"Ea':[S0=S0&a'=a']");
}

#[test]
fn test_symmetry() {
    let atom1 = Formula::new("a=b");
    let atom2 = Formula::new("b=S(a+S0)");
    assert_eq!(symmetry(&atom1).to_string(),"b=a");
    assert_eq!(symmetry(&atom2).to_string(),"S(a+S0)=b");
}

#[test]
fn test_transitivity() {
    let atom1 = Formula::new("a=b");
    let atom2 = Formula::new("b=S(a+S0)");
    assert_eq!(transitivity(&atom1,&atom2).to_string(),"a=S(a+S0)");
}

#[test]
fn test_predecessor() {
    let atom = Formula::new("Sm''=SSu");
    assert_eq!(predecessor(&atom).to_string(),"m''=Su");
}

#[test]
fn test_successor() {
    let atom = Formula::new("Sm''=SSu");
    assert_eq!(successor(&atom).to_string(),"SSm''=SSSu");
}

#[test]
fn test_interchange_ea() {
    let formula1 = Formula::new("Aa:~Eu':(a+u')=Sa");
    let formula2 = Formula::new("[Aa:~Eu':(a+u')=Sa&~Eu':u'=SS0]");
    let variable = Variable::new("u'");
    assert_eq!(interchange_ea(&formula1,&variable,0).to_string(),"Aa:Au':~(a+u')=Sa");
    assert_eq!(interchange_ea(&formula2,&variable,1).to_string(),"[Aa:~Eu':(a+u')=Sa&Au':~u'=SS0]");
}

#[test]
fn test_interchange_ae() {
    let formula1 = Formula::new("Aa:Au':~(a+u')=Sa");
    let formula2 = Formula::new("[Aa:~Eu':(a+u')=Sa&Au':~u'=SS0]");
    let variable = Variable::new("u'");
    assert_eq!(interchange_ae(&formula1,&variable,0).to_string(),"Aa:~Eu':(a+u')=Sa");
    assert_eq!(interchange_ae(&formula2,&variable,0).to_string(),"[Aa:~Eu':(a+u')=Sa&~Eu':u'=SS0]");
}

#[test]
fn test_induction() {
    let v = Term::new("v");
    let base = Formula::new("0=0");
    let gen = Formula::new("Av:[v=v>Sv=Sv]");
    assert_eq!(induction(&v,&base,&gen).to_string(),"Av:v=v");
}
