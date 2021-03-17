use onig::Regex;
use num::{bigint::BigUint};
use std::{str::from_utf8};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref QUANT: Regex = Regex::new("~?[AE][a-z]\'*:").unwrap();
    pub static ref VAR: Regex = Regex::new("[a-z]\'*").unwrap();
    pub static ref SUCC_VAR: Regex = Regex::new("S*[a-z]\'*").unwrap();
    pub static ref SUCC_VAR_MIN_ONE: Regex = Regex::new("S+[a-z]\'*").unwrap();
    pub static ref NUM: Regex = Regex::new("S*0").unwrap();
    pub static ref NUM_GEQ_ONE: Regex = Regex::new("S+0").unwrap();
    pub static ref FORALL_CHAIN: Regex = Regex::new("((?<!~)A[a-z]\'*:)+").unwrap();
    pub static ref NOT_FORALL_CHAIN: Regex = Regex::new("(~A[a-z]\'*:)+").unwrap();
    pub static ref EXISTS_CHAIN: Regex = Regex::new("((?<!~)E[a-z]\'*:)+").unwrap();
    pub static ref NOT_EXISTS_CHAIN: Regex = Regex::new("(~E[a-z]\'*:)+").unwrap();
}

pub fn get_vars(s: &str) ->  Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for st in VAR.find_iter(s) {
        out.push(s[st.0..st.1].to_owned());
    }
    out
}


pub fn to_latex(text: String) -> String {
    let mut latex = "".to_string();

    let mut text = text;
    text = text.replace("A","\\forall ");
    text = text.replace("E","\\exists ");
    text = text.replace("*","\\cdot ");
    text = text.replace(">","\\rightarrow ");
    text = text.replace("&","\\wedge ");
    text = text.replace("|","\\vee ");
    text = text.replace("[","\\langle");
    text = text.replace("]","\\rangle");

    latex.push_str(&text);

    latex
}


// ASCII ONLY
fn nth_char(text: &str, n: usize) -> &str {
    &text[n..n+1]
}

// Should be able to eliminate the ":" ending and replace it with english conjunctions in english_all_quants()
pub fn english_quant(text: &str) -> String {
    if nth_char(text,0) == "~" {
        let var = &text[2..text.len()-1];
        if nth_char(text,1) == "E" {
            return format!("there is no {}, such that ", var)
        } else if nth_char(text,1) == "A" {
            return format!("it is not true that for all {}, ", var)
        } else {
            panic!("found invalid quantifier during translation")
        }

    } else {
        let var = &text[1..text.len()-1];
        if nth_char(text,0) == "E" {
            return format!("there exists {}, such that ", var)
        } else if nth_char(text,0) == "A" {
            return format!("for all {}, ", var)
        } else {
            panic!("found invalid quantifier during translation")
        }
    }
}

// Should handle conjuctions of quantifiers
pub fn english_all_quants(text: String) -> String {
    let mut text = text;
    let mut q = QUANT.find(&text);
    while q.is_some() {
        let (lo,hi) = q.unwrap();
        if hi+1 == text.len() {
            panic!("quantifiers cannot appear at the end of a formula")
        }
        let nice_name = english_quant(&text[lo..hi]);
        text.replace_range(lo..hi, &nice_name);

        q = QUANT.find(&text);
    }
    text
}

pub fn english_quant_chains(text: String) -> String {
    let mut text = text;
    let mut exists = EXISTS_CHAIN.find(&text);

    while exists.is_some() {
        let e = exists.unwrap();
        let range = e.0..e.1;
        let vars = get_vars(&text[range.clone()]);
        let replacement = format!("there exist {}, such that ", vars.join(" and "));
        text.replace_range(range, &replacement);
        exists = EXISTS_CHAIN.find(&text);
    }

    let mut forall = FORALL_CHAIN.find(&text);

    while forall.is_some() {
        let e = forall.unwrap();
        let range = e.0..e.1;
        let vars = get_vars(&text[range.clone()]);
        let replacement = format!("for all {}, ", vars.join(" and "));
        text.replace_range(range, &replacement);
        forall = FORALL_CHAIN.find(&text);
    }

    let mut for_no = NOT_EXISTS_CHAIN.find(&text);

    while for_no.is_some() {
        let e = for_no.unwrap();
        let range = e.0..e.1;
        let vars = get_vars(&text[range.clone()]);
        let replacement = format!("there is no {}, such that ", vars.join(" or "));
        text.replace_range(range, &replacement);
        for_no = NOT_EXISTS_CHAIN.find(&text);
    }
    
    let mut not_all = NOT_FORALL_CHAIN.find(&text);

    while not_all.is_some() {
        let e = not_all.unwrap();
        let range = e.0..e.1;
        let vars = get_vars(&text[range.clone()]);
        let replacement = format!("it is not true that for all {}, ", vars.join(" and "));
        text.replace_range(range, &replacement);
        not_all = NOT_FORALL_CHAIN.find(&text);
    }

    text
}

pub fn english_num(text: String) -> String {
    let mut text = text;
    let mut n = NUM_GEQ_ONE.find(&text);
    while n.is_some() {
        let (lo,hi) = n.unwrap();
        text.replace_range(lo..hi, &format!("{}",hi-lo-1));
        n = NUM_GEQ_ONE.find(&text);
    }
    text
}

pub fn english_successor(text: String) -> String {
    let mut text = text;
    let mut n = SUCC_VAR_MIN_ONE.find(&text);
    while n.is_some() {
        let (lo,hi) = n.unwrap();
        let substr = &text.clone()[lo..hi];
        let addend = substr.matches("S").count();
        let var = &substr[addend..];
        text.replace_range(lo..hi, &format!("({} + {})",var,addend));
        n = SUCC_VAR_MIN_ONE.find(&text);
    }
    text
}


pub fn to_english(text: String) -> String {
    let mut text = text;
    text = text.replace("="," = ");
    text = text.replace("+"," + ");
    text = text.replace("*"," × ");
    text = text.replace(">"," implies that ");
    text = text.replace("&"," and ");
    text = text.replace("|"," or ");
    text = english_quant_chains(text);
    text = english_num(text);
    text = english_successor(text);
    text
}

// Each symbol could be represented with 6 bits instead of eight but this is easier 
pub fn arithmetize(text: String) -> BigUint {
    BigUint::from_bytes_be(&text.into_bytes())
}

pub fn dearithmetize(number: BigUint) -> String {
    match from_utf8(&number.to_bytes_be()) {
        Ok(s) => s.to_string(),
        Err(e) => panic!("{}",e), 
    }
}



#[test]
fn test_to_english() {
    let s1 = "Az:~Eb:(z+b)=SSS0".to_string();
    let s2 = "[~Ao':o'*SS0=0>Eb:Ec:(0*(b+SSc'))=S0]".to_string();
    let s3 = "Aa:Ab:Ec:[(a+1)=c&(b+0)=c]".to_string();
    assert_eq!(to_english(s1.clone()),"for all z, there is no b, such that (z + b) = 3");
    assert_eq!(to_english(s2.clone()),"[it is not true that for all o', o' × 2 = 0 implies that there exist b and c, such that (0 × (b + (c' + 2))) = 1]");
    assert_eq!(to_english(s3.clone()),"for all a and b, there exist c, such that [(a + 1) = c and (b + 0) = c]");

    english_quant_chains(s3);
}