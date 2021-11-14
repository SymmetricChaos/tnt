use std::fmt::Display;

#[derive(Debug)]
pub enum Quant {
    Existential(String),
    Universal(String)
}

impl Display for Quant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quant::Existential(v) => write!(f, "E{}:", v),
            Quant::Universal(v) => write!(f, "A{}:", v),
        }
    }
}

#[derive(Debug)]
pub enum LogicOp {
    And,
    Or,
    Implies
}

impl Display for LogicOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicOp::And => write!(f, "&"),
            LogicOp::Or => write!(f, "|"),
            LogicOp::Implies => write!(f, ">"),
        }
    }
}

#[derive(Debug)]
pub enum ArithmeticOp {
    Add,
    Mul
}

impl Display for ArithmeticOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmeticOp::Add => write!(f, "+"),
            ArithmeticOp::Mul => write!(f, "*"),
        }
    }
}

#[derive(Debug)]
pub enum TntNode {
    Equality(Box<TntNode>, Box<TntNode>),
    Arithmetic(ArithmeticOp, Box<TntNode>, Box<TntNode>),
    Logical(LogicOp, Box<TntNode>, Box<TntNode>),
    Successor(Box<TntNode>),
    Quantification(Quant, Box<TntNode>),
    Number(String),
    Variable(String),
}

impl Display for TntNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TntNode::Equality(lhs, rhs) => write!(f, "{}={}", lhs, rhs),
            TntNode::Arithmetic(op, lhs, rhs) => write!(f, "{}{}{}", lhs, op, rhs),
            TntNode::Logical(op, lhs, rhs) => write!(f, "{}{}{}", lhs, op, rhs),
            TntNode::Successor(contents) => write!(f, "S{}", contents),
            TntNode::Quantification(q, contents) => write!(f, "{}{}", q, contents),
            TntNode::Number(n) => write!(f, "{}", n),
            TntNode::Variable(v) => write!(f, "{}", v),
        }
    }
}