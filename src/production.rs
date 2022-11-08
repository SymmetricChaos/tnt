//! Create inferences from other statements of TNT, will return LogicError if constraints are not met.

use std::convert::TryFrom;

use indexmap::IndexSet;

use crate::logic_errors::LogicError;
use crate::{succ, Formula, Term, ZERO};

/// In a given Formula with some Variable universally quantified remove the quantification and change the Variable to some Term
/// ```
/// use tnt::terms::Term;
/// use tnt::formula::Formula;
/// use tnt::operations::production::specification;
/// let a = &Term::var("a");
/// let n = &Term::try_from("SS0").unwrap();
/// let f = &Formula::try_from("Ea':Aa:[a=a&a'=a']").unwrap();
/// specification(f,a,n); // Ea':[SS0=SS0&a'=a']
/// ```
pub fn specification(
    formula: &Formula,
    var_name: &'static str,
    term: &Term,
) -> Result<Formula, LogicError> {
    if formula.contains_var_bound_universal(&var_name) {
        let vars_in_term = {
            let mut m = IndexSet::new();
            term.get_vars(&mut m);
            m
        };
        let bound_in_formula = {
            let mut m = IndexSet::new();
            formula.get_vars_bound(&mut m);
            m
        };
        for var in vars_in_term {
            if bound_in_formula.contains(&var) && term.to_string() != var_name.to_string() {
                return Err(LogicError(format!("Specification Error: The Term `{}` contains a Term::Variable with the name `{}` which is already bound in the Formula `{}`",term,var_name,formula)));
            }
        }
        let mut out = formula.clone();
        out.specify(&var_name, term);
        Ok(out)
    } else {
        Err(LogicError(format!("Specification Error: There is no Term::Variable with the name `{}` univerally quantified in the Formula `{}`",var_name,formula)))
    }
}

/// In a given Formula with some Variable not quantified, universally quantify that variable. This is additionally restricted within the Deduction struct.
/// ```
/// use tnt::{Term,Formula,generalization};
/// let a = "a";
/// let f = &Formula::try_from("Ea':[a=a&a'=a']");
/// generalization(f,a); // Aa:Ea':[a=a&a'=a']
/// ```
pub fn generalization(formula: &Formula, var_name: &'static str) -> Result<Formula, LogicError> {
    if !formula.contains_var_bound(&var_name) {
        Ok(Formula::forall(var_name, formula))
    } else {
        Err(LogicError::new(format!(
            "Generalization Error: The there is a Term::Variable with the name `{}` is already bound in the Formula `{}`",
            var_name, formula
        )))
    }
}

/// In a given Formula with some Variable not quantified, existentially quantify that variable.
/// ```
/// use tnt::terms::{Variable,Number,Term};
/// use tnt::formula::Formula;
/// use tnt::operations::production::existence;
/// let a = "a");
/// let f = &Formula::try_from("Ea':[a=a&a'=a']");
/// existence(f,a); // Ea':Ea:[a=a&a'=a']
/// ```
pub fn existence(formula: &Formula, var_name: &'static str) -> Result<Formula, LogicError> {
    if !formula.contains_var_bound(&var_name) {
        Ok(Formula::exists(var_name, formula))
    } else {
        Err(LogicError::new(format!(
            "Existence Error: The there is a Term::Variable with the name `{}` is already bound in the Formula `{}`",
            var_name, formula
        )))
    }
}

