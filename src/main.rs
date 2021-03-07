#![allow(dead_code)]

mod types;
mod properties;
mod operations;
mod string_manip;

// Nonstandard symbols used: ∀∃∧∨⊃·

fn main() {
    let a = types::Variable::new("a");
    let one = types::Term::new("S0");
    let complex_term = types::Term::new("S(a+Sb)");
    let atom = operations::eq(&one,&one);
    let long_formula = types::Formula::new("<~∃b:~a=b∧∀c:~a=c>");
    let sum_of_terms = operations::add(&one,&one);
    //let term_plus_atom = operations::add(&term,&atom); <- should refuse to compile
    let all_a_is_a = types::Formula::new("∀a:a=a");
    let specification = operations::specify(&all_a_is_a,&a,&one);

    println!("{}",a);
    println!("{}",one);
    println!("{}",complex_term);
    println!("{}",atom);
    println!("{}",long_formula);
    println!("{}",sum_of_terms);
    println!("{}",all_a_is_a);
    println!("{}",specification);
}
