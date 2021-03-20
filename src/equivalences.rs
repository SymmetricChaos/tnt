use crate::types::{Term,Formula,TNT};
use crate::string_manip::left_match;

// Return the contents of every pair of [] brackets (the ones used for our logical operations), including overlaps, 
// along with their start position and length in the original string
// Returns None if the brackets are malformed
fn all_brackets(s: &str) -> Option<Vec<(usize,usize)>> {
    let mut starts: Vec<usize> = Vec::new();
    let mut spans: Vec<(usize,usize)> = Vec::new();

    let sym = s.char_indices();
    for (pos,c) in sym {
        if c == '[' {
            starts.push(pos);
            continue
        } else if c == ']' {
            let left = starts.pop();
            match left {
                Some(num) => spans.push((num,pos)),
                None => return None,
            }
        }
    }

    if starts.len() != 0 {
        return None
    }

    // Return the substrings, their starts, their ends, and the position of the next character
    // This construction gives the contents of the bracket without the brackets themselves
    let mut out: Vec<(usize,usize)> = Vec::new();
    for (lo,hi) in spans {
        out.push((lo,hi))
    }

    Some(out)
}

fn extract_substrings(s: &String, positions: Vec<(usize,usize)>) -> Vec<&str> {
    let mut out = Vec::<&str>::new();
    for (lo, hi) in positions {
        out.push(&s[lo+1..hi])
    }
    out
}

// p&p -> p
// p|p -> p
pub fn indempotence(formula: &TNT) -> Option<TNT> {
    let s = formula.to_string();
    let leftmost = match left_match(&s, vec!['['],vec!['&','|']) {
        Some(v) => v,
        None => return None
    };
    let l = &s[1..leftmost.1];
    let r = &s[leftmost.1+1..s.len()-1];
    if l == r {
        return Some(TNT::new(l));
    } else {
        return None
    }
}



#[test]
fn test_all_brackets() {
    let s0 = "[[a=b|c=d]>[a=c&b=d]]".to_string();
    let b0 = all_brackets(&s0);
    println!("{:?}",extract_substrings(&s0, b0.unwrap()));
}

#[test]
fn test_indempotence() {
    let t0 = TNT::new("[a=a|a=a]");
    let t1 = TNT::new("[(b+S0)=SS0&(b+S0)=SS0]");
    assert_eq!(indempotence(&t0).unwrap().to_string(),"a=a");
    assert_eq!(indempotence(&t1).unwrap().to_string(),"(b+S0)=SS0");
}