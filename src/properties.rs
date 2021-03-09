use onig::Regex;

use crate::string_manip::{strip_succ_all,split_arithmetic,split_logical,get_vars,get_bound_vars,get_free_vars,strip_quant};

pub fn is_var(s: &str) -> bool {
    // Valid variables are any one of the lower case letters a to z followed by zero or more apostophes
    let re = Regex::new(r"^[a-z]'*$").unwrap();
    return re.is_match(&s)
}


pub fn is_num(s: &str) -> bool {
    if !s.contains("0") {
        return false
    } else if strip_succ_all(s) == "0" {
        return true
    }
    return false
}

pub fn is_equation(s: &str) -> bool {
    // Any arithmetic combination of numbers and variable or successors of them is an equation
    // This will match any Term
    let s = strip_succ_all(s);
    if is_var(s) || is_num(s) {
        return true
    } else {
        let (l,r) = match split_arithmetic(s) {
            Some(t) => t,
            None => return false
        };
        return is_equation(l) && is_equation(r)
    }
}





pub fn is_well_quantified(s: &str) -> bool {
    // Variables used, ignoring quantifiers
    let vars = get_vars(strip_quant(s));
    // Bound variables
    let bvars = get_bound_vars(s);
    for b in bvars {
        if !vars.contains(&b) {
            return false
        }
    }
    true
}

pub fn is_simple_formula(s: &str) -> bool {
    // A simple formula is precisely an equivalence of any two terms
    let eq = match s.find("=") {
        Some(num) => num,
        None => return false
    };
    let l = &s[0..eq];
    let r = &s[eq+1..];
    is_equation(l) && is_equation(r)
}

