use std::fmt::Display;

#[derive(Debug)]
pub enum Quantifier {
    Existential(Box<TntNode>),
    Universal(Box<TntNode>),
}

impl Display for Quantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quantifier::Existential(v) => write!(f, "E{}:",v.to_string()),
            Quantifier::Universal(v) => write!(f, "A{}:",v.to_string()),
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
    Quantification(String, Quantifier, Box<TntNode>),
    Number(String),
    Variable(String),
}

impl Display for TntNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TntNode::Equality(lhs, rhs) => write!(f, "{}={}", lhs, rhs),
            TntNode::Arithmetic(op, lhs, rhs) => write!(f, "({}{}{})", lhs, op, rhs),
            TntNode::Logical(op, lhs, rhs) => write!(f, "[{}{}{}]", lhs, op, rhs),
            TntNode::Successor(expression) => write!(f, "S{}", expression),
            TntNode::Quantification(neg, q, formula) => write!(f, "{}{}{}", neg, q, formula),
            TntNode::Number(n) => write!(f, "{}", n),
            TntNode::Variable(v) => write!(f, "{}", v),
        }
    }
}