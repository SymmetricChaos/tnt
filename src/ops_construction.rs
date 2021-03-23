use crate::types::{Term,Formula,Variable};

// Rules of construction. 
// Arithmetic construction is handled by degining +, *, and << for Term types to represent
// addition, multiplication, and successor respectively

// Logical
pub fn eq<A: Term, B: Term>(x: &A, y: &B) -> Formula {
    let new_s = format!("{}={}",x.get_string(),y.get_string());
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
pub fn exists(v: &Variable, x: &Formula) -> Formula {
    Formula::new_complex(&format!("E{}:{}",v,x))
}

pub fn forall(v: &Variable, x: &Formula) -> Formula {
    Formula::new_complex(&format!("A{}:{}",v,x))
}


// TODO: test pathalogical inputs for all of these


#[test]
fn test_eq() {
    use crate::types::{Number,Equation};
    let a = &Variable::new("a");
    let x = &Variable::new("x''");
    let zero = &Number::new("0");
    let one = &Number::new("S0");
    let equation = &Equation::new("((b*SSS0)+Sv')");
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
    let a = &Variable::new("a");
    let x = &Variable::new("x''");
    let f0 = &Formula::new("a=a");
    let f1 = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
    assert_eq!(exists(a,f0).to_string(),"Ea:a=a");
    assert_eq!(exists(x,f1).to_string(),"Ex'':~Ab':[(b'+b')=S0|S0=b']");
}

#[test]
fn test_forall() {
    let a = &Variable::new("a");
    let x = &Variable::new("x''");
    let f0 = &Formula::new("a=a");
    let f1 = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
    assert_eq!(forall(a,f0).to_string(),"Aa:a=a");
    assert_eq!(forall(x,f1).to_string(),"Ax'':~Ab':[(b'+b')=S0|S0=b']");
}
