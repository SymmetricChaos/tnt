//! Create inferences from other statements of TNT, will return LogicError if constraints are not met.

use std::collections::HashSet;

use crate::logic_errors::LogicError;
use crate::{eq, exists, forall, succ, Formula, Term};

/// In a given Formula with some Variable universally quantified remove the quantification and change the Variable to some Term
/// ```
/// use tnt::terms::Term;
/// use tnt::formula::Formula;
/// use tnt::operations::production::specification;
/// let a = &Variable::new("a");
/// let n = &Number::new("SS0");
/// let f = &Formula::new("Ea':Aa:[a=a&a'=a']");
/// specification(f,a,n); // Ea':[SS0=SS0&a'=a']
/// ```
pub fn specification(
    formula: &Formula,
    var_name: &'static str,
    term: &Term,
) -> Result<Formula, LogicError> {
    if formula.contains_var_bound_universal(var_name) {
        let vars_in_term = {
            let mut m = HashSet::new();
            term.get_vars(&mut m);
            m
        };
        let bound_in_formula = {
            let mut m = HashSet::new();
            formula.get_vars_bound(&mut m);
            m
        };
        for var in vars_in_term {
            if bound_in_formula.contains(&var) && term.to_string() != var_name.to_string() {
                return Err(LogicError(format!("Specification Error: The Term `{}` contains a Term::Variable with the name `{}` which is already bound in the Formula `{}`",term,var_name,formula)));
            }
        }
        let mut out = formula.clone();
        out.specify(var_name, term);
        Ok(out)
    } else {
        Err(LogicError(format!("Specification Error: There is no Term::Variable with the name `{}` univerally quantified in the Formula `{}`",var_name,formula)))
    }
}