pub fn is_complex_formula(s: &str) -> bool {
    // Complex formula is any well formed formula that is not simple
    if !is_well_quantified(s) {
        return false
    }
    let s = strip_quant(s);
    if is_simple_formula(s) || is_equation(s) {
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

pub fn is_formula(s: &str) -> bool {
    if !is_well_quantified(s) {
        return false
    }
    let s = strip_quant(s);
    is_simple_formula(s) || is_complex_formula(s)
}



// Generally need more adversarial inputs for this, especially invalid strings

// Test the following valid strings:
// a, z', 0, S0, Sa+Sb a=x, S0 = Sv, <~∃b:~a=b∧∀c:~a=c>, <~(a+b)=0➔<a=b∨0=S0>>

// Test the following invalid strings:
// 'k, SS(), <~∃b:a=a∧c=c>, ∃a':∀a:a=a

#[test]
fn test_is_var() {
    assert_eq!(is_var("a"),true,"a is a variable");
    assert_eq!(is_var("z'"),true,"z' is a variable");
    assert_eq!(is_var("0"),false,"0 is not a variable");
    assert_eq!(is_var("S0"),false,"S0 is not a variable");
    assert_eq!(is_var("(Sa+Sb)"),false,"(Sa+Sb) is not a variable");
    assert_eq!(is_var("a=x"),false,"a=x is not a variable");
    assert_eq!(is_var("S0=Sv"),false,"S0=Sv is not a variable");
    assert_eq!(is_var("<~∃b:~a=b∧∀c:~a=c>"),false,"<~∃b:~a=b∧∀c:~a=c> is not a variable");
    assert_eq!(is_var("<~(a+b)=0⊃<a=b∨0=S0>>"),false,"<~(a+b)=0⊃<a=b∨0=S0>> is not a variable");
    assert_eq!(is_var("∃a':∀a:a=a"),false,"∃a':∀a:a=a should be rejected, it is malformed");
    assert_eq!(is_var("'k"),false,"'k should be rejected, it is malformed");
    assert_eq!(is_var("SS()"),false,"SS() should be rejected, it is malformed");
    assert_eq!(is_var("<~∃b:a=a∧c=c>"),false,"<~∃b:a=a∧c=c> should be rejected, it is malformed");
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
    assert_eq!(is_num("<~∃b:~a=b∧∀c:~a=c>"),false,"<~∃b:~a=b∧∀c:~a=c> is not a number");
    assert_eq!(is_num("<~(a+b)=0⊃<a=b∨0=S0>>"),false,"<~(a+b)=0⊃<a=b∨0=S0>> is not a number");
    assert_eq!(is_num("∃a':∀a:a=a"),false,"∃a':∀a:a=a should be rejected, it is malformed");
    assert_eq!(is_num("'k"),false,"'k should be rejected, it is malformed");
    assert_eq!(is_num("SS()"),false,"SS() should be rejected, it is malformed");
    assert_eq!(is_num("<~∃b:a=a∧c=c>"),false,"<~∃b:a=a∧c=c> should be rejected, it is malformed");
}


#[test]
fn test_is_term() {
    assert_eq!(is_equation("a"),true,"a is a term");
    assert_eq!(is_equation("z'"),true,"z' is a term");
    assert_eq!(is_equation("0"),true,"0 is a term");
    assert_eq!(is_equation("S0"),true,"S0 is a term");
    assert_eq!(is_equation("(Sa+Sb)"),true,"(Sa+Sb) is a term");
    assert_eq!(is_equation("a=x"),false,"a=x is not a term");
    assert_eq!(is_equation("S0=Sv"),false,"S0=Sv is not a term");
    assert_eq!(is_equation("<~∃b:~a=b∧∀c:~a=c>"),false,"<~∃b:~a=b∧∀c:~a=c> is not a term");
    assert_eq!(is_equation("<~(a+b)=0⊃<a=b∨0=S0>>"),false,"<~(a+b)=0⊃<a=b∨0=S0>> is not a term");
    assert_eq!(is_equation("∃a':∀a:a=a"),false,"∃a':∀a:a=a should be rejected, it is malformed");
    assert_eq!(is_equation("'k"),false,"'k should be rejected, it is malformed");
    assert_eq!(is_equation("SS()"),false,"SS() should be rejected, it is malformed");
    assert_eq!(is_equation("<~∃b:a=a∧c=c>"),false,"<~∃b:a=a∧c=c> should be rejected, it is malformed");
}

#[test]
fn test_is_atom() {
    assert_eq!(is_simple_formula("a"),false,"a is not an atom");
    assert_eq!(is_simple_formula("z'"),false,"z' is not an atom");
    assert_eq!(is_simple_formula("0"),false,"0 is not an atom");
    assert_eq!(is_simple_formula("S0"),false,"S0 is not an atom");
    assert_eq!(is_simple_formula("(Sa+Sb)"),false,"(Sa+Sb) is not an atom");
    assert_eq!(is_simple_formula("a=x"),true,"a=x is an atom");
    assert_eq!(is_simple_formula("S0=Sv"),true,"S0=Sv is an atom");
    assert_eq!(is_simple_formula("<~∃b:~a=b∧∀c:~a=c>"),false,"<~∃b:~a=b∧∀c:~a=c> is not an atom");
    assert_eq!(is_simple_formula("<~(a+b)=0⊃<a=b∨0=S0>>"),false,"<~(a+b)=0⊃<a=b∨0=S0>> is not an atom");
    assert_eq!(is_simple_formula("∃a':∀a:a=a"),false,"∃a':∀a:a=a should be rejected, it is malformed");
    assert_eq!(is_simple_formula("'k"),false,"'k should be rejected, it is malformed");
    assert_eq!(is_simple_formula("SS()"),false,"SS() should be rejected, it is malformed");
    assert_eq!(is_simple_formula("<~∃b:a=a∧c=c>"),false,"<~∃b:a=a∧c=c> should be rejected, it is malformed");
}

#[test]
fn test_is_compound() {
    assert_eq!(is_complex_formula("a"),false,"a is not a compound");
    assert_eq!(is_complex_formula("z'"),false,"z' is not a compound");
    assert_eq!(is_complex_formula("0"),false,"0 is not a compound");
    assert_eq!(is_complex_formula("S0"),false,"S0 is not a compound");
    assert_eq!(is_complex_formula("(Sa+Sb)"),false,"(Sa+Sb) is not an compound");
    assert_eq!(is_complex_formula("a=x"),false,"a=x is not a compound");
    assert_eq!(is_complex_formula("S0=Sv"),false,"S0=Sv is not a compound");
    assert_eq!(is_complex_formula("<~∃b:~a=b∧∀c:~a=c>"),true,"<~∃b:~a=b∧∀c:~a=c> is a compound");
    assert_eq!(is_complex_formula("<~(a+b)=0⊃<a=b∨0=S0>>"),true,"<~(a+b)=0⊃<a=b∨0=S0>> is a compound");
    assert_eq!(is_complex_formula("∃a':∀a:a=a"),false,"∃a':∀a:a=a should be rejected, it is malformed");
    assert_eq!(is_complex_formula("'k"),false,"'k should be rejected, it is malformed");
    assert_eq!(is_complex_formula("SS()"),false,"SS() should be rejected, it is malformed");
    assert_eq!(is_complex_formula("<~∃b:a=a∧c=c>"),false,"<~∃b:a=a∧c=c> should be rejected, it is malformed");
}

#[test]
fn test_is_well_formed_formula() {
    assert_eq!(is_formula("a"),false,"a is not a well-formed formula");
    assert_eq!(is_formula("z'"),false,"z' is not a well-formed formula");
    assert_eq!(is_formula("0"),false,"0 is not a well-formed formula");
    assert_eq!(is_formula("S0"),false,"S0 is not a well-formed formula");
    assert_eq!(is_formula("(Sa+Sb)"),false,"(Sa+Sb) is not a well-formed formula");
    assert_eq!(is_formula("a=x"),true,"a=x is a well-formed formula");
    assert_eq!(is_formula("S0=Sv"),true,"S0=Sv is a well-formed formula");
    assert_eq!(is_formula("<~∃b:~a=b∧∀c:~a=c>"),true,"<~∃b:~a=b∧∀c:~a=c> is a well-formed formula");
    assert_eq!(is_formula("<~(a+b)=0⊃<a=b∨0=S0>>"),true,"<~(a+b)=0⊃<a=b∨0=S0>> is a well-formed formula");
    assert_eq!(is_formula("∃a':∀a:a=a"),false,"∃a':∀a:a=a should be rejected, it is malformed");
    assert_eq!(is_formula("'k"),false,"'k should be rejected, it is malformed");
    assert_eq!(is_formula("SS()"),false,"SS() should be rejected, it is malformed");
    assert_eq!(is_formula("<~∃b:a=a∧c=c>"),false,"<~∃b:a=a∧c=c> should be rejected, it is malformed");
}