use crate::deduction::Deduction;

pub fn translate_deduction(proof: &Deduction) {
    let names = ["axiom","specification","generalization",
                 "existence","successor","predecessor",
                 "interchange_ea","interchange_ae","symmetry",
                 "transitivity","supposition","implication",
                 "induction"];
    
    for p in proof.all_theorems_raw() {
        let start = match p.rule_num {
            0 => "We take as an axiom that",
            _ => &p.rule,
        };
        println!("{}) {} {}",p.position,start,p.formula.english());
    }
}