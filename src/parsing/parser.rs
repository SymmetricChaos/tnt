use pest::{Parser, iterators::Pair};

use crate::parsing::syntax_tree::LogicOp;

use super::syntax_tree::{ArithmeticOp, TntNode};


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
        Rule::quantification => todo!(),

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


#[test]
fn test_simple_equality() {
    let tnt = "S0=a''";
    let ast = str_to_ast(tnt, Rule::equality);
    assert_eq!(tnt,format!("{}",ast.unwrap()));
}

#[test]
fn test_simple_arithmetic() {
    let tnt = "(S0+a'')";
    let ast = str_to_ast(tnt,  Rule::arithmetic_expr);
    assert_eq!(tnt,format!("{}",ast.unwrap()));
}
