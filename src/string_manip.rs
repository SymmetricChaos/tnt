use onig::Regex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref HEAD_QUANT: Regex = Regex::new("^[AE][a-z]\'*:").unwrap();
    pub static ref QUANT_VAR: Regex = Regex::new("[a-z]\'*(?=:)").unwrap();
    pub static ref UN_QUANT_VAR: Regex = Regex::new("[a-z]\'*(?!:)").unwrap();
    pub static ref VAR: Regex = Regex::new("[a-z]\'*").unwrap();
    pub static ref QUANT: Regex = Regex::new("~?[AE][a-z]\'*:").unwrap();
}

pub fn strip_succ_all(s: &str) -> &str {
    s.trim_start_matches('S')
}

// Get the outermost leftmost match for some arbitray bracketing system
pub fn left_match(s: &str, leftb: Vec<char>, rightb: Vec<char>) -> Option<(usize,usize)> {
    let mut starts: Vec<usize> = Vec::new();

    let sym = s.char_indices();
    for (pos,c) in sym {
        if leftb.contains(&c) {
            starts.push(pos);
            continue
        } else if rightb.contains(&c) {
            let left = starts.pop();
            if starts.len() == 0 && left.is_some() {
                return Some((left.unwrap(),pos))
            } else if left.is_none() {
                return None
            }
        }
    }
    None
}

pub fn split_arithmetic(s: &str) -> Option<(&str,&str)> {
    let leftmost = match left_match(s, vec!['('],vec!['*','+']) {
        Some(v) => v,
        None => return None
    };
    let l = &s[1..leftmost.1];
    let r = &s[leftmost.1+1..s.len()-1];
    Some((l,r))
}

// Partition a &str into two pieces at the outermost leftmost logical operation
// Use only to validate Formula::Complex
pub fn split_logical(s: &str) -> Option<(&str,&str)> {
    let leftmost = match left_match(s, vec!['['],vec!['&','|','>']) {
        Some(v) => v,
        None => return None
    };
    let l = &s[1..leftmost.1];
    let r = &s[leftmost.1+1..s.len()-1];
    Some((l,r))
}

// Partition a &str into two pieces at the outermost leftmost equals sign
// Use only to validate Formula::Simple
pub fn split_eq(s: &str) -> Option<(&str,&str)> {
    if !s.contains(s) {
        return None
    }
    let sp: Vec<&str> = s.splitn(2,"=").collect();
    Some((sp[0],sp[1]))
}

// Left half of an implies statement
pub fn left_implies(s: &str) -> Option<&str> {
    let leftmost = match left_match(s, vec!['['],vec!['>']) {
        Some(v) => v,
        None => return None
    };
    println!("{:?}",leftmost);
    let l = &s[leftmost.0+1..leftmost.1];
    Some(l)
}


// Vector of all variables
pub fn get_vars(s: &str) ->  Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for st in VAR.find_iter(s) {
        out.push(s[st.0..st.1].to_owned());
    }
    out
}

// Vector of all variables except the ones in quantifications, needed to check if a formula is well quantified
pub fn get_unquant_vars(s: &str) ->  Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for st in UN_QUANT_VAR.find_iter(s) {
        out.push(s[st.0..st.1].to_owned());
    }
    out
}

// Vector of string representing quantifications of variables
// Currnetly unused
pub fn _get_quants(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for st in QUANT.find_iter(s) {
        out.push(s[st.0..st.1].to_owned());
    }
    out
}

// Vector of quantified variables
pub fn get_bound_vars(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for st in QUANT_VAR.find_iter(s) {
        out.push(s[st.0..st.1].to_owned());
    }
    out
}

// Vector of variables not quantified in the formula
pub fn get_free_vars(s: &str) -> Vec<String> {
    let var = get_vars(s);
    let bound = get_bound_vars(s);
    let mut free = Vec::new();
    for v in var {
        if !bound.contains(&v) {
            free.push(v)
        }
    }
    free
}


// Remove leading negations
pub fn strip_neg(s: &str) -> &str {
    s.trim_start_matches('~')
}


// Remove the leading quantifiers and negations
pub fn strip_quant(s: &str) -> &str {
    let mut s = strip_neg(s);
    let mut m = HEAD_QUANT.find(s);
    while m.is_some() {
        let e = m.unwrap().1;
        s = &s[e..];
        s = strip_neg(s);
        m = HEAD_QUANT.find(s);
    }
    s
}

// Currently unused
// TODO: Optimization tie this directly to the variable itself so that the regex doesn't need to be built every time to variable is searched for
// (?!') is the negative lookahead for an apostrophe so we match pattern only if it is NOT followed by an apostrophe
pub fn replace_var_in_string(s: &str, pattern: &str, replacement: &str) -> String {
    let p = format!("{}(?!')",pattern);
    let re = Regex::new(&p).unwrap();
    let out = re.replace_all(s,replacement);
    out.to_string()
}


#[test]
fn test_strip_succ_all() {
    assert_eq!(strip_succ_all("SSSi'"),"i'");
}

#[test]
fn test_strip_quants() {
    assert_eq!(strip_quant("Ab:Ea:a=a"),"a=a");
}

#[test]
fn test_get_vars() {
    let v1 = vec!["a'","b","a","a'","b"];
    assert_eq!(get_vars("Ea':Ab:(a+a')=b"),v1);
}

#[test]
fn test_get_bound_vars() {
    let v1 = vec!["a'","b"];
    assert_eq!(get_bound_vars("Ea':Ab:(a+a')=b"),v1);
}

#[test]
fn test_replace_var_in_string() {
    assert_eq!(replace_var_in_string("Ea':Ab:(a'+a)=b","a","x"),"Ea':Ab:(a'+x)=b");
}

#[test]
fn test_split_arithmetic() {
    assert_eq!(split_arithmetic("(a+Sb)"),Some(("a","Sb")));
}

#[test]
fn test_split_logical() {
    assert_eq!(split_logical("[u=u>Su=Su]"),Some(("u=u","Su=Su")));
}

#[test]
fn test_left_implies() {
    assert_eq!(left_implies("Ad:[Ac:(c+d)=(d+c)>Ac:(c+Sd)=(Sd+c)]"),Some("Ac:(c+d)=(d+c)"));
}