/// In a given Formula change the nth occurrence of the quantification ~E<var_name>: to A<var_name>:~
/// ```
/// use tnt::{Term,Fomula};
/// use tnt::production::interchange_ea;
/// let b = "b";
/// let f = &Formula::try_from("~Eb:[a=b|Sa=b]").unwrap();
/// interchange_ea(f,b,0); // Ab:~[a=b&Sa=b]
/// ```
pub fn interchange_ea(
    formula: &Formula,
    var_name: &str,
    nth: usize,
) -> Result<Formula, LogicError> {
    let e = format!("~E{}:", var_name);
    let a = format!("A{}:~", var_name);
    let mut new_string = formula.to_string();
    let old_string = formula.to_string();
    let quantifications = old_string.match_indices(&e);
    let count = quantifications.clone().count();
    if count == 0 {
        return Err(LogicError(format!(
            "Interchange Error: The quantification `{}` does not exist in the Formula `{}`",
            e, formula
        )));
    }
    if count < nth {
        return Err(LogicError(format!(
            "Interchange Error: The quantification `{}` only appears {} times in the Formula `{}`",
            e, count, formula
        )));
    }
    for (pos, q) in quantifications.enumerate() {
        if pos == nth {
            new_string.replace_range(q.0..q.0 + q.1.len(), &a);
            break;
        }
    }
    Ok(Formula::try_from(new_string).unwrap())
}

/// In a given Formula change the nth occurrence of the quantification A<var_name>:~ to ~E<var_name>:
/// ```
/// use tnt::{Term,Fomula};
/// use tnt::production::interchange_ae;
/// let b = "b";
/// let f = &Formula::try_from("Ab:~[a=b|Sa=b]");
/// interchange_ae(f,b,0); // ~Eb:[a=b|Sa=b]
/// ```
pub fn interchange_ae(
    formula: &Formula,
    var_name: &str,
    nth: usize,
) -> Result<Formula, LogicError> {
    let e = format!("~E{}:", var_name);
    let a = format!("A{}:~", var_name);
    let mut new_string = formula.to_string();
    let old_string = formula.to_string();
    let quantifications = old_string.match_indices(&a);
    let count = quantifications.clone().count();
    if count == 0 {
        return Err(LogicError(format!(
            "Interchange Error: The quantification `{}` does not exist in the Formula `{}`",
            a, formula
        )));
    }
    if count < nth {
        return Err(LogicError(format!(
            "Interchange Error: The quantification `{}` only appears {} times in the Formula `{}`",
            a, count, formula
        )));
    }
    for (pos, q) in quantifications.enumerate() {
        if pos == nth {
            new_string.replace_range(q.0..q.0 + q.1.len(), &e);
            break;
        }
    }
    Ok(Formula::try_from(new_string).unwrap())
}

/// Perform induction.
/// ```

/// ```
pub fn induction(var_name: &str, base: &Formula, general: &Formula) -> Result<Formula, LogicError> {
    // If the variable name requested doesn't exist in the general case then we can stop immediately.
    if !general.contains_var(&var_name) {
        return Err(LogicError(format!(
            "Induction Error: The Term::Variable `{var_name}` does not appear in the general case `{general}`"
        )));
    }

    // Likewise if the variable name requested DOES exist in the base case then we can stop immediately.
    if base.contains_var(&var_name) {
        return Err(LogicError(format!(
            "Induction Error: The Term::Variable `{var_name}` appears in the base case `{base}`"
        )));
    }

    // Now we must extract the left side of the implication from the general case and provide an
    // error if this is not possible.
    let left_implication = if let Formula::Universal(_, inner) = general {
        if let Formula::Implies(left, _right) = &**inner {
            *left.clone()
        } else {
            return Err(LogicError(format!(
                "Induction Error: The general case `{general}` does not contain an implication"
            )));
        }
    } else {
        return Err(LogicError(format!(
            "Induction Error: The general case `{general}` is not a universal quantification"
        )));
    };

    // If the variable name is being used in a quantification of the left side of the implication we must stop
    if left_implication.contains_var_bound(&var_name) {
        return Err(LogicError(format!(
            "Induction Error: The Term::Variable `{var_name}` is already bound in the Formula `{left_implication}`, which is the left side of the general case `{general}`" 
        )));
    }

    // The left side of the implication when the variable is replaced with Zero should match the base case.
    let mut formula_zero = left_implication.clone();
    formula_zero.replace_free(&var_name, &ZERO);
    if &formula_zero != base {
        return Err(LogicError(format!(
            "Induction Error: The base case `{base}` is not of the same form as `{left_implication}`, which is the left side of the general case `{general}`" 
        )));
    }

    // The implication of the general case must be that the left side implies that the variable can be replaced with its successor everywhere and still be true
    let successor_of_var = succ(&Term::var(var_name));
    let mut formula_succ = left_implication.clone();
    formula_succ.replace_free(&var_name, &successor_of_var);
    let correct_general = Formula::forall(
        var_name,
        &Formula::implies(&left_implication, &formula_succ),
    );
    if &correct_general != general {
        return Err(LogicError(format!(
            "Induction Error: The general case should be `{correct_general}` but the general case provided is actually `{general}`" 
        )));
    }

    Ok(Formula::forall(var_name, &left_implication))
}

