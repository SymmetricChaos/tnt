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

        // Numbers are variables are terminal symbols that we immediately capture
        Rule::num => TntNode::Number(pair.as_str().to_string()),
        Rule::var => TntNode::Variable(pair.as_str().to_string()),

        Rule::equality => {
            let mut t = pair.into_inner();
            let lhs = t.next().unwrap();
            let rhs = t.next().unwrap();
            TntNode::Equality(Box::new(build_ast(lhs)),Box::new(build_ast(rhs)))
        },
        Rule::logical_formula => todo!(),
        Rule::quantification => todo!(),

        Rule::arithmetic_expr => todo!(),
        Rule::successor_expr => TntNode::Successor( Box::new( build_ast(pair.into_inner().next().unwrap())) ),
        _ => unreachable!()
    }
}

#[test]
fn simple_test() {
    let tnt = "S0=a''";
    let tree = TntParser::parse(Rule::equality, tnt)
    .expect("unsuccessful parse") // unwrap the parse result, might fail
    .next().unwrap(); // get and unwrap the outermost rule, never fails
    println!("{:#?}",tree);
    let ast = build_ast(tree);
    println!("{}",ast);
}