/// In a given Formula with some Variable not quantified, universally quantify that variable. This is additionally restricted within the Deduction struct.
/// ```
/// use tnt::terms::{Variable,Number,Term};
/// use tnt::formula::Formula;
/// use tnt::operations::production::generalization;
/// let a = &Variable::new("a");
/// let f = &Formula::new("Ea':[a=a&a'=a']");
/// generalization(f,a); // Ea':Aa:[a=a&a'=a']
/// ```
pub fn generalization(formula: &Formula, var_name: &'static str) -> Result<Formula, LogicError> {
    if !formula.contains_var_bound(var_name) {
        Ok(forall(var_name, formula))
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
/// let f = &Formula::new("Ea':[a=a&a'=a']");
/// existence(f,a); // Ea':Ea:[a=a&a'=a']
/// ```
pub fn existence(formula: &Formula, var_name: &'static str) -> Result<Formula, LogicError> {
    if !formula.contains_var_bound(var_name) {
        Ok(exists(var_name, formula))
    } else {
        Err(LogicError::new(format!(
            "Existence Error: The there is a Term::Variable with the name `{}` is already bound in the Formula `{}`",
            var_name, formula
        )))
    }
}

// /// In a given Formula change the nth occurrence of the quantification ~E<var>: to A<var>:~
// /// ```
// /// use tnt::{Term,Fomula};
// /// use tnt::production::interchange_ea;
// /// let b = "b";
// /// let f = &Formula::try_from("~Eb:[a=b|Sa=b]").unwrap();
// /// interchange_ea(f,b,0); // Ab:~[a=b&Sa=b]
// /// ```
// pub fn interchange_ea(
//     formula: &Formula,
//     var_name: &str,
//     nth: usize,
// ) -> Result<Formula, LogicError> {
//     let e = format!("~E{}:", var_name);
//     let a = format!("A{}:~", var_name);
//     let mut new_s = formula.to_string().clone();
//     let xs = formula.to_string();
//     let qs = xs.match_indices(&e);
//     let count = qs.clone().count();
//     if count == 0 {
//         return Err(LogicError(format!(
//             "Interchange Error: The quantification `{}` does not exist in the Formula `{}`",
//             e, formula
//         )));
//     }
//     if count < nth {
//         Err(LogicError(format!(
//             "Interchange Error: The quantification `{}` only appears {} times in the Formula `{}`",
//             e, count, formula
//         )));
//     }
//     for (pos, q) in qs.enumerate() {
//         if pos == nth {
//             new_s.replace_range(q.0..q.0 + q.1.len(), &a);
//             break;
//         }
//     }
//     Ok(Formula::new_complex(&new_s))
// }

// /// In a given Formula change the nth occurrence of the quantification ~E<var>: to A<var>:~
// /// ```
// /// use tnt::terms::{Variable,Term};
// /// use tnt::formula::Formula;
// /// use tnt::operations::production::interchange_ae;
// /// let b = &Variable::new("b");
// /// let f = &Formula::new("Ab:~[a=b|Sa=b]");
// /// interchange_ae(f,b,0); // ~Eb:[a=b|Sa=b]
// /// ```
// pub fn interchange_ae(x: &Formula, v: &Variable, nth: usize) -> Result<Formula, LogicError> {
//     let e = format!("~E{}:", v);
//     let a = format!("A{}:~", v);
//     let mut new_s = x.to_string().clone();
//     let xs = &x.to_string();
//     let qs = xs.match_indices(&a);
//     let count = qs.clone().count();
//     if count == 0 {
//         let msg = format!(
//             "Interchange Error: The quantification `{}` does not exist in the Formula `{}`",
//             e, x
//         );
//         return Err(LogicError::new(msg));
//     }
//     if count < nth {
//         let msg = format!(
//             "Interchange Error: The quantification `{}` only appears {} times in the Formula `{}`",
//             e, count, x
//         );
//         return Err(LogicError::new(msg));
//     }
//     for (pos, q) in qs.enumerate() {
//         if pos == nth {
//             new_s.replace_range(q.0..q.0 + q.1.len(), &e);
//             break;
//         }
//     }
//     Ok(Formula::new_complex(&new_s))
// }

// /// Perform induction
// /// ```

// /// ```
// pub fn induction(v: &Variable, base: &Formula, general: &Formula) -> Result<Formula, LogicError> {
//     // The theorem we need to generalize is the outermost, leftmost implication of the general case
//     // Need to change this from causing a panic if malformed to causing a LogicError
//     let theorem = Formula::new(left_implies(&general.to_string()).unwrap());
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
/// let f = &Formula::new("a=b");
/// successor(f); // Sa=Sb
/// ```
pub fn successor(formula: &Formula) -> Result<Formula, LogicError> {
    if let Formula::Equality(l, r) = formula {
        Ok(eq(&succ(&r), &succ(&l)))
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
/// let f = &Formula::new("Sa=Sb");
/// predecessor(f); // a=b
/// ```
pub fn predecessor(formula: &Formula) -> Result<Formula, LogicError> {
    if let Formula::Equality(l, r) = formula {
        match (l, r) {
            (Term::Successor(pl), Term::Successor(pr)) => Ok(eq(&pl, &pr)),
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
/// let f = &Formula::new("SSa=Sb'");
/// symmetry(f); // Sb'=SSa
/// ```
pub fn symmetry(formula: &Formula) -> Result<Formula, LogicError> {
    if let Formula::Equality(l, r) = formula {
        Ok(eq(&r, &l))
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
/// let f1 = &Formula::new("SSa=Sb'");
/// let f2 = &Formula::new("Sb'=(1+1)");
/// transitivity(f1,f2); // SSa=(1+1)
/// ```
pub fn transitivity(
    left_formula: &Formula,
    right_formula: &Formula,
) -> Result<Formula, LogicError> {
    match (left_formula, right_formula) {
        (Formula::Equality(left_l, left_r), Formula::Equality(right_l, right_r)) => {
            if left_r == right_l {
                return Ok(eq(left_l, right_r));
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

// #[cfg(test)]
// mod test {

//     use crate::ONE;

//     use super::*;

//     #[test]
//     fn test_specification() -> Result<(), LogicError> {
//         let a = "a";
//         let one = ONE;
//         let formula1 = &Formula::new("Aa:a=a");
//         let formula2 = &Formula::new("Ea':Aa:[a=a&a'=a']");
//         assert_eq!(specification(formula1, a, one)?.to_string(), "S0=S0");
//         assert_eq!(
//             specification(formula2, a, one)?.to_string(),
//             "Ea':[S0=S0&a'=a']"
//         );
//         Ok(())
//     }

//     #[test]
//     fn test_specification_err1() {
//         let a = &Variable::new("b");
//         let one = &Number::new("S0");
//         let formula1 = &Formula::new("Aa:a=a");
//         assert!(specification(formula1, a, one).is_err());
//     }

//     #[test]
//     fn test_specification_err2() {
//         let a = &Variable::new("a");
//         let one = &Number::new("S0");
//         let formula1 = &Formula::new("Aa:a=a");
//         assert!(specification(formula1, a, &(a + one)).is_err());
//     }

//     #[test]
//     fn test_generalization() -> Result<(), LogicError> {
//         let a = &Variable::new("a");
//         let x = &Variable::new("x'");
//         let formula1 = &Formula::new("a=a");
//         let formula2 = &Formula::new("Ea':Aa:[a=a&x'=a']");
//         assert_eq!(generalization(formula1, a)?.to_string(), "Aa:a=a");
//         assert_eq!(
//             generalization(formula2, x)?.to_string(),
//             "Ax':Ea':Aa:[a=a&x'=a']"
//         );
//         Ok(())
//     }

//     #[test]
//     fn test_generalization_err() {
//         let c = &Variable::new("c");
//         let formula1 = &Formula::new("Ec:a=c");
//         assert!(generalization(formula1, c).is_err());
//     }

//     #[test]
//     fn test_symmetry() -> Result<(), LogicError> {
//         let simple1 = &Formula::new("a=b");
//         let simple2 = &Formula::new("b=S(a+S0)");
//         assert_eq!(symmetry(simple1)?.to_string(), "b=a");
//         assert_eq!(symmetry(simple2)?.to_string(), "S(a+S0)=b");
//         Ok(())
//     }

//     #[test]
//     fn test_symmetry_err() {
//         let complex = &Formula::new("Aa:a=b");
//         assert!(symmetry(complex).is_err());
//     }

//     #[test]
//     fn test_transitivity() -> Result<(), LogicError> {
//         let atom1 = Formula::new("a=b");
//         let atom2 = Formula::new("b=S(a+S0)");
//         assert_eq!(transitivity(&atom1, &atom2)?.to_string(), "a=S(a+S0)");
//         Ok(())
//     }

//     #[test]
//     fn test_transitivity_err_1_left() {
//         let complex1 = &Formula::new("Aa:a=b");
//         let complex2 = &Formula::new("p=j''");
//         assert!(transitivity(complex1, complex2).is_err());
//     }

//     #[test]
//     fn test_transitivity_err_1_right() {
//         let complex1 = &Formula::new("Aa:a=b");
//         let complex2 = &Formula::new("p=j''");
//         assert!(transitivity(complex2, complex1).is_err());
//     }

//     #[test]
//     fn test_transitivity_err_2() {
//         let complex1 = &Formula::new("p=j''");
//         let complex2 = &Formula::new("q'=p");
//         assert!(transitivity(complex1, complex2).is_err());
//     }

//     #[test]
//     fn test_predecessor() -> Result<(), LogicError> {
//         let simple = &Formula::new("Sm''=SSu");
//         assert_eq!(predecessor(simple)?.to_string(), "m''=Su");
//         Ok(())
//     }

//     #[test]
//     fn test_predecessor_err_1() {
//         let complex = &Formula::new("~Ei:(i+SS0)=g");
//         assert!(predecessor(complex).is_err());
//     }

//     #[test]
//     fn test_predecessor_err_2() {
//         let simple = &Formula::new("SSb'=0");
//         assert!(predecessor(simple).is_err());
//     }

//     #[test]
//     fn test_successor() -> Result<(), LogicError> {
//         let simple = &Formula::new("Sm''=SSu");
//         assert_eq!(successor(simple)?.to_string(), "SSm''=SSSu");
//         Ok(())
//     }

//     #[test]
//     fn test_successor_err() {
//         let complex = &Formula::new("~Ei:(i+SS0)=g");
//         assert!(successor(complex).is_err());
//     }

//     #[test]
//     fn test_interchange_ea() -> Result<(), LogicError> {
//         let formula1 = &Formula::new("Aa:~Eu':(a+u')=Sa");
//         let formula2 = &Formula::new("[Aa:~Eu':(a+u')=Sa&~Eu':u'=SS0]");
//         let variable = &Variable::new("u'");
//         assert_eq!(
//             interchange_ea(formula1, variable, 0)?.to_string(),
//             "Aa:Au':~(a+u')=Sa"
//         );
//         assert_eq!(
//             interchange_ea(formula2, variable, 1)?.to_string(),
//             "[Aa:~Eu':(a+u')=Sa&Au':~u'=SS0]"
//         );
//         Ok(())
//     }

//     #[test]
//     fn test_interchange_ea_err() {
//         let formula1 = &Formula::new("Aa:~Eu':(a+u')=Sa");
//         let variable = &Variable::new("z");
//         assert!(interchange_ea(formula1, variable, 0).is_err());
//     }

//     #[test]
//     fn test_interchange_ae() -> Result<(), LogicError> {
//         let formula1 = &Formula::new("Aa:Au':~(a+u')=Sa");
//         let formula2 = &Formula::new("[Aa:~Eu':(a+u')=Sa&Au':~u'=SS0]");
//         let variable = &Variable::new("u'");
//         assert_eq!(
//             interchange_ae(formula1, variable, 0)?.to_string(),
//             "Aa:~Eu':(a+u')=Sa"
//         );
//         assert_eq!(
//             interchange_ae(formula2, variable, 0)?.to_string(),
//             "[Aa:~Eu':(a+u')=Sa&~Eu':u'=SS0]"
//         );
//         Ok(())
//     }

//     #[test]
//     fn test_interchange_ae_err() {
//         let formula1 = &Formula::new("Aa:~Eu':(a+u')=Sa");
//         let variable = &Variable::new("z");
//         assert!(interchange_ae(formula1, variable, 0).is_err());
//     }

//     #[test]
//     fn test_induction() -> Result<(), LogicError> {
//         let v = &Variable::new("v");
//         let base = &Formula::new("0=0");
//         let gen = &Formula::new("Av:[v=v>Sv=Sv]");
//         assert_eq!(induction(v, base, gen)?.to_string(), "Av:v=v");
//         Ok(())
//     }
// }
