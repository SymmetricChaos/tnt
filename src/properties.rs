use fancy_regex::Regex;
use lazy_static::lazy_static;

use crate::string_manip::{strip_succ_all, split_arithmetic, split_logical, get_unquant_vars, get_bound_vars, get_free_vars, strip_quant};

lazy_static! {
    pub static ref VAR: Regex = Regex::new("^[a-z]\'*$").unwrap();
    pub static ref NUM: Regex = Regex::new("^S*0$").unwrap();
    pub static ref NUM_GEQ_ONE: Regex = Regex::new("S+0").unwrap();
    pub static ref FORALL_CHAIN: Regex = Regex::new("((?<!~)A[a-z]\'*:)+").unwrap();
    pub static ref NOT_FORALL_CHAIN: Regex = Regex::new("(~A[a-z]\'*:)+").unwrap();
    pub static ref EXISTS_CHAIN: Regex = Regex::new("((?<!~)E[a-z]\'*:)+").unwrap();
    pub static ref NOT_EXISTS_CHAIN: Regex = Regex::new("(~E[a-z]\'*:)+").unwrap();
}

pub fn is_var(s: &str) -> bool {
    return VAR.is_match(&s).unwrap()
}

pub fn is_num(s: &str) -> bool {
    return NUM.is_match(&s).unwrap()
}

/// This will check if the given &str can form a Variable, Number, or Expression.
pub fn is_expression(s: &str) -> bool {
    let s = strip_succ_all(s);
    if is_var(s) || is_num(s) {
        return true
    } else {
        let (l,r,_) = match split_arithmetic(s) {
            Some(t) => t,
            None => return false
        };
        return is_expression(l) && is_expression(r)
    }
}




// A well-formed formula must have no bound variables that are unused
pub fn is_well_quantified(s: &str) -> bool {
    let vars = get_unquant_vars(s);
    let bvars = get_bound_vars(s);
    for b in bvars {
        if !vars.contains(&b) {
            return false
        }
    }
    true
}

/// A simple formula is precisely an equivalence of any two terms.
pub fn is_simple_formula(s: &str) -> bool {
    let eq = match s.find("=") {
        Some(num) => num,
        None => return false
    };
    let l = &s[0..eq];
    let r = &s[eq+1..];
    is_expression(l) && is_expression(r)
}

/// Complex formula is any well formed formula that is not simple
pub fn is_complex_formula(s: &str) -> bool {
    if !is_well_quantified(s) {
        return false
    }
    let s = strip_quant(s);
    if is_simple_formula(s) || is_expression(s) {
        return false
    } else {
        let (l,r) = match split_logical(s) {
            Some(v) => v,
            None => return false
        };
        // If a variable is free on one side of a logical statement it cannot be bound on the other side
        let lfv = get_free_vars(l);
        let rfv = get_free_vars(r);
        let lbv = get_bound_vars(l);
        let rbv = get_bound_vars(r);

        for v in lfv {
            if rbv.contains(&v) {
                return false
            }
        }
        for v in rfv {
            if lbv.contains(&v) {
                return false
            }
        }
        if is_simple_formula(l) {
            if is_simple_formula(r) {
                return true
            }
            return is_complex_formula(r)
        }
        if is_simple_formula(r) {
            if is_simple_formula(l) {
                return true
            }
            return is_complex_formula(l)
        }
    }
    true
}

/// Matches any &str that can be accept by Formula::new().
pub fn is_formula(s: &str) -> bool {
    if !is_well_quantified(s) {
        return false
    }
    let s = strip_quant(s);
    is_simple_formula(s) || is_complex_formula(s)
}




// Generally need more adversarial inputs for this, especially invalid strings

// Test the following valid strings:
// a, z', 0, S0, Sa+Sb a=x, S0 = Sv, [~Eb:~a=b&Ac:~a=c], [~(a+b)=0???[a=b|0=S0]], Aa:~Ec:(SSa*SSc)=b, Ea:Eb:[(SS0*a)=c&(SSSSS0*b)=c], Aa:[Ec:(a*c)=b>Ed:(d*SS0)=a]

