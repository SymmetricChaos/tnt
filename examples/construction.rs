use tnt::types::TNT;

fn main() {

    let decision_formula = [
        TNT::new("Ea:(a*SS0)=b"),
        TNT::new("Aa:[Ec:(a*c)=b>Ed:(d*SS0)=a]"),];

    let decision_name = [
        "b is divisible by two",
        "b is a power of two",];

    println!("Some valid statements of tnt:");

    for (formula,name) in decision_formula.iter().zip(&decision_name) {
        println!("\n{}\n{}\n{}",name,formula,formula.english());
    }
    
}