// pub fn induction(v: &Variable, base: &Formula, general: &Formula) -> Result<Formula, LogicError> {
//     // The theorem we need to generalize is the outermost, leftmost implication of the general case
//     let theorem = Formula::try_from(left_implies(&general.to_string()).unwrap());
//     if get_bound_vars(&theorem.to_string()).contains(&v.to_string()) {
//         let msg = format!("Induction Error: The Variable `{}` is already bound in the Formula `{}`, which is the left side of the general case `{}`",v,theorem,general);
//         return Err(LogicError::new(msg));
//     } else {
//         let vs = &v.succ();
//         let xs = theorem.replace_var(v, vs);
//         let x0 = theorem.replace_var(v, &Number::zero());
//         if x0.to_string() != base.to_string() {
//             let msg = format!(
//                 "Induction Error: The base case must be the Formula `{}`",
//                 x0
//             );
//             return Err(LogicError::new(msg));
//         }
//         if general.to_string() != format!("A{}:[{}>{}]", v, theorem, xs) {
//             let msg = format!(
//                 "Induction Error: The general case must be the Formula `A{}:[{}>{}]`",
//                 v, theorem, xs
//             );
//             return Err(LogicError::new(msg));
//         }
//         return Ok(forall(v, &theorem));
//     }
// }

/// Given a Formula::Equality return the successor of both sides
/// ```
/// use tnt::{Formula,successor};
/// let f = &Formula::try_from("a=b").unwrap();
/// successor(f); // Sa=Sb
/// ```
pub fn successor(formula: &Formula) -> Result<Formula, LogicError> {
    if let Formula::Equality(l, r) = formula {
        Ok(Formula::eq(&succ(&l), &succ(&r)))
    } else {
        Err(LogicError(format!(
            "Successor Error: {} is not a Formula::Equality",
            formula
        )))
    }
}

/// Given a Formula::Equality return the predecessor of both sides
/// ```
/// use tnt::{Formula,predecessor};
/// let f = &Formula::try_from("Sa=Sb");
/// predecessor(f); // a=b
/// ```
pub fn predecessor(formula: &Formula) -> Result<Formula, LogicError> {
    if let Formula::Equality(l, r) = formula {
        match (l, r) {
            (Term::Successor(pl), Term::Successor(pr)) => Ok(Formula::eq(&pl, &pr)),
            _ => Err(LogicError(format!(
                "Predecessor Error: {} does not have Term::Succ on both sides",
                formula
            ))),
        }
    } else {
        Err(LogicError(format!(
            "Predecessor Error: {} is not a Formula::Equality",
            formula
        )))
    }
}

/// Given a Formula::Equality flip the two sides of the equality
/// ```
/// use tnt::{Formula,symmetry};
/// let f = &Formula::try_from("SSa=Sb'");
/// symmetry(f); // Sb'=SSa
/// ```
pub fn symmetry(formula: &Formula) -> Result<Formula, LogicError> {
    if let Formula::Equality(l, r) = formula {
        Ok(Formula::eq(&r, &l))
    } else {
        Err(LogicError(format!(
            "Symmetry Error: {} is not a Formula::Equality",
            formula
        )))
    }
}

