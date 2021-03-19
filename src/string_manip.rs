use onig::Regex;

// Currently unused
pub fn _strip_succ(s: &str) -> &str {
    let mut s = s;
    if s.starts_with("S") {
        s = s.strip_prefix("S").unwrap();
    }
    s
}


pub fn strip_succ_all(s: &str) -> &str {
    let mut s = s;
    loop {
        if s.starts_with("S") {
            s = s.strip_prefix("S").unwrap();
        } else {
            break
        }
    }
    s
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
// Used to validate Formula::Complex
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
// Used to validate Formula::Simple
pub fn split_eq(s: &str) -> Option<(&str,&str)> {
    if !s.contains(s) {
        return None
    }
    let sp: Vec<&str> = s.splitn(2,"=").collect();
    Some((sp[0],sp[1]))
}

pub fn left_implies(s: &str) -> Option<&str> {
    let leftmost = match left_match(s, vec!['['],vec!['>']) {
        Some(v) => v,
        None => return None
    };
    println!("{:?}",leftmost);
    let l = &s[leftmost.0+1..leftmost.1];
    Some(l)
}


// Set of all variables
pub fn get_vars(s: &str) ->  Vec<String> {
    let re = Regex::new(r"[a-z]'*").unwrap();
    let mut out: Vec<String> = Vec::new();
    for st in re.find_iter(s) {
        out.push(s[st.0..st.1].to_owned());
    }
    out
}


// Set of string representing quantifications of variables
pub fn get_quants(s: &str) -> Vec<String> {
    let re = Regex::new("[AE][a-z]\'*:").unwrap();
    let mut out: Vec<String> = Vec::new();
    for st in re.find_iter(s) {
        out.push(s[st.0..st.1].to_owned());
    }
    out
}


// Set of quantified variables
pub fn get_bound_vars(s: &str) -> Vec<String> {
    let quants = get_quants(s);
    let mut bound: Vec<String> = Vec::new();
    for q in quants.iter() {
        bound.push(q[1..q.len()-1].to_owned());
    }
    bound
}


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
    let mut s = s;
    while s.starts_with("~") {
        s = s.strip_prefix("~").unwrap();
    }
    s
}


// Remove the leading quantifiers and negations
pub fn strip_quant(s: &str) -> &str {
    let mut s = strip_neg(s);
    let re = Regex::new("^[AE][a-z]\'*:").unwrap();
    let mut m = re.find(s);
    while m.is_some() {
        let e = m.unwrap().1;
        s = &s[e..];
        s = strip_neg(s);
        m = re.find(s);
    }
    s
}

// Currently unused
// TODO: Optimization tie this directly to the variable itself so that the regex doesn't need to be built every time to variable is searched for
// Just in case we need to check directly if a variable exists in a string
pub fn _var_in_string(s: &str, v: &str) -> bool {
    // (?!') is the negative lookahead for an apostrophe so we match pattern only if it is NOT followed by an apostrophe
    let p = format!("{}(?!')",v);
    let re = Regex::new(&p).unwrap();
    if re.find(s).is_some() {
        return true
    } else {
        return false
    }
}


pub fn replace_var_in_string(s: &str, pattern: &str, replacement: &str) -> String {
    let p = format!("{}(?!')",pattern);
    let re = Regex::new(&p).unwrap();
    let out = re.replace_all(s,replacement);
    out.to_string()
}





#[test]
fn test_strip_succ() {
    assert_eq!(_strip_succ("SSSi'"),"SSi'");
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
fn test_var_in_string() {
    assert_eq!(_var_in_string("Ea':Ab:(a'+a')=b","a"),false);
    assert_eq!(_var_in_string("Ea:Eb:(a'+a')=b","a''"),false);
    assert_eq!(_var_in_string("Ea:Eb:(a'+a')=b","c"),false);
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
