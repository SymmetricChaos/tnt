use pest::{Parser, iterators::Pair};

use crate::parsing::syntax_tree::LogicOp;

use super::syntax_tree::{ArithmeticOp, Quantifier, TntNode};


#[derive(Parser)]
#[grammar = "parsing\\tnt.pest"]
pub struct TntParser;


pub fn str_to_ast(text: &str, rule: Rule) -> Result<TntNode,pest::error::Error<Rule>> {
    let mut tree = TntParser::parse(rule, text)?;
    Ok(build_ast(tree.next().unwrap()))
}

pub fn formula_str_to_ast(text: &str) -> Result<TntNode,pest::error::Error<Rule>> {
    let mut tree = TntParser::parse(Rule::formula, text)?;
    Ok(build_ast(tree.next().unwrap()))
}

pub fn expression_str_to_ast(text: &str) -> Result<TntNode,pest::error::Error<Rule>> {
    let mut tree = TntParser::parse(Rule::expr, text)?;
    Ok(build_ast(tree.next().unwrap()))
}

pub fn build_ast(pair: Pair<Rule>) -> TntNode {
    match pair.as_rule() {
        // Expressions and formulas just mask more specific rules that we want to use
        Rule::formula => build_ast(pair.into_inner().next().unwrap()),
        Rule::expr => build_ast(pair.into_inner().next().unwrap()),

        // Numbers are variables are terminal symbols that we immediately capture
        Rule::num => TntNode::Number(pair.as_str().to_string()),
        Rule::var => TntNode::Variable(pair.as_str().to_string()),

        Rule::equality => {
            let mut t = pair.into_inner();
            let lhs = t.next().unwrap();
            let rhs = t.next().unwrap();
            TntNode::Equality(Box::new(build_ast(lhs)),Box::new(build_ast(rhs)))
        },
        Rule::logical_formula => expand_logical_formula(pair),
        Rule::quantification => expand_quantification(pair),

        Rule::arithmetic_expr => expand_arithmetic_expr(pair),
        Rule::successor_expr => TntNode::Successor( Box::new( build_ast(pair.into_inner().next().unwrap())) ),
        _ => unreachable!()
    }
}

fn match_logical_op(s: &str) -> LogicOp {
    match s {
        "&" => LogicOp::And,
        "|" => LogicOp::Or,
        ">" => LogicOp::Implies,
        _ => unreachable!()
    }
}

fn match_arithmetic_op(s: &str) -> ArithmeticOp {
    match s {
        "+" => ArithmeticOp::Add,
        "*" => ArithmeticOp::Mul,
        _ => unreachable!()
    }
}

fn expand_logical_formula(pair: Pair<Rule>) -> TntNode {
    let mut t = pair.into_inner();
    let lhs = t.next().unwrap();
    let op = match_logical_op(t.next().unwrap().as_str());
    let rhs = t.next().unwrap();
    TntNode::Logical(op, Box::new(build_ast(lhs)), Box::new(build_ast(rhs)))
}

fn expand_arithmetic_expr(pair: Pair<Rule>) -> TntNode {
    let mut t = pair.into_inner();
    let lhs = t.next().unwrap();
    let op = match_arithmetic_op(t.next().unwrap().as_str());
    let rhs = t.next().unwrap();
    TntNode::Arithmetic(op, Box::new(build_ast(lhs)), Box::new(build_ast(rhs)))
}

fn expand_quantifier(pair: Pair<Rule>) -> (String,Quantifier) {
    let mut t = pair.into_inner();
    let negation = t.next().unwrap().as_str().to_string();
    let quant = {
        let q = t.next().unwrap();
        match q.as_rule() {
            Rule::existential => {
                let var = build_ast(q.into_inner().next().unwrap());
                Quantifier::Existential(Box::new(var))
            },
            Rule::universal => {
                let var = build_ast(q.into_inner().next().unwrap());
                Quantifier::Universal(Box::new(var))
            },
            _ => unreachable!()
        }
    };
    (negation,quant)
}

fn expand_quantification(pair: Pair<Rule>) -> TntNode {
    let mut t = pair.into_inner();
    let (negation, quant) = expand_quantifier(t.next().unwrap());
    let contents =  Box::new(build_ast(t.next().unwrap()));
    TntNode::Quantification(negation, quant, contents)
}





// Formulas
#[test]
fn test_simple_equality() {
    let tnt = "S0=a''";
    let ast = formula_str_to_ast(tnt);
    assert_eq!(tnt,format!("{}",ast.unwrap()));
}

#[test]
fn test_compound_equality() {
    let tnt = "S0=(b+b)";
    let ast = formula_str_to_ast(tnt);
    assert_eq!(tnt,format!("{}",ast.unwrap()));
}

#[test]
fn test_quantification() {
    let tnt = "~~Ea:a=a";
    let ast = formula_str_to_ast(tnt);
    assert_eq!(tnt,format!("{}",ast.unwrap()));
}




// Expressions
#[test]
fn test_addition() {
    let tnt = "(0+Sa'')";
    let ast = expression_str_to_ast(tnt);
    assert_eq!(tnt,format!("{}",ast.unwrap()));
}

#[test]
fn test_multiplication() {
    let tnt = "(x''*SSSSSS0)";
    let ast = expression_str_to_ast(tnt);
    assert_eq!(tnt,format!("{}",ast.unwrap()));
}

#[test]
fn test_complex_arithmetic() {
    let tnt = "SS((b+S0)*Sa'')";
    let ast = expression_str_to_ast(tnt);
    assert_eq!(tnt,format!("{}",ast.unwrap()));
}