/// Given two Formula::Equality where the right side of the first matches the left side of the second return the Formula that is the equality of their left and right
/// ```
/// use tnt::{Formula,transitivity};
/// let f1 = &Formula::try_from("SSa=Sb'");
/// let f2 = &Formula::try_from("Sb'=(1+1)");
/// transitivity(f1,f2); // SSa=(1+1)
/// ```
pub fn transitivity(
    left_formula: &Formula,
    right_formula: &Formula,
) -> Result<Formula, LogicError> {
    match (left_formula, right_formula) {
        (Formula::Equality(left_l, left_r), Formula::Equality(right_l, right_r)) => {
            if left_r == right_l {
                return Ok(Formula::eq(left_l, right_r));
            } else {
                return Err(LogicError(format!(
                    "Transitivity Error: the terms `{}` and `{}` do not match",
                    left_r, right_l
                )));
            }
        }
        _ => {
            return Err(LogicError(format!(
                "Transitivity Error: the formulas `{}` and `{}` are not both Formula::Equality",
                left_formula, right_formula
            )))
        }
    }
}

#[cfg(test)]
mod test {

    use crate::ONE;

    use super::*;

    #[test]
    fn test_specification() -> Result<(), LogicError> {
        let a = "a";
        let formula1 = &Formula::try_from("Aa:a=a").unwrap();
        let formula2 = &Formula::try_from("Ea':Aa:[a=a&a'=a']").unwrap();
        assert_eq!(specification(formula1, a, &ONE)?.to_string(), "S0=S0");
        assert_eq!(
            specification(formula2, a, &ONE)?.to_string(),
            "Ea':[S0=S0&a'=a']"
        );
        Ok(())
    }

    #[test]
    fn test_specification_2() -> Result<(), LogicError> {
        let a = "a";
        let formula1 = &Formula::try_from("Aa:Ab:(a+Sb)=S(a+b)").unwrap();
        assert_eq!(
            specification(formula1, a, &Term::var("d"))?.to_string(),
            "Ab:(d+Sb)=S(d+b)"
        );
        Ok(())
    }

    #[test]
    fn test_specification_err1() {
        let a = "b";
        let formula1 = &Formula::try_from("Aa:a=a").unwrap();
        assert!(specification(formula1, a, &ONE).is_err());
    }

    // #[test]
    // fn test_specification_err2() {
    //     let a = Term::var("a");
    //     let formula1 = &Formula::try_from("Aa:a=a").unwrap();
    //     assert!(specification(formula1, "a", &(&a + &ONE)).is_err());
    // }

    #[test]
    fn test_generalization() -> Result<(), LogicError> {
        let a = "a";
        let x = "x'";
        let formula1 = &Formula::try_from("a=a").unwrap();
        let formula2 = &Formula::try_from("Ea':Aa:[a=a&x'=a']").unwrap();
        assert_eq!(generalization(formula1, a)?.to_string(), "Aa:a=a");
        assert_eq!(
            generalization(formula2, x)?.to_string(),
            "Ax':Ea':Aa:[a=a&x'=a']"
        );
        Ok(())
    }

    #[test]
    fn test_generalization_err() {
        let c = "c";
        let formula1 = &Formula::try_from("Ec:a=c").unwrap();
        assert!(generalization(formula1, c).is_err());
    }

    #[test]
    fn test_symmetry() -> Result<(), LogicError> {
        let simple1 = &Formula::try_from("a=b").unwrap();
        let simple2 = &Formula::try_from("b=S(a+S0)").unwrap();
        assert_eq!(symmetry(simple1)?.to_string(), "b=a");
        assert_eq!(symmetry(simple2)?.to_string(), "S(a+S0)=b");
        Ok(())
    }

    #[test]
    fn test_symmetry_err() {
        let complex = &Formula::try_from("Aa:a=b").unwrap();
        assert!(symmetry(complex).is_err());
    }

    #[test]
    fn test_transitivity() -> Result<(), LogicError> {
        let atom1 = Formula::try_from("a=b").unwrap();
        let atom2 = Formula::try_from("b=S(a+S0)").unwrap();
        assert_eq!(transitivity(&atom1, &atom2)?.to_string(), "a=S(a+S0)");
        Ok(())
    }

