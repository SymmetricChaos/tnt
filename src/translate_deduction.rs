use crate::deduction::Deduction;

pub fn translate_deduction(proof: &Deduction) {
    let names = ["axiom","specification","generalization",
                 "existence","successor","predecessor",
                 "interchange_ea","interchange_ae","symmetry",
                 "transitivity","supposition","implication",
                 "induction"];
    
    for p in proof.all_theorems_raw() {
        let start = match p.rule_num {
            0 => "take as an axiom",
            _ => ""
        };

        println!("{}: {}",p.rule,p.formula.english())
    }
}