// Test the following invalid strings:
// 'k, SS(), [~Eb:a=a&c=c], Ea':Aa:a=a

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_is_var() {
        assert_eq!(is_var("a"),true,"a is a variable");
        assert_eq!(is_var("z'"),true,"z' is a variable");
        assert_eq!(is_var("0"),false,"0 is not a variable");
        assert_eq!(is_var("S0"),false,"S0 is not a variable");
        assert_eq!(is_var("(Sa+Sb)"),false,"(Sa+Sb) is not a variable");
        assert_eq!(is_var("a=x"),false,"a=x is not a variable");
        assert_eq!(is_var("S0=Sv"),false,"S0=Sv is not a variable");
        assert_eq!(is_var("[~Eb:~a=b&Ac:~a=c]"),false,"[~Eb:~a=b&Ac:~a=c] is not a variable");
        assert_eq!(is_var("[~(a+b)=0>[a=b|0=S0]]"),false,"[~(a+b)=0>[a=b|0=S0]] is not a variable");

        assert_eq!(is_var("Ea':Aa:a=a"),false,"Ea':Aa:a=a should be rejected, it is malformed");
        assert_eq!(is_var("'k"),false,"'k should be rejected, it is malformed");
        assert_eq!(is_var("SS()"),false,"SS() should be rejected, it is malformed");
        assert_eq!(is_var("[~Eb:a=a&c=c]"),false,"[~Eb:a=a&c=c] should be rejected, it is malformed");
    }

    #[test]
    fn test_is_num() {
        assert_eq!(is_num("a"),false,"a is not a number");
        assert_eq!(is_num("z'"),false,"z' is not a number");
        assert_eq!(is_num("0"),true,"0 is a number");
        assert_eq!(is_num("S0"),true,"S0 is a number");
        assert_eq!(is_num("(Sa+Sb)"),false,"(Sa+Sb) is not a number");
        assert_eq!(is_num("a=x"),false,"a=x is not a number");
        assert_eq!(is_num("S0=Sv"),false,"S0=Sv is not a number");
        assert_eq!(is_num("[~Eb:~a=b&Ac:~a=c]"),false,"[~Eb:~a=b&Ac:~a=c] is not a number");
        assert_eq!(is_num("[~(a+b)=0>[a=b|0=S0]]"),false,"[~(a+b)=0>[a=b|0=S0]] is not a number");

        assert_eq!(is_num("Ea':Aa:a=a"),false,"Ea':Aa:a=a should be rejected, it is malformed");
        assert_eq!(is_num("'k"),false,"'k should be rejected, it is malformed");
        assert_eq!(is_num("SS()"),false,"SS() should be rejected, it is malformed");
        assert_eq!(is_num("[~Eb:a=a&c=c]"),false,"[~Eb:a=a&c=c] should be rejected, it is malformed");
    }


    #[test]
    fn test_is_equation() {
        assert_eq!(is_expression("a"),true,"a is a term");
        assert_eq!(is_expression("z'"),true,"z' is a term");
        assert_eq!(is_expression("0"),true,"0 is a term");
        assert_eq!(is_expression("S0"),true,"S0 is a term");
        assert_eq!(is_expression("(Sa+Sb)"),true,"(Sa+Sb) is a term");
        assert_eq!(is_expression("a=x"),false,"a=x is not a term");
        assert_eq!(is_expression("S0=Sv"),false,"S0=Sv is not a term");
        assert_eq!(is_expression("[~Eb:~a=b&Ac:~a=c]"),false,"[~Eb:~a=b&Ac:~a=c] is not a term");
        assert_eq!(is_expression("[~(a+b)=0>[a=b|0=S0]]"),false,"[~(a+b)=0>[a=b|0=S0]] is not a term");

        assert_eq!(is_expression("Ea':Aa:a=a"),false,"Ea':Aa:a=a should be rejected, it is malformed");
        assert_eq!(is_expression("'k"),false,"'k should be rejected, it is malformed");
        assert_eq!(is_expression("SS()"),false,"SS() should be rejected, it is malformed");
        assert_eq!(is_expression("[~Eb:a=a&c=c]"),false,"[~Eb:a=a&c=c] should be rejected, it is malformed");
    }

    #[test]
    fn test_is_simple_formula() {
        assert_eq!(is_simple_formula("a"),false,"a is not an atom");
        assert_eq!(is_simple_formula("z'"),false,"z' is not an atom");
        assert_eq!(is_simple_formula("0"),false,"0 is not an atom");
        assert_eq!(is_simple_formula("S0"),false,"S0 is not an atom");
        assert_eq!(is_simple_formula("(Sa+Sb)"),false,"(Sa+Sb) is not an atom");
        assert_eq!(is_simple_formula("a=x"),true,"a=x is an atom");
        assert_eq!(is_simple_formula("S0=Sv"),true,"S0=Sv is an atom");
        assert_eq!(is_simple_formula("[~Eb:~a=b&Ac:~a=c]"),false,"[~Eb:~a=b&Ac:~a=c] is not an atom");
        assert_eq!(is_simple_formula("[~(a+b)=0>[a=b|0=S0]]"),false,"[~(a+b)=0>[a=b|0=S0]] is not an atom");
        assert_eq!(is_simple_formula("Ea:Eb:[(SS0*a)=c&(SSSSS0*b)=c]"),false,"Ea:Eb:[(SS0*a)=c&(SSSSS0*b)=c] is not a simple formula");
        assert_eq!(is_simple_formula("Aa:[Ec:(a*c)=b>Ed:(d*SS0)=a]"),false,"Aa:[Ec:(a*c)=b>Ed:(d*SS0)=a] is not a simple formula");

        assert_eq!(is_simple_formula("Ea':Aa:a=a"),false,"Ea':Aa:a=a should be rejected, it is malformed");
        assert_eq!(is_simple_formula("'k"),false,"'k should be rejected, it is malformed");
        assert_eq!(is_simple_formula("SS()"),false,"SS() should be rejected, it is malformed");
        assert_eq!(is_simple_formula("[~Eb:a=a&c=c]"),false,"[~Eb:a=a&c=c] should be rejected, it is malformed");
    }

    #[test]
    fn test_is_complex_formula() {
        assert_eq!(is_complex_formula("a"),false,"a is not a compound");
        assert_eq!(is_complex_formula("z'"),false,"z' is not a compound");
        assert_eq!(is_complex_formula("0"),false,"0 is not a compound");
        assert_eq!(is_complex_formula("S0"),false,"S0 is not a compound");
        assert_eq!(is_complex_formula("(Sa+Sb)"),false,"(Sa+Sb) is not an compound");
        assert_eq!(is_complex_formula("a=x"),false,"a=x is not a compound");
        assert_eq!(is_complex_formula("S0=Sv"),false,"S0=Sv is not a compound");
        assert_eq!(is_complex_formula("[~Eb:~a=b&Ac:~a=c]"),true,"[~Eb:~a=b&Ac:~a=c] is a compound");
        assert_eq!(is_complex_formula("[~(a+b)=0>[a=b|0=S0]]"),true,"[~(a+b)=0>[a=b|0=S0]] is a compound");
        assert_eq!(is_complex_formula("Ea:Eb:[(SS0*a)=c&(SSSSS0*b)=c]"),true,"Ea:Eb:[(SS0*a)=c&(SSSSS0*b)=c] is a complex formula");
        assert_eq!(is_complex_formula("Aa:[Ec:(a*c)=b>Ed:(d*SS0)=a]"),true,"Aa:[Ec:(a*c)=b>Ed:(d*SS0)=a] is a complex formula");

        assert_eq!(is_complex_formula("Ea':Aa:a=a"),false,"Ea':Aa:a=a should be rejected, it is malformed");
        assert_eq!(is_complex_formula("'k"),false,"'k should be rejected, it is malformed");
        assert_eq!(is_complex_formula("SS()"),false,"SS() should be rejected, it is malformed");
        assert_eq!(is_complex_formula("[~Eb:a=a&c=c]"),false,"[~Eb:a=a&c=c] should be rejected, it is malformed");
    }

    #[test]
    fn test_is_formula() {
        assert_eq!(is_formula("a"),false,"a is not a well-formed formula");
        assert_eq!(is_formula("z'"),false,"z' is not a well-formed formula");
        assert_eq!(is_formula("0"),false,"0 is not a well-formed formula");
        assert_eq!(is_formula("S0"),false,"S0 is not a well-formed formula");
        assert_eq!(is_formula("(Sa+Sb)"),false,"(Sa+Sb) is not a well-formed formula");
        assert_eq!(is_formula("a=x"),true,"a=x is a well-formed formula");
        assert_eq!(is_formula("S0=Sv"),true,"S0=Sv is a well-formed formula");
        assert_eq!(is_formula("[~Eb:~a=b&Ac:~a=c]"),true,"[~Eb:~a=b&Ac:~a=c] is a well-formed formula");
        assert_eq!(is_formula("[~(a+b)=0>[a=b|0=S0]]"),true,"[~(a+b)=0>[a=b|0=S0]] is a well-formed formula");
        assert_eq!(is_formula("Ea:Eb:[(SS0*a)=c&(SSSSS0*b)=c]"),true,"Ea:Eb:[(SS0*a)=c&(SSSSS0*b)=c] is a well-formed formula");
        assert_eq!(is_formula("Aa:[Ec:(a*c)=b>Ed:(d*SS0)=a]"),true,"Aa:[Ec:(a*c)=b>Ed:(d*SS0)=a] is a well-formed formula");
        
        assert_eq!(is_formula("Ea':Aa:a=a"),false,"Ea':Aa:a=a should be rejected, it is malformed");
        assert_eq!(is_formula("'k"),false,"'k should be rejected, it is malformed");
        assert_eq!(is_formula("SS()"),false,"SS() should be rejected, it is malformed");
        assert_eq!(is_formula("[~Eb:a=a&c=c]"),false,"[~Eb:a=a&c=c] should be rejected, it is malformed");
    }

}