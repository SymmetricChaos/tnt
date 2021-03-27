use crate::types::{Formula,Variable,Term,Expression,Number};
use crate::ops_construction::*;
use crate::string_manip::{split_eq, get_bound_vars, left_implies, get_vars};
use crate::errors::LogicError;

// Rules of production
// These may check for additional internal contraints and will panic on failure
pub fn specification<T: Term>(x: &Formula, v: &Variable, t: &T) -> Result<Formula,LogicError> {
    if x.to_string().contains(&format!("A{}:",v)) {
        let var_in_t = get_vars(&t.get_string());
        let bound_in_x = get_bound_vars(&x.to_string());
        for var in var_in_t {
            if bound_in_x.contains(&var) && t.get_string() != v.get_string() {
                let msg = format!("Specification Error: The Term `{}` contains the Variable `{}` which is already bound in the Formula `{}`",t.get_string(),var,x);
                return Err(LogicError::new(msg))
            }
        }
        return Ok(x.specify_var(v, t))
    } else {
        let msg = format!("Specification Error: The Variable `{}` is not univerally quantified in the Formula `{}`",v,x);
        return Err(LogicError::new(msg))
    }
}

pub fn generalization(x: &Formula, v: &Variable) -> Result<Formula,LogicError> {
    if !x.contains_var_bound(&v) {
        return Ok(forall(v,x))
    } else {
        let msg = format!("Generalization Error: The Variable `{}` is already bound in the Formula `{}`",v,x);
        return Err(LogicError::new(msg))
    }
}

pub fn existence<T: Term>(x: &Formula, t: &T, v: &Variable) -> Result<Formula,LogicError> {
    if !get_bound_vars(&x.to_string()).contains(&v.to_string()) {
        let out = exists(v,x);
        return Ok(Formula::new(&out.to_string().replace(&t.get_string(), &v.to_string())))
        
    } else {
        let msg = format!("Existence Error: the Variable `{}` is already bound in the formula `{}`",v,x);
        return Err(LogicError::new(msg))
    }
}

pub fn interchange_ea(x: &Formula, v: &Variable, nth: usize) -> Result<Formula,LogicError> {
    let e = format!("~E{}:",v);
    let a = format!("A{}:~",v);
    let mut new_s = x.to_string().clone();
    let xs = x.to_string();
    let qs = xs.match_indices(&e);
    let count = qs.clone().count();
    if count == 0 {
        let msg = format!("Interchange Error: The quantification `{}` does not exist in the Formula `{}`",e,x);
        return Err(LogicError::new(msg))
    }
    if count < nth {
        let msg = format!("Interchange Error: The quantification `{}` only appears {} times in the Formula `{}`",e,count,x);
        return Err(LogicError::new(msg))
    }
    for (pos,q) in qs.enumerate() {
        if pos == nth {
            new_s.replace_range(q.0..q.0+q.1.len(), &a);
            break
        }
    }
    Ok(Formula::new_complex(&new_s))
}

pub fn interchange_ae(x: &Formula, v: &Variable, nth: usize) -> Result<Formula,LogicError> {
    let e = format!("~E{}:",v);
    let a = format!("A{}:~",v);
    let mut new_s = x.to_string().clone();
    let xs = &x.to_string();
    let qs = xs.match_indices(&a);
    let count = qs.clone().count();
    if count == 0 {
        let msg = format!("Interchange Error: The quantification `{}` does not exist in the Formula `{}`",e,x);
        return Err(LogicError::new(msg))
    }
    if count < nth {
        let msg = format!("Interchange Error: The quantification `{}` only appears {} times in the Formula `{}`",e,count,x);
        return Err(LogicError::new(msg))
    }
    for (pos,q) in qs.enumerate() {
        if pos == nth {
            new_s.replace_range(q.0..q.0+q.1.len(), &e);
            break
        }
    }
    Ok(Formula::new_complex(&new_s))
}

pub fn induction(v: &Variable, base: &Formula, general: &Formula) -> Result<Formula,LogicError> {
    // The theorem we need to generalize is the outermost, leftmost implication of the general case
    let theorem = Formula::new(left_implies(&general.to_string()).unwrap());

    if get_bound_vars(&theorem.to_string()).contains(&v.to_string()) {
        let msg = format!("Induction Error: The Variable `{}` is already bound in the Formula `{}`, which is the left side of the general case {}",v,theorem,general);
        return Err(LogicError::new(msg))
    } else {
        let xs = theorem.replace_var(v, &(v << 1));
        let x0 = theorem.replace_var(v, &Number::zero());
        if x0.to_string() != base.to_string() {
            let msg = format!("Induction Error: The base case must be the Formula `{}`",x0);
            return Err(LogicError::new(msg))
        }
        if general.to_string() != format!("A{}:[{}>{}]",v,theorem,xs) {
            let msg = format!("Induction Error: The general case must be the Formula `A{}:[{}>{}]`",v,theorem,xs);
            return Err(LogicError::new(msg))
        }
        return Ok(forall(v,&theorem))
    }
}

