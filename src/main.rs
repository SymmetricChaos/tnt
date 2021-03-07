mod Types;
mod Properties;
mod Operations;
mod StringManipulation;

// Nonstandard symbols used: ∀∃∧∨⊃·

fn main() {
    let variable = Types::Variable::new("a");
    let term = Types::Term::new("S0");
    let complex_term = Types::Term::new("S(a+Sb)");
    let atom = Types::Formula::new("a=a");
    let long_formula = Types::Formula::new("<~∃b:~a=b∧∀c:~a=c>");
    let sum_of_terms = Operations::add(&term,&term);
    //let term_plus_atom = Operations::add(&term,&atom); <- should refuse to compile
    let all_a_is_a = Operations::forall(&variable,&atom);

    println!("{}",variable);
    println!("{}",term);
    println!("{}",complex_term);
    println!("{}",atom);
    println!("{}",long_formula);
    println!("{}",sum_of_terms);
    println!("{}",all_a_is_a);
}
