use pest::{Parser, error::Error, iterators::Pairs};
use pest_derive::*;


#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct TNTParser;

pub fn parse_formula(s: &str) -> Result<Pairs<Rule>,Error<Rule>> {
    let p = TNTParser::parse(Rule::formula, s)?;
    for pair in p.clone() {
        println!("{}",pair);
    }
    Ok(p)
}

#[test]
fn test_parser() {
    parse_formula("(S0+Sb)=S(S0+b)");
}