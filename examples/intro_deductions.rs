use tnt::terms::{Expression, Number, Variable, Term};
use tnt::formula::Formula;
use tnt::deduction::Deduction;
use tnt::axioms::PEANO;
use tnt::logic_errors::LogicError;

fn main() -> Result<(),LogicError> {
    println!("The full logic of TNT is expressed by the Deduction struct which allows proofs to be created by chaining arguments together.");
    
    println!("We need a few things to set up the Deductuion. First we need a set of axioms, statements of TNT taken to be true without proof. Currently only the PEANO axioms are provided.");
    
    println!("\nuse tnt::axioms::PEANO;");
    println!("use tnt::deduction::Deduction;");

    let mut e = Deduction::new("Prove that 1 is the Left Multiplicative Identity", PEANO.clone());
    println!("let mut e = Deduction::new(\"Prove that 1 is the Left Multiplicative Identity\", PEANO.clone());");

    println!("\nWe will also need some Variables and Numbers to use. Notice that we only want to use references to these since they do not implement Copy and we need to reuse them.");
    let a = &Variable::new("a");
    let one = &Number::one();

    println!("
let a = &Variable::new(\"a\");
let one = &Number::one();");

    println!("\nWe begin by using the .add_axiom() method to state one of the PEANO axioms. For this proof we'll use the 3rd axiom which states: Aa:(a*0)=0. Then we immediately use the .sepecification() method to create a new statement saying: (S0*0)=0.");

    e.add_axiom(&PEANO[3])?;
    println!("\ne.add_axiom(PEANO[3])?;");

    e.specification(0, a, one)?;
    println!("e.specification(0, a, one)?;");

    println!("\nNotice that we use the ? operator after each method. All of the inferrential methods of Deduction return a Result<(),LogicError> that will give us a nice explanation if any rule is broken.");

    Ok(())
}