pub fn successor(a: &Formula) -> Result<Formula,LogicError> {
    if let Formula::Simple(_) = a {
        if let Some((l,r)) = split_eq(&a.to_string()) {
            let lt = Expression::new(&format!("S{}",l));
            let rt = Expression::new(&format!("S{}",r));
            return Ok(eq(&lt,&rt))
        } else {
            unreachable!("Successor Error: unable to split {}",a)
        }
    } else {
        let msg = format!("Successor Error: {} is not a Formula::Simple which is required in order to split it",a);
        return Err(LogicError::new(msg))
    }
}

pub fn predecessor(a: &Formula) -> Result<Formula,LogicError> {
    if let Formula::Simple(_) = a {
        if let Some((l,r)) = split_eq(&a.to_string()) {
            if l.starts_with("S") && r.starts_with("S") {
                let lt = Expression::new(&l.strip_prefix("S").unwrap());
                let rt = Expression::new(&r.strip_prefix("S").unwrap());
                return Ok(eq(&lt,&rt))
            } else {
                let msg = format!("Predecessor Error: both terms the Formula `{}` must begin with S",a);
                return Err(LogicError::new(msg))
            }
        }
        unreachable!("Predecessor Error: unable to split {}",a)
    } else {
        let msg = format!("Predecessor Error: {} is not a Formula::Simple which is required in order to split it",a);
        return Err(LogicError::new(msg))
    }
}

pub fn symmetry(a: &Formula) -> Result<Formula,LogicError> {
    if let Formula::Simple(_) = a {
        if let Some((l,r)) = split_eq(&a.to_string()) {
            let lt = Expression::new(&l);
            let rt = Expression::new(&r);
            return Ok(eq(&rt,&lt))
        } else {
            unreachable!("Symmetry Error: unable to split {}",a)
        }
    } else {
        let msg = format!("Successor Error: {} is not a Formula::Simple which is required in order to split it",a);
        return Err(LogicError::new(msg))
    }
}

pub fn transitivity(a1: &Formula, a2: &Formula) -> Result<Formula,LogicError> {
    if let Formula::Simple(_) = a1 {
        if let Formula::Simple(_) = a2 {
            if let Some((l1,r1)) = split_eq(&a1.to_string()) {
                if let Some((l2,r2)) = split_eq(&a2.to_string()) {
                    if r1 != l2 {
                        let msg = format!("Transitivity Error: The right term of {} does not match the left term of {}",a1,a2);
                        return Err(LogicError::new(msg))
                    }
                    let lt = Expression::new(&l1);
                    let rt = Expression::new(&r2);
                    return Ok(eq(&lt,&rt))
                } else {
                    unreachable!("Transitivity Error: unable to split {}",a2)
                }
            } else {
                unreachable!("Transitivity Error: unable to split {}",a1)
            }
        } else {
            let msg = format!("Transitivity Error: {} is not a Formula::Simple which is required in order to split it",a2);
            return Err(LogicError::new(msg))
        }
    } else {
        let msg = format!("Transitivity Error: {} is not a Formula::Simple which is required in order to split it",a1);
        return Err(LogicError::new(msg))
    }
}





// TODO: test pathalogical inputs for all of these
// TODO: test panic modes for all of these

#[test]
fn test_specification() -> Result<(),LogicError> {
    use crate::types::Number;
    let a = &Variable::new("a");
    let one = &Number::new("S0");
    let formula1 = &Formula::new("Aa:a=a");
    let formula2 = &Formula::new("Ea':Aa:[a=a&a'=a']");
    assert_eq!(specification(formula1,a,one)?.to_string(),"S0=S0");
    assert_eq!(specification(formula2,a,one)?.to_string(),"Ea':[S0=S0&a'=a']");
    Ok(())
}

#[test]
fn test_specification_err1() {
    use crate::types::Number;
    let a = &Variable::new("b");
    let one = &Number::new("S0");
    let formula1 = &Formula::new("Aa:a=a");
    assert!(specification(formula1,a,one).is_err());
}

#[test]
fn test_specification_err2() {
    use crate::types::Number;
    let a = &Variable::new("a");
    let one = &Number::new("S0");
    let formula1 = &Formula::new("Aa:a=a");
    assert!(specification(formula1,a,&(a+one)).is_err());
}



#[test]
fn test_generalization() -> Result<(),LogicError> {
    let a = &Variable::new("a");
    let x = &Variable::new("x'");
    let formula1 = &Formula::new("a=a");
    let formula2 = &Formula::new("Ea':Aa:[a=a&x'=a']");
    assert_eq!(generalization(formula1,a)?.to_string(),"Aa:a=a");
    assert_eq!(generalization(formula2,x)?.to_string(),"Ax':Ea':Aa:[a=a&x'=a']");
    Ok(())
}

