
pub enum Quant {
    Existential(String),
    Universal(String)
}

pub enum LogicOp {
    And,
    Or,
    Implies
}

pub enum ArithmeticOp {
    Add,
    Mul
}

pub enum TntNode {
    Equality(Box<TntNode>, Box<TntNode>),
    Arithmetic(ArithmeticOp, Box<TntNode>, Box<TntNode>),
    Logical(LogicOp, Box<TntNode>, Box<TntNode>),
    Successor(Box<TntNode>),
    Quantification(Quant, Box<TntNode>),
    Number(String),
    Variable(String),
}