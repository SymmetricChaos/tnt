use pest::{Parser, iterators::Pair};

use super::formula_tree::{Variable,Formula,Term};


#[derive(Parser)]
#[grammar = "parsing\\tnt.pest"]
pub struct TntParser;


pub fn print_parse_tree(text: &str, rule: Rule) -> Result<(),pest::error::Error<Rule>> {
    let tree = TntParser::parse(rule, text)?;
    println!("{:#?}",tree);
    Ok(())
}

pub fn formula_to_ast(text: &str) -> Result<Formula,pest::error::Error<Rule>> {
    let mut tree = TntParser::parse(Rule::formula, text)?;
    Ok(build_formula_tree(tree.next().unwrap()))
}

pub fn term_to_ast(text: &str) -> Result<Term,pest::error::Error<Rule>> {
    let mut tree = TntParser::parse(Rule::term, text)?;
    Ok(build_term_tree(tree.next().unwrap()))
}



pub fn build_term_tree(pair: Pair<Rule>) -> Term {
    match pair.as_rule() {
        Rule::zero => Term::Zero,
        Rule::variable => {
            Term::Variable(Variable::new(pair.as_str()))
        },
        Rule::multiplication => {
            let mut t = pair.into_inner();
            let lhs = t.next().unwrap();
            let rhs = t.next().unwrap();
            Term::Mul(Box::new(build_term_tree(lhs)),Box::new(build_term_tree(rhs)))
        },
        Rule::addition => {
            let mut t = pair.into_inner();
            let lhs = t.next().unwrap();
            let rhs = t.next().unwrap();
            Term::Add(Box::new(build_term_tree(lhs)),Box::new(build_term_tree(rhs)))
        },
        Rule::successor => {
            let mut t = pair.into_inner();
            let term = t.next().unwrap();
            Term::Successor(Box::new(build_term_tree(term)))
        },
        _ => unreachable!("input to build_term_tree was not a term")
    }
}

pub fn build_formula_tree(pair: Pair<Rule>) -> Formula {
    match pair.as_rule() {
        Rule::existential => {
            let mut t = pair.into_inner();
            let v = t.next().unwrap();
            let form = t.next().unwrap();
            Formula::Exists(Variable::new(v.as_str()),Box::new(build_formula_tree(form)))
        },
        Rule::universal => {
            let mut t = pair.into_inner();
            let v = t.next().unwrap();
            let form = t.next().unwrap();
            Formula::ForAll(Variable::new(v.as_str()),Box::new(build_formula_tree(form)))
        },
        Rule::equality => {
            let mut t = pair.into_inner();
            let lhs = t.next().unwrap();
            let rhs = t.next().unwrap();
            Formula::Equality(build_term_tree(lhs),build_term_tree(rhs))
        },
        Rule::and => {
            let mut t = pair.into_inner();
            let lhs = t.next().unwrap();
            let rhs = t.next().unwrap();
            Formula::And(Box::new(build_formula_tree(lhs)),Box::new(build_formula_tree(rhs)))
        },
        Rule::or => {
            let mut t = pair.into_inner();
            let lhs = t.next().unwrap();
            let rhs = t.next().unwrap();
            Formula::Or(Box::new(build_formula_tree(lhs)),Box::new(build_formula_tree(rhs)))
        },
        Rule::implies => {
            let mut t = pair.into_inner();
            let lhs = t.next().unwrap();
            let rhs = t.next().unwrap();
            Formula::Implies(Box::new(build_formula_tree(lhs)),Box::new(build_formula_tree(rhs)))
        },
        Rule::negation => {
            let mut t = pair.into_inner();
            let form = t.next().unwrap();
            Formula::Negation(Box::new(build_formula_tree(form)))
        },
        _ => unreachable!("input to build_formula_tree was not a formula")
    }
}





#[cfg(test)]
mod test_expressions {

    use super::*;

    #[test]
    fn test_addition() {
        let tnt = "(0+Sa)";
        let ast = term_to_ast(tnt).unwrap();
        assert_eq!(tnt,format!("{}",&ast));
    }

    #[test]
    fn test_multiplication() {
        let tnt = "(x''*SSSSSS0)";
        let ast = term_to_ast(tnt).unwrap();
        assert_eq!(tnt,format!("{}",&ast));
    }

    #[test]
    fn test_complex_arithmetic() {
        let tnt = "SS((b+S0)*Sa'')";
        let ast = term_to_ast(tnt).unwrap();
        assert_eq!(tnt,format!("{}",&ast));
    }
}


#[cfg(test)]
mod test_formulas {

    use super::*;

    #[test]
    fn test_simple_equality() {
        let tnt = "S0=a''";
        let ast = formula_to_ast(tnt).unwrap();
        assert_eq!(tnt,format!("{}",&ast));
    }

    #[test]
    fn test_compound_equality() {
        let tnt = "S0=(b+b)";
        let ast = formula_to_ast(tnt).unwrap();
        assert_eq!(tnt,format!("{}",&ast));
    }

    #[test]
    fn test_quantification() {
        let tnt = "~~Ea':z=a";
        let ast = formula_to_ast(tnt).unwrap();
        assert_eq!(tnt,format!("{}",&ast));
    }

    #[test]
    fn test_complex_formula() {
        let tnt = "Aa:Ab:(a*Sb)=((a*b)+a)";
        let ast = formula_to_ast(tnt).unwrap();
        assert_eq!(tnt,format!("{}",&ast));
    }

    #[test]
    fn test_very_complex_formula() {
        let tnt = "Aa:[Ec:(a*c)=b>Ed:(d*SS0)=a]";
        let ast = formula_to_ast(tnt).unwrap();
        assert_eq!(tnt,format!("{}",&ast));
    }
}


