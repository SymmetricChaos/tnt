mod types;
mod properties;
mod operations;
mod string_manip;

// Nonstandard symbols used: ∀∃∧∨⊃·

fn main() {
    let variable = types::Variable::new("a");
    let term = types::Term::new("S0");
    let complex_term = types::Term::new("S(a+Sb)");
    let atom = operations::eq(&term,&term);
    let long_formula = types::Formula::new("<~∃b:~a=b∧∀c:~a=c>");
    let sum_of_terms = operations::add(&term,&term);
    //let term_plus_atom = operations::add(&term,&atom); <- should refuse to compile
    let all_a_is_a = operations::forall(&variable,&atom);

    println!("{}",variable);
    println!("{}",term);
    println!("{}",complex_term);
    println!("{}",atom);
    println!("{}",long_formula);
    println!("{}",sum_of_terms);
    println!("{}",all_a_is_a);
}
