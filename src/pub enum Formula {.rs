pub enum Formula {
    Simple,
    Complex,
}
pub struct Simple {
    pub s: String,
}
pub struct Complex {
    pub s: String,
}





pub enum Term {
    Variable,
    Number,
    Equation,
}

pub struct Variable {
    pub s: String,
}

pub struct Number {
    pub s: String,
    pub bound_vars: Vec<String>
}

pub struct Equation {
    pub s: String,
}
