//! Arbitrary logical combinations of the inputs.
use crate::types::Formula;
use crate::terms::{Term,Variable};

// These are guaranteed to produce well-formed formulas of TNT. However they may produce false statements.

/// Equality of two types that implement Term
/// ```
/// use tnt::types::{Variable,Term};
/// use tnt::operations::construction::eq;
/// let a = &Variable::new("a");
/// eq(a,a); // a=a
/// ```
pub fn eq<A: Term, B: Term>(x: &A, y: &B) -> Formula {
    let new_s = format!("{}={}",x.get_string(),y.get_string());
    Formula::new_simple(&new_s)
}

/// Negation of a Formula
/// ```
/// use tnt::types::Formula;
/// use tnt::operations::construction::not;
/// let f = &Formula::new("Ea:Sa=SS0");
/// not(f); // ~Ea:Sa=SS0:
/// ```
pub fn not(x: &Formula) -> Formula {
    let new_s = format!("~{}",x);
    Formula::new_complex(&new_s)
}

/// Logical OR of two Formulas
/// ```
/// use tnt::types::Formula;
/// use tnt::operations::construction::or;
/// let f1 = &Formula::new("Aa:a=0");
/// let f2 = &Formula::new("Aa:Eb:a=Sb");
/// or(f1,f2); // [Aa:a=0|Aa:Eb:a=Sb]:
/// ```
/// 
pub fn or(x: &Formula, y: &Formula) -> Formula {
    let new_s = format!("[{}|{}]",x,y);
    Formula::new_complex(&new_s)
}

/// Logical AND of two Formulas
/// ```
/// use tnt::types::Formula;
/// use tnt::operations::construction::and;
/// let f1 = &Formula::new("Aa:a=0");
/// let f2 = &Formula::new("Aa:Eb:a=Sb");
/// and(f1,f2); // [Aa:a=0&Aa:Eb:a=Sb]:
/// ```
/// 
pub fn and(x: &Formula, y: &Formula) -> Formula {
    let new_s = format!("[{}&{}]",x,y);
    Formula::new_complex(&new_s)
}

/// Left Formula implies Right Formula
/// ```
/// use tnt::types::Formula;
/// use tnt::operations::construction::implies;
/// let f1 = &Formula::new("Aa:a=0");
/// let f2 = &Formula::new("Aa:Eb:a=Sb");
/// implies(f1,f2); // [Aa:a=0>Aa:Eb:a=Sb]:
/// ```
/// 
pub fn implies(x: &Formula, y: &Formula) -> Formula {
    let new_s = format!("[{}>{}]",x,y);
    Formula::new_complex(&new_s)
}

/// Assert some value for a Variable makes the Forumla true
/// ```
/// use tnt::types::{Variable,Formula};
/// use tnt::operations::construction::exists;
/// let a = &Formula::new("a");
/// let f = &Formula::new("(Sa*SS0)=Sb");
/// implies(a,f); // Ea:(Sa*SS0)=Sb
/// ```
/// 
pub fn exists(v: &Variable, x: &Formula) -> Formula {
    Formula::new_complex(&format!("E{}:{}",v,x))
}

/// Assert that all values of a Variable make the Formula true
/// ```
/// use tnt::types::{Variable,Formula};
/// use tnt::operations::construction::forall;
/// let a = &Formula::new("a");
/// let f = &Formula::new("(Sa*SS0)=Sb");
/// forall(a,f); // Aa:(Sa*SS0)=Sb
/// ```
/// 
pub fn forall(v: &Variable, x: &Formula) -> Formula {
    Formula::new_complex(&format!("A{}:{}",v,x))
}



// TODO: test pathalogical inputs for all of these
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_eq() {
        use crate::terms::{Number,Expression};
        let a = &Variable::new("a");
        let x = &Variable::new("x''");
        let zero = &Number::new("0");
        let one = &Number::new("S0");
        let expression = &Expression::new("((b*SSS0)+Sv')");
        assert_eq!(eq(a,x).to_string(),"a=x''");
        assert_eq!(eq(x,zero).to_string(),"x''=0");
        assert_eq!(eq(expression,one).to_string(),"((b*SSS0)+Sv')=S0");
    }
    
    #[test]
    fn test_not() {
        let a = &Formula::new("a=a");
        let b = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
        assert_eq!(not(a).to_string(),"~a=a");
        assert_eq!(not(b).to_string(),"~~Ab':[(b'+b')=S0|S0=b']");
    }
    
    #[test]
    fn test_or() {
        let a = &Formula::new("a=a");
        let b = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
        let c = &Formula::new("Ec:(c+S0)=0");
        assert_eq!(or(a,b).to_string(),"[a=a|~Ab':[(b'+b')=S0|S0=b']]");
        assert_eq!(or(a,c).to_string(),"[a=a|Ec:(c+S0)=0]");
    }
    
    #[test]
    fn test_and() {
        let a = &Formula::new("a=a");
        let b = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
        let c = &Formula::new("Ec:(c+S0)=0");
        assert_eq!(and(a,b).to_string(),"[a=a&~Ab':[(b'+b')=S0|S0=b']]");
        assert_eq!(and(a,c).to_string(),"[a=a&Ec:(c+S0)=0]");
    }
    
    #[test]
    fn test_implies() {
        let a = &Formula::new("a=a");
        let b = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
        let c = &Formula::new("Ec:(c+S0)=0");
        assert_eq!(implies(a,b).to_string(),"[a=a>~Ab':[(b'+b')=S0|S0=b']]");
        assert_eq!(implies(a,c).to_string(),"[a=a>Ec:(c+S0)=0]");
    }
    
    #[test]
    fn test_exists() {
        let a = &Variable::new("a");
        let x = &Variable::new("x''");
        let f0 = &Formula::new("a=a");
        let f1 = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
        assert_eq!(exists(a,f0).to_string(),"Ea:a=a");
        assert_eq!(exists(x,f1).to_string(),"Ex'':~Ab':[(b'+b')=S0|S0=b']");
    }
    
    #[test]
    fn test_forall() {
        let a = &Variable::new("a");
        let x = &Variable::new("x''");
        let f0 = &Formula::new("a=a");
        let f1 = &Formula::new("~Ab':[(b'+b')=S0|S0=b']");
        assert_eq!(forall(a,f0).to_string(),"Aa:a=a");
        assert_eq!(forall(x,f1).to_string(),"Ax'':~Ab':[(b'+b')=S0|S0=b']");
    }
}

