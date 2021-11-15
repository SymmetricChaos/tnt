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

impl Quantifier {
    pub fn pretty_print(&self) -> String {
        match self {
            Quantifier::Existential(v) => format!("∃{}:",v.to_string()),
            Quantifier::Universal(v) => format!("∀{}:",v.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum LogicOp {
    And,
    Or,
    Implies
}

impl LogicOp {
    pub fn pretty_print(&self) -> String {
        match self {
            LogicOp::And => format!("∧"),
            LogicOp::Or => format!("∨"),
            LogicOp::Implies => format!("⇒"),
        }
    }
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

impl ArithmeticOp {
    pub fn pretty_print(&self) -> String {
        match self {
            ArithmeticOp::Add => format!("+"),
            ArithmeticOp::Mul => format!("×"),
        }
    }
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
    Negation(Box<TntNode>),
    Quantification(String, Quantifier, Box<TntNode>),
    Number(String),
    Variable(String),
}

impl TntNode {
    pub fn pretty_print(&self) -> String {
        match self {
            TntNode::Equality(lhs, rhs) => format!("{}={}", lhs.pretty_print(), rhs.pretty_print()),
            TntNode::Arithmetic(op, lhs, rhs) => format!("({}{}{})", lhs.pretty_print(), op.pretty_print(), rhs.pretty_print()),
            TntNode::Logical(op, lhs, rhs) => format!("[{}{}{}]", lhs.pretty_print(), op.pretty_print(), rhs.pretty_print()),
            TntNode::Successor(expression) => format!("S{}",expression.pretty_print()),
            TntNode::Negation(quantification) => format!("¬{}",quantification.pretty_print()),
            TntNode::Quantification(neg, q, formula) => format!("{}{}{}", neg, q.pretty_print(), formula.pretty_print()),
            TntNode::Number(n) => format!("{}",n),
            TntNode::Variable(v) => format!("{}",v),
        }
    }
}

impl Display for TntNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TntNode::Equality(lhs, rhs) => write!(f, "{}={}", lhs, rhs),
            TntNode::Arithmetic(op, lhs, rhs) => write!(f, "({}{}{})", lhs, op, rhs),
            TntNode::Logical(op, lhs, rhs) => write!(f, "[{}{}{}]", lhs, op, rhs),
            TntNode::Successor(expression) => write!(f, "S{}", expression),
            TntNode::Negation(quantification) => write!(f, "~{}",quantification),
            TntNode::Quantification(neg, q, formula) => write!(f, "{}{}{}", neg, q, formula),
            TntNode::Number(n) => write!(f, "{}", n),
            TntNode::Variable(v) => write!(f, "{}", v),
        }
    }
}