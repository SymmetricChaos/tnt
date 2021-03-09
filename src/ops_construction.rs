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
pub fn not(x: &Formula) -> Formula {
    let new_s = format!("~{}",x);
    Formula::new_complex(&new_s)
}

pub fn eq(x: &Term, y: &Term) -> Formula {
    let new_s = format!("{}={}",x,y);
    Formula::new_simple(&new_s)
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


// TODO: test pathalogical inputs
#[test]
fn test_add() {
    let a = Term::new_variable("a");
    let b = Term::new_variable("b");
    assert_eq!(add(&a,&b).to_string(),"(a+b)");
    assert_eq!(add(&a,&add(&b,&b)).to_string(),"(a+(b+b))");
}