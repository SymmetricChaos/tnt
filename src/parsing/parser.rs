use pest::{Parser, iterators::Pair};

use crate::parsing::syntax_tree::LogicOp;

use super::syntax_tree::{ArithmeticOp, Quantifier, TntNode};


#[derive(Parser)]
#[grammar = "parsing\\tnt.pest"]
pub struct TntParser;


pub fn parse_proof(proof: &str) -> Result<Vec<TntNode>,pest::error::Error<Rule>> {
    let lines = proof.split("\n");
    let mut vec = Vec::new();
    for line in lines {
        let ast = formula_str_to_ast(line)?;
        vec.push(ast)
    }
    Ok(vec)
}

pub fn print_parse_tree(text: &str, rule: Rule) -> Result<(),pest::error::Error<Rule>> {
    let tree = TntParser::parse(rule, text)?;
    println!("{:#?}",tree);
    Ok(())
}

pub fn str_to_ast(text: &str, rule: Rule) -> Result<TntNode,pest::error::Error<Rule>> {
    let mut tree = TntParser::parse(rule, text)?;
    Ok(build_ast(tree.next().unwrap()))
}

pub fn formula_str_to_ast(text: &str) -> Result<TntNode,pest::error::Error<Rule>> {
    let mut tree = TntParser::parse(Rule::formula, text)?;
    Ok(build_ast(tree.next().unwrap()))
}

pub fn expression_str_to_ast(text: &str) -> Result<TntNode,pest::error::Error<Rule>> {
    let mut tree = TntParser::parse(Rule::expression, text)?;
    Ok(build_ast(tree.next().unwrap()))
}

pub fn build_ast(pair: Pair<Rule>) -> TntNode {
    match pair.as_rule() {
        // Expressions and formulas just mask more specific rules that we want to use
        Rule::formula => build_ast(pair.into_inner().next().unwrap()),
        Rule::expression => build_ast(pair.into_inner().next().unwrap()),

        // Numbers are variables are terminal symbols that we immediately capture
        Rule::number => TntNode::Number(pair.as_str().to_string()),
        Rule::variable => TntNode::Variable(pair.as_str().to_string()),

        Rule::equality => {
            let mut t = pair.into_inner();
            let lhs = t.next().unwrap();
            let rhs = t.next().unwrap();
            TntNode::Equality(Box::new(build_ast(lhs)),Box::new(build_ast(rhs)))
        },
        Rule::logical_formula => expand_logical_formula(pair),
        Rule::quantification => expand_quantification(pair),

        Rule::arithmetic_expr => expand_arithmetic_expr(pair),

        // Unary nodes
        Rule::negated_quantification => TntNode::Negation( Box::new( build_ast(pair.into_inner().next().unwrap())) ),
        Rule::successor_expr => TntNode::Successor( Box::new( build_ast(pair.into_inner().next().unwrap())) ),
        _ => unreachable!()
    }
}

fn match_logical_op(s: &str) -> LogicOp {
    match s {
        "&" => LogicOp::And,
        "|" => LogicOp::Or,
        ">" => LogicOp::Implies,
        "∧" => LogicOp::And,
        "∨" => LogicOp::Or,
        "⇒" => LogicOp::Implies,
        _ => unreachable!()
    }
}

fn match_arithmetic_op(s: &str) -> ArithmeticOp {
    match s {
        "+" => ArithmeticOp::Add,
        "*" => ArithmeticOp::Mul,
        "×" => ArithmeticOp::Mul,
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

fn expand_quantifier(pair: Pair<Rule>) -> Quantifier {
    match pair.as_rule() {
        Rule::existential => {
            let var = build_ast(pair.into_inner().next().unwrap());
            Quantifier::Existential(Box::new(var))
        },
        Rule::universal => {
            let var = build_ast(pair.into_inner().next().unwrap());
            Quantifier::Universal(Box::new(var))
        },
        _ => unreachable!("not universal of existential")
    }
}

fn expand_quantification(pair: Pair<Rule>) -> TntNode {
    let t = pair.into_inner().next().unwrap();
    match t.as_rule() {
        Rule::quantified_formula => {
            let mut q = t.into_inner();
            let quant = expand_quantifier(q.next().unwrap());
            let formula =  Box::new(build_ast(q.next().unwrap()));
            TntNode::Quantification(quant, formula)
        },
        Rule::negated_quantification => {
            let mut q = t.into_inner();
            let quantification =  Box::new(expand_quantification(q.next().unwrap()));
            TntNode::Negation(quantification)
        },
        _ => unreachable!()
    }

}





// Formulas
#[test]
fn test_simple_equality() {
    let tnt = "S0=a''";
    let ast = formula_str_to_ast(tnt).unwrap();
    assert_eq!(tnt,format!("{}",&ast));
}

#[test]
fn test_compound_equality() {
    let tnt = "S0=(b+b)";
    let ast = formula_str_to_ast(tnt).unwrap();
    assert_eq!(tnt,format!("{}",&ast));
}

#[test]
fn test_quantification() {
    let tnt = "~~Ea':z=a";
    let ast = formula_str_to_ast(tnt).unwrap();
    assert_eq!(tnt,format!("{}",&ast));
}

#[test]
fn test_complex_formula() {
    let tnt = "Aa:Ab:(a*Sb)=((a*b)+a)";
    let ast = formula_str_to_ast(tnt).unwrap();
    assert_eq!(tnt,format!("{}",&ast));
}

#[test]
fn test_very_complex_formula() {
    let tnt = "Aa:[Ec:(a*c)=b>Ed:(d*SS0)=a]";
    let ast = formula_str_to_ast(tnt).unwrap();
    assert_eq!(tnt,format!("{}",&ast));
    println!("{}",&ast.pretty_print());
}




// Expressions
#[test]
fn test_addition() {
    let tnt = "(0+Sa'')";
    let ast = expression_str_to_ast(tnt).unwrap();
    assert_eq!(tnt,format!("{}",&ast));
}

#[test]
fn test_multiplication() {
    let tnt = "(x''*SSSSSS0)";
    let ast = expression_str_to_ast(tnt).unwrap();
    assert_eq!(tnt,format!("{}",&ast));
}

#[test]
fn test_complex_arithmetic() {
    let tnt = "SS((b+S0)*Sa'')";
    let ast = expression_str_to_ast(tnt).unwrap();
    assert_eq!(tnt,format!("{}",&ast));
}
