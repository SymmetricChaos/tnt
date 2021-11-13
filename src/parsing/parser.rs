use pest::{Parser, iterators::Pair};

use super::syntax_tree::TntNode;


#[derive(Parser)]
#[grammar = "parsing\\tnt.pest"]
pub struct TntParser;

pub fn build_ast(pair: Pair<Rule>) -> TntNode {
    match pair.as_rule() {
        // Expressions and formulas just mask more specific rules that we want to use
        Rule::formula => build_ast(pair.into_inner().next().unwrap()),
        Rule::expr => build_ast(pair.into_inner().next().unwrap()),
        Rule::equality => todo!(),
        Rule::logical_formula => todo!(),
        Rule::logical_op => todo!(),
        Rule::quantification => todo!(),
        Rule::quant => todo!(),

        Rule::num => TntNode::Number(pair.as_str().to_string()),
        Rule::var => TntNode::Variable(pair.as_str().to_string()),
        Rule::arithmetic_expr => todo!(),
        Rule::arith_op => todo!(),
        Rule::successor_expr => todo!(),
    }
}

#[test]
fn simple_test() {
    let tnt = "0=a'";
    let tree = TntParser::parse(Rule::formula, tnt)
    .expect("unsuccessful parse") // unwrap the parse result, might fail
    .next().unwrap(); // get and unwrap the outermost rule, never fails
    println!("{:#?}",tree)
}