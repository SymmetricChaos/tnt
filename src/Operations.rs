use crate::Types::{Term,Formula};

pub fn add(x: &Term, y: &Term) -> Term {
    let new_s = format!("({}+{})",x.s,y.s);
    Term{s: new_s}
}

pub fn mul(x: &Term, y: &Term) -> Term {
    let new_s = format!("({}·{})",x.s,y.s);
    Term{s: new_s}
}

pub fn succ(x: &Term) -> Term {
    let new_s = format!("S{}",x.s);
    Term{s: new_s}
}

pub fn eq(x: &Term, y: &Term) -> Formula {
    let new_s = format!("{}={}",x.s,y.s);
    Formula{s: new_s}
}