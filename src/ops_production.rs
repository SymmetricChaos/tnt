use crate::types::{Formula,Variable,Term,Expression,Number};
use crate::ops_construction::*;
use crate::string_manip::{split_eq, get_bound_vars, left_implies, get_vars};

// Rules of production
// These may check for additional internal contraints and will panic on failure
pub fn specification<T: Term>(x: &Formula, v: &Variable, t: &T) -> Formula {
    if x.to_string().contains(&format!("A{}:",v)) {
        let var_in_t = get_vars(&t.get_string());
        let bound_in_x = get_bound_vars(&x.to_string());
        for var in var_in_t {
            if bound_in_x.contains(&var) && t.get_string() != v.get_string() {
                panic!("Specification Error: The Term {} contains the variable {} which is already bound in {}",t.get_string(),var,x)
            }
        }
        return x.specify_var(v, t)
    } else {
        panic!("Specification Error: {} is not univerally quantified in {}",v,x)
    }
}

pub fn generalization(x: &Formula, v: &Variable) -> Formula {
    if !x.contains_var_bound(&v) {
        return forall(v,x)
    } else {
        panic!("Generalization Error: {} is already bound in {}",v,x)
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

pub fn interchange_ea(x: &Formula, v: &Variable, nth: usize) -> Formula {
    let e = format!("~E{}:",v);
    let a = format!("A{}:~",v);
    let mut new_s = x.to_string().clone();
    let xs = x.to_string();
    let qs = xs.match_indices(&e);
    if qs.clone().count() < nth {
        panic!("Interchange Error: There quantification {} does not exist in {} {} times",e,x,nth);
    }
    for (pos,q) in qs.enumerate() {
        if pos == nth {
            new_s.replace_range(q.0..q.0+q.1.len(), &a);
            break
        }
    }
    Formula::new_complex(&new_s)
}

pub fn interchange_ae(x: &Formula, v: &Variable, nth: usize) -> Formula {
    let e = format!("~E{}:",v);
    let a = format!("A{}:~",v);
    let mut new_s = x.to_string().clone();
    let xs = &x.to_string();
    let qs = xs.match_indices(&a);
    if qs.clone().count() < nth {
        panic!("Interchange Error: There quantification {} does not exist in {} {} times",e,x,nth);
    }
    for (pos,q) in qs.enumerate() {
        if pos == nth {
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
        let xs = theorem.replace_var(v, &(v << 1));
        let x0 = theorem.replace_var(v, &Number::new("0"));
        if x0.to_string() != base.to_string() {
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
            let lt = Expression::new(&format!("S{}",l));
            let rt = Expression::new(&format!("S{}",r));
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
                let lt = Expression::new(&l.strip_prefix("S").unwrap());
                let rt = Expression::new(&r.strip_prefix("S").unwrap());
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

pub fn symmetry(a: &Formula) -> Formula {
    if let Formula::Simple(_) = a {
        if let Some((l,r)) = split_eq(&a.to_string()) {
            let lt = Expression::new(&l);
            let rt = Expression::new(&r);
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
                    let lt = Expression::new(&l1);
                    let rt = Expression::new(&r2);
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
    use crate::types::Number;
    let a = &Variable::new("a");
    let one = &Number::new("S0");
    let formula1 = &Formula::new("Aa:a=a");
    let formula2 = &Formula::new("Ea':Aa:[a=a&a'=a']");
    assert_eq!(specification(formula1,a,one).to_string(),"S0=S0");
    assert_eq!(specification(formula2,a,one).to_string(),"Ea':[S0=S0&a'=a']");
}

#[test]
#[should_panic]
fn test_specification_err1() {
    use crate::types::Number;
    let a = &Variable::new("b");
    let one = &Number::new("S0");
    let formula1 = &Formula::new("Aa:a=a");
    specification(formula1,a,one);
}

#[test]
#[should_panic]
fn test_specification_err2() {
    use crate::types::Number;
    let a = &Variable::new("a");
    let one = &Number::new("S0");
    let formula1 = &Formula::new("Aa:a=a");
    specification(formula1,a,&(a+one));
}



#[test]
fn test_generalization() {
    let a = &Variable::new("a");
    let x = &Variable::new("x'");
    let formula1 = &Formula::new("a=a");
    let formula2 = &Formula::new("Ea':Aa:[a=a&x'=a']");
    assert_eq!(generalization(formula1,a).to_string(),"Aa:a=a");
    assert_eq!(generalization(formula2,x).to_string(),"Ax':Ea':Aa:[a=a&x'=a']");
}

#[test]
#[should_panic]
fn test_generalization_err() {
    let c = &Variable::new("c");
    let formula1 = &Formula::new("Ec:a=c");
    println!("{}",generalization(formula1,c));
}



#[test]
fn test_symmetry() {
    let simple1 = &Formula::new("a=b");
    let simple2 = &Formula::new("b=S(a+S0)");
    assert_eq!(symmetry(simple1).to_string(),"b=a");
    assert_eq!(symmetry(simple2).to_string(),"S(a+S0)=b");
}

#[test]
#[should_panic]
fn test_symmetry_err() {
    let complex = &Formula::new("Aa:a=b");
    symmetry(complex);
}



#[test]
fn test_transitivity() {
    let atom1 = Formula::new("a=b");
    let atom2 = Formula::new("b=S(a+S0)");
    assert_eq!(transitivity(&atom1,&atom2).to_string(),"a=S(a+S0)");
}

#[test]
#[should_panic]
fn test_transitivity_err_1_left() {
    let complex1 = &Formula::new("Aa:a=b");
    let complex2 = &Formula::new("p=j''");
    transitivity(complex1, complex2);
}

#[test]
#[should_panic]
fn test_transitivity_err_1_right() {
    let complex1 = &Formula::new("Aa:a=b");
    let complex2 = &Formula::new("p=j''");
    transitivity(complex2, complex1);
}

#[test]
#[should_panic]
fn test_transitivity_err_2() {
    let complex1 = &Formula::new("p=j''");
    let complex2 = &Formula::new("q'=p");
    transitivity(complex1, complex2);
}



#[test]
fn test_predecessor() {
    let simple = &Formula::new("Sm''=SSu");
    assert_eq!(predecessor(simple).to_string(),"m''=Su");
}

#[test]
#[should_panic]
fn test_predecessor_err_1() {
    let complex = &Formula::new("~Ei:(i+SS0)=g");
    predecessor(complex);
}

#[test]
#[should_panic]
fn test_predecessor_err_2() {
    let simple = &Formula::new("SSb'=0");
    predecessor(simple);
}



#[test]
fn test_successor() {
    let simple = &Formula::new("Sm''=SSu");
    assert_eq!(successor(simple).to_string(),"SSm''=SSSu");
}

#[test]
#[should_panic]
fn test_successor_err() {
    let complex = &Formula::new("~Ei:(i+SS0)=g");
    successor(complex);
}



#[test]
fn test_interchange_ea() {
    let formula1 = &Formula::new("Aa:~Eu':(a+u')=Sa");
    let formula2 = &Formula::new("[Aa:~Eu':(a+u')=Sa&~Eu':u'=SS0]");
    let variable = &Variable::new("u'");
    assert_eq!(interchange_ea(formula1,variable,0).to_string(),"Aa:Au':~(a+u')=Sa");
    assert_eq!(interchange_ea(formula2,variable,1).to_string(),"[Aa:~Eu':(a+u')=Sa&Au':~u'=SS0]");
}

#[test]
#[should_panic]
fn test_interchange_ea_err() {
    let formula1 = &Formula::new("Aa:~Eu':(a+u')=Sa");
    let variable = &Variable::new("z");
    interchange_ea(formula1,variable,0);
}



#[test]
fn test_interchange_ae() {
    let formula1 = &Formula::new("Aa:Au':~(a+u')=Sa");
    let formula2 = &Formula::new("[Aa:~Eu':(a+u')=Sa&Au':~u'=SS0]");
    let variable = &Variable::new("u'");
    assert_eq!(interchange_ae(formula1,variable,0).to_string(),"Aa:~Eu':(a+u')=Sa");
    assert_eq!(interchange_ae(formula2,variable,0).to_string(),"[Aa:~Eu':(a+u')=Sa&~Eu':u'=SS0]");
}



#[test]
fn test_induction() {
    let v = &Variable::new("v");
    let base = &Formula::new("0=0");
    let gen = &Formula::new("Av:[v=v>Sv=Sv]");
    assert_eq!(induction(v,base,gen).to_string(),"Av:v=v");
}
