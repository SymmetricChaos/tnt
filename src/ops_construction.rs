use crate::types::{Term,Formula};

// Rules of construction. 

// Arithmetic
pub fn succ(x: &Term) -> Term {
    let new_s = format!("S{}",x);
    Term::new(&new_s)
}

pub fn add(x: &Term, y: &Term) -> Term {
    let new_s = format!("({}+{})",x,y);
    Term::new_equation(&new_s)
}

pub fn mul(x: &Term, y: &Term) -> Term {
    let new_s = format!("({}*{})",x,y);
    Term::new_equation(&new_s)
}


// Logical
pub fn eq(x: &Term, y: &Term) -> Formula {
    let new_s = format!("{}={}",x,y);
    Formula::new_simple(&new_s)
}

pub fn not(x: &Formula) -> Formula {
    let new_s = format!("~{}",x);
    Formula::new_complex(&new_s)
}

pub fn or(x: &Formula, y: &Formula) -> Formula {
    let new_s = format!("[{}|{}]",x,y);
    Formula::new_complex(&new_s)
}

pub fn and(x: &Formula, y: &Formula) -> Formula {
    let new_s = format!("[{}&{}]",x,y);
    Formula::new_complex(&new_s)
}

pub fn implies(x: &Formula, y: &Formula) -> Formula {
    let new_s = format!("[{}>{}]",x,y);
    Formula::new_complex(&new_s)
}


// Quantification
// Maybe some way to avoid panic here
pub fn exists(v: &Term, x: &Formula) -> Formula {
    if let Term::Variable(var) = v {
        Formula::new_complex(&format!("E{}:{}",var,x))
    } else {
        panic!("{} is not a Term::Variable",v)
    }
}

pub fn forall(v: &Term, x: &Formula) -> Formula {
    if let Term::Variable(var) = v {
        Formula::new_complex(&format!("A{}:{}",var,x))
    } else {
        panic!("{} is not a Term::Variable",v)
    }
}


// TODO: test pathalogical inputs for all of these

#[test]
fn test_succ() {
    let a = &Term::new("a");
    let x = &Term::new("x''");
    let zero = &Term::new("0");
    let one = &Term::new("S0");
    let equation = &Term::new("((b*SSS0)+Sv')");
    assert_eq!(succ(a).to_string(),"Sa");
    assert_eq!(succ(x).to_string(),"Sx''");
    assert_eq!(succ(zero).to_string(),"S0");
    assert_eq!(succ(one).to_string(),"SS0");
    assert_eq!(succ(equation).to_string(),"S((b*SSS0)+Sv')");
}

#[test]
fn test_add() {
    let a = &Term::new("a");
    let x = &Term::new("x''");
    let zero = &Term::new("0");
    let one = &Term::new("S0");
    let equation = &Term::new("((b*SSS0)+Sv')");
    assert_eq!(add(a,x).to_string(),"(a+x'')");
    assert_eq!(add(x,zero).to_string(),"(x''+0)");
    assert_eq!(add(equation,one).to_string(),"(((b*SSS0)+Sv')+S0)");
}

#[test]
fn test_mul() {
    let a = &Term::new("a");
    let x = &Term::new("x''");
    let zero = &Term::new("0");
    let one = &Term::new("S0");
    let equation = &Term::new("((b*SSS0)+Sv')");
    assert_eq!(mul(a,x).to_string(),"(a*x'')");
    assert_eq!(mul(x,zero).to_string(),"(x''*0)");
    assert_eq!(mul(equation,one).to_string(),"(((b*SSS0)+Sv')*S0)");
}

#[test]
fn test_eq() {
    let a = &Term::new("a");
    let x = &Term::new("x''");
    let zero = &Term::new("0");
    let one = &Term::new("S0");
    let equation = &Term::new("((b*SSS0)+Sv')");
    assert_eq!(eq(a,x).to_string(),"a=x''");
    assert_eq!(eq(x,zero).to_string(),"x''=0");
    assert_eq!(eq(equation,one).to_string(),"((b*SSS0)+Sv')=S0");
}

#[test]
fn test_not() {
    let a = &Formula::new("a=a");
    let b = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
    assert_eq!(not(a).to_string(),"~a=a");
    assert_eq!(not(b).to_string(),"~~Ab':[(b'+b')=S0|S0=b']");
}

#[test]
fn test_or() {
    let a = &Formula::new("a=a");
    let b = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
    let c = &Formula::new("Ec:(c+S0)=0");
    assert_eq!(or(a,b).to_string(),"[a=a|~Ab':[(b'+b')=S0|S0=b']]");
    assert_eq!(or(a,c).to_string(),"[a=a|Ec:(c+S0)=0]");
}

#[test]
fn test_and() {
    let a = &Formula::new("a=a");
    let b = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
    let c = &Formula::new("Ec:(c+S0)=0");
    assert_eq!(and(a,b).to_string(),"[a=a&~Ab':[(b'+b')=S0|S0=b']]");
    assert_eq!(and(a,c).to_string(),"[a=a&Ec:(c+S0)=0]");
}

#[test]
fn test_implies() {
    let a = &Formula::new("a=a");
    let b = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
    let c = &Formula::new("Ec:(c+S0)=0");
    assert_eq!(implies(a,b).to_string(),"[a=a>~Ab':[(b'+b')=S0|S0=b']]");
    assert_eq!(implies(a,c).to_string(),"[a=a>Ec:(c+S0)=0]");
}

#[test]
fn test_exists() {
    let a = &Term::new("a");
    let x = &Term::new("x''");
    let f0 = &Formula::new("a=a");
    let f1 = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
    assert_eq!(exists(a,f0).to_string(),"Ea:a=a");
    assert_eq!(exists(x,f1).to_string(),"Ex'':~Ab':[(b'+b')=S0|S0=b']");
}

#[test]
#[should_panic]
fn test_exists_panic() {
    let a = &Term::new("0");
    let f0 = &Formula::new("a=a");
    assert_eq!(exists(a,f0).to_string(),"Ea:a=a");
}

#[test]
fn test_forall() {
    let a = &Term::new("a");
    let x = &Term::new("x''");
    let f0 = &Formula::new("a=a");
    let f1 = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
    assert_eq!(forall(a,f0).to_string(),"Aa:a=a");
    assert_eq!(forall(x,f1).to_string(),"Ax'':~Ab':[(b'+b')=S0|S0=b']");
}

#[test]
#[should_panic]
fn test_forall_panic() {
    let a = &Term::new("0");
    let f0 = &Formula::new("a=a");
    assert_eq!(forall(a,f0).to_string(),"Ea:a=a");
}
