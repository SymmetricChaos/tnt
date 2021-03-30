use pest::{Parser, error::Error, iterators::Pairs};
use pest_derive::*;


#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct TNTParser;

pub fn extract_tokens(pairs: Pairs<Rule>) {
    
    for pair in pairs.flatten() {
        
        println!("{:?}, {:?}",pair.as_str(),pair.as_rule());
        let x = pair.into_inner();
        if x.as_str() == "" {
            extract_tokens(x);
        }
    }
}


#[test]
fn test_parser() -> Result<(),Error<Rule>> {

    let f = "S0=a";
    let p = TNTParser::parse(Rule::formula, f)?;
    extract_tokens(p);

    println!("");

    let f = "[Aa:[(S0*SS0)=a|Sa=S0]>Eb:(Sb+0)=((a+b)*SSS0)]";
    let p = TNTParser::parse(Rule::formula, f)?;
    extract_tokens(p);
    
    Ok(())
}