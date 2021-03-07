use onig::Regex;

pub fn strip_succ(s: &str) -> &str {
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


// Returns the contents and positions of the outermost pairs of brackets
pub fn bracket_match(s: &str, leftb: Vec<char>, rightb: Vec<char>) -> Option<Vec<(&str,usize,usize,usize)>> {
    let mut starts: Vec<usize> = Vec::new();
    let mut spans: Vec<(usize,usize,usize)> = Vec::new();

    let sym = s.char_indices();
    for (pos,c) in sym {
        if leftb.contains(&c) {
            starts.push(pos);
            continue
        } else if rightb.contains(&c) {
            let left = starts.pop();
            match left {
                Some(num) => spans.push((num,pos,c.len_utf8())),
                None => return None,
            }
        }
    }

    /*
    Not needed for our purposes
    if starts.len() != 0 {
        panic!() // Too many left brackets. How to raise a proper error?
    }
    */

    if spans.len() == 0 {
        return None
    }

    // Return the substrings, their starts, their ends, and the position of the next character
    // This construction gives the contents of the bracket without the brackets themselves
    let mut out: Vec<(&str,usize,usize,usize)> = Vec::new();
    for (lo,hi,w) in spans {
        out.push((&s[lo..hi+w],lo,hi,hi+w))
    }

    Some(out)
}

pub fn left_string(s: &str, leftb: Vec<char>, rightb: Vec<char>) -> Option<(&str,usize,usize,usize)> {
    let mut bracks = match bracket_match(s,leftb,rightb) {
        Some(v) => v,
        None => return None
    };
    bracks.sort_by_key(|x| x.1);
    Some(bracks[0])
}


pub fn split_arithmetic(s: &str) -> Option<(&str,&str)> {
    let leftmost = match left_string(s, vec!['('],vec!['·','+']) {
        Some(v) => v,
        None => return None
    };
    let l = &s[1..leftmost.2];
    let r = &s[leftmost.3..s.len()-1];
    Some((l,r))
}


pub fn split_logical(s: &str) -> Option<(&str,&str)> {
    let leftmost = match left_string(s, vec!['<'],vec!['∧','∨','🡢']) {
        Some(v) => v,
        None => return None
    };
    let l = &s[1..leftmost.2];
    let r = &s[leftmost.3..s.len()-1];
    Some((l,r))
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
    let re = Regex::new("[∀∃][a-z]\'*:").unwrap();
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
        bound.push(q[3..q.len()-1].to_owned());
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
        s = s.strip_prefix("S").unwrap();
    }
    s
}


// Remove the leading quantifiers and negations
pub fn strip_quant(s: &str) -> &str {
    let mut s = strip_neg(s);
    let re = Regex::new("^[∀∃][a-z]\'*:").unwrap();
    let mut m = re.find(s);
    while m.is_some() {
        let e = m.unwrap().1;
        s = &s[e..];
        s = strip_neg(s);
        m = re.find(s);
    }
    s
}


// Just in case we need to check directly if a variable exists in a string
pub fn var_in_string(s: &str, v: &str) -> bool {
    if s.len() < v.len() {
        return false
    }
    let w = v.len();
    let mut sym = s.char_indices().peekable();
    loop {
        let (pos,_) = sym.next().unwrap();
        if pos + w == s.len() {
            if &s[pos..] == v {
                return true
            }
            return false
        }
        // If the next value is an apostrope we know we don't have a match
        if sym.peek().unwrap().1 == '\'' {
            continue
        } else {
            if s.is_char_boundary(pos+w) {
                if &s[pos..pos+w] == v {
                    return true
                }
            }
        }
    }
}

pub fn replace_var_in_string(s: &str, pattern: &str, replacement: &str) -> String {
    // (?!') is the negative lookahead for an apostrophe so we match pattern only if it is NOT followed by an apostrophe
    let p = format!("(?m){}(?!')",pattern);
    let re = Regex::new(&p).unwrap();
    let out = re.replace_all(s,replacement);
    out.to_string()
}




#[test]
fn test_strip_quants() {
    assert_eq!(strip_quant("∀b:∃a:a=a"),"a=a");
}

#[test]
fn test_get_vars() {
    let v1 = vec!["a'","a","b"];
    assert_eq!(get_vars("∃a':∀b:(a+a')=b"),v1);
}

#[test]
fn test_get_bound_vars() {
    let v1 = vec!["a'","b"];
    assert_eq!(get_bound_vars("∃a':∀b:(a+a')=b"),v1);
}

#[test]
fn test_var_in_string() {
    assert_eq!(var_in_string("∃a':∀b:(a'+a')=b","a"),false);
    assert_eq!(var_in_string("∃a:∃b:(a'+a')=b","a''"),false);
    assert_eq!(var_in_string("∃a:∃b:(a'+a')=b","c"),false);
}


#[test]
fn test_replace_var_in_string() {
    assert_eq!(replace_var_in_string("∃a':∀b:(a'+a)=b","a","x"),"∃a':∀b:(a'+x)=b");
}

#[test]
fn test_split_arithmetic() {
    assert_eq!(split_arithmetic("(a+Sb)"),Some(("a","Sb")),"(a+Sb) should split into a and Sb");
}

