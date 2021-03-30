use pest::{Parser, error::Error, iterators::Pairs};
use pest_derive::*;


#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct TNTParser;

pub fn extract_tokens(pairs: Pairs<Rule>) {
    for pair in pairs.flatten() {
        println!("{}",pair.as_str());
        extract_tokens(pair.into_inner());
    }
}


#[test]
fn test_parser() -> Result<(),Error<Rule>> {
    let formula = "[~Ao':o'*SS0=0>Eb:Ec:(0*S(b+SSc'))=S0]";
    let p = TNTParser::parse(Rule::formula, formula)?;
    extract_tokens(p);
    Ok(())
}