#[test]
fn test_generalization_err() {
    let c = &Variable::new("c");
    let formula1 = &Formula::new("Ec:a=c");
    assert!(generalization(formula1,c).is_err());
}



#[test]
fn test_symmetry() -> Result<(),LogicError> {
    let simple1 = &Formula::new("a=b");
    let simple2 = &Formula::new("b=S(a+S0)");
    assert_eq!(symmetry(simple1)?.to_string(),"b=a");
    assert_eq!(symmetry(simple2)?.to_string(),"S(a+S0)=b");
    Ok(())
}

#[test]
fn test_symmetry_err() {
    let complex = &Formula::new("Aa:a=b");
    assert!(symmetry(complex).is_err());
}



#[test]
fn test_transitivity() -> Result<(),LogicError> {
    let atom1 = Formula::new("a=b");
    let atom2 = Formula::new("b=S(a+S0)");
    assert_eq!(transitivity(&atom1,&atom2)?.to_string(),"a=S(a+S0)");
    Ok(())
}

#[test]
fn test_transitivity_err_1_left() {
    let complex1 = &Formula::new("Aa:a=b");
    let complex2 = &Formula::new("p=j''");
    assert!(transitivity(complex1, complex2).is_err());
}

#[test]
fn test_transitivity_err_1_right() {
    let complex1 = &Formula::new("Aa:a=b");
    let complex2 = &Formula::new("p=j''");
    assert!(transitivity(complex2, complex1).is_err());
}

#[test]
fn test_transitivity_err_2() {
    let complex1 = &Formula::new("p=j''");
    let complex2 = &Formula::new("q'=p");
    assert!(transitivity(complex1, complex2).is_err());
}



#[test]
fn test_predecessor() -> Result<(),LogicError>  {
    let simple = &Formula::new("Sm''=SSu");
    assert_eq!(predecessor(simple)?.to_string(),"m''=Su");
    Ok(())
}

#[test]
fn test_predecessor_err_1() {
    let complex = &Formula::new("~Ei:(i+SS0)=g");
    assert!(predecessor(complex).is_err());
}

#[test]
fn test_predecessor_err_2() {
    let simple = &Formula::new("SSb'=0");
    assert!(predecessor(simple).is_err());
}



#[test]
fn test_successor() -> Result<(),LogicError>  {
    let simple = &Formula::new("Sm''=SSu");
    assert_eq!(successor(simple)?.to_string(),"SSm''=SSSu");
    Ok(())
}

#[test]
fn test_successor_err() {
    let complex = &Formula::new("~Ei:(i+SS0)=g");
    assert!(successor(complex).is_err());
}



#[test]
fn test_interchange_ea() -> Result<(),LogicError> {
    let formula1 = &Formula::new("Aa:~Eu':(a+u')=Sa");
    let formula2 = &Formula::new("[Aa:~Eu':(a+u')=Sa&~Eu':u'=SS0]");
    let variable = &Variable::new("u'");
    assert_eq!(interchange_ea(formula1,variable,0)?.to_string(),"Aa:Au':~(a+u')=Sa");
    assert_eq!(interchange_ea(formula2,variable,1)?.to_string(),"[Aa:~Eu':(a+u')=Sa&Au':~u'=SS0]");
    Ok(())
}

#[test]
fn test_interchange_ea_err() {
    let formula1 = &Formula::new("Aa:~Eu':(a+u')=Sa");
    let variable = &Variable::new("z");
    assert!(interchange_ea(formula1,variable,0).is_err());
}



#[test]
fn test_interchange_ae() -> Result<(),LogicError> {
    let formula1 = &Formula::new("Aa:Au':~(a+u')=Sa");
    let formula2 = &Formula::new("[Aa:~Eu':(a+u')=Sa&Au':~u'=SS0]");
    let variable = &Variable::new("u'");
    assert_eq!(interchange_ae(formula1,variable,0)?.to_string(),"Aa:~Eu':(a+u')=Sa");
    assert_eq!(interchange_ae(formula2,variable,0)?.to_string(),"[Aa:~Eu':(a+u')=Sa&~Eu':u'=SS0]");
    Ok(())
}

#[test]
fn test_interchange_ae_err() {
    let formula1 = &Formula::new("Aa:~Eu':(a+u')=Sa");
    let variable = &Variable::new("z");
    assert!(interchange_ae(formula1,variable,0).is_err());
}



#[test]
fn test_induction() -> Result<(),LogicError> {
    let v = &Variable::new("v");
    let base = &Formula::new("0=0");
    let gen = &Formula::new("Av:[v=v>Sv=Sv]");
    assert_eq!(induction(v,base,gen)?.to_string(),"Av:v=v");
    Ok(())
}