    #[test]
    fn test_transitivity_err_1_left() {
        let complex1 = &Formula::try_from("Aa:a=b").unwrap();
        let complex2 = &Formula::try_from("p=j''").unwrap();
        assert!(transitivity(complex1, complex2).is_err());
    }

    #[test]
    fn test_transitivity_err_1_right() {
        let complex1 = &Formula::try_from("Aa:a=b").unwrap();
        let complex2 = &Formula::try_from("p=j''").unwrap();
        assert!(transitivity(complex2, complex1).is_err());
    }

    #[test]
    fn test_transitivity_err_2() {
        let complex1 = &Formula::try_from("p=j''").unwrap();
        let complex2 = &Formula::try_from("q'=p").unwrap();
        assert!(transitivity(complex1, complex2).is_err());
    }

    #[test]
    fn test_predecessor() -> Result<(), LogicError> {
        let simple = &Formula::try_from("Sm''=SSu").unwrap();
        assert_eq!(predecessor(simple)?.to_string(), "m''=Su");
        Ok(())
    }

    #[test]
    fn test_predecessor_err_1() {
        let complex = &Formula::try_from("~Ei:(i+SS0)=g").unwrap();
        assert!(predecessor(complex).is_err());
    }

    #[test]
    fn test_predecessor_err_2() {
        let simple = &Formula::try_from("SSb'=0").unwrap();
        assert!(predecessor(simple).is_err());
    }

    #[test]
    fn test_successor() -> Result<(), LogicError> {
        let simple = &Formula::try_from("Sm''=SSu").unwrap();
        assert_eq!(successor(simple)?.to_string(), "SSm''=SSSu");
        Ok(())
    }

    #[test]
    fn test_successor_err() {
        let complex = &Formula::try_from("~Ei:(i+SS0)=g").unwrap();
        assert!(successor(complex).is_err());
    }

    #[test]
    fn test_interchange_ea() -> Result<(), LogicError> {
        let formula1 = &Formula::try_from("Aa:~Eu':(a+u')=Sa").unwrap();
        let formula2 = &Formula::try_from("[Aa:~Eu':(a+u')=Sa&~Eu':u'=SS0]").unwrap();
        let variable = "u'";
        assert_eq!(
            interchange_ea(formula1, variable, 0)?.to_string(),
            "Aa:Au':~(a+u')=Sa"
        );
        assert_eq!(
            interchange_ea(formula2, variable, 1)?.to_string(),
            "[Aa:~Eu':(a+u')=Sa&Au':~u'=SS0]"
        );
        Ok(())
    }

    #[test]
    fn test_interchange_ea_err() {
        let formula1 = &Formula::try_from("Aa:~Eu':(a+u')=Sa").unwrap();
        let variable = "z";
        assert!(interchange_ea(formula1, variable, 0).is_err());
    }

    #[test]
    fn test_interchange_ae() -> Result<(), LogicError> {
        let formula1 = &Formula::try_from("Aa:Au':~(a+u')=Sa").unwrap();
        let formula2 = &Formula::try_from("[Aa:~Eu':(a+u')=Sa&Au':~u'=SS0]").unwrap();
        let variable = "u'";
        assert_eq!(
            interchange_ae(formula1, variable, 0)?.to_string(),
            "Aa:~Eu':(a+u')=Sa"
        );
        assert_eq!(
            interchange_ae(formula2, variable, 0)?.to_string(),
            "[Aa:~Eu':(a+u')=Sa&~Eu':u'=SS0]"
        );
        Ok(())
    }

    #[test]
    fn test_interchange_ae_err() {
        let formula1 = &Formula::try_from("Aa:~Eu':(a+u')=Sa").unwrap();
        let variable = "z";
        assert!(interchange_ae(formula1, variable, 0).is_err());
    }

    #[test]
    fn test_induction() -> Result<(), LogicError> {
        let v = "v";
        let base = &Formula::try_from("0=0").unwrap();
        let gen = &Formula::try_from("Av:[v=v>Sv=Sv]").unwrap();
        assert_eq!(induction(v, base, gen)?.to_string(), "Av:v=v");
        Ok(())
    }
}
