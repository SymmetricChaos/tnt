use fancy_regex::Regex;
use num::bigint::BigUint;
use std::str::from_utf8;
use lazy_static::lazy_static;
use crate::{properties::{is_expression, is_num, is_var}, string_manip::{get_vars,replace_all_re,split_arithmetic,left_match}};

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



pub fn to_latex(text: String) -> String {
    let mut latex = "".to_string();

    let mut text = text;
    text = text.replace("A","\\forall ");
    text = text.replace("E","\\exists ");
    text = text.replace("*","\\cdot ");
    text = text.replace(">","\\rightarrow ");
    text = text.replace("&","\\wedge ");
    text = text.replace("|","\\vee ");
    text = text.replace("[","\\langle ");
    text = text.replace("]","\\rangle ");
    text = text.replace("~", "\\neg ");

    latex.push_str(&text);

    latex
}

pub fn english_quant_chains(text: String) -> String {
    let mut text = text;
    let mut exists = EXISTS_CHAIN.find(&text).unwrap();

    while exists.is_some() {
        let range = exists.unwrap().range();
        let vars = get_vars(&text[range.clone()]);
        if &text[range.clone()].matches('E').count() == &1 {
            let replacement = format!("there exists {}, such that ", vars.join(""));
            text.replace_range(range, &replacement);
        } else {
            let replacement = format!("there exist {}, such that ", vars.join(" and "));
            text.replace_range(range, &replacement);
        }
        exists = EXISTS_CHAIN.find(&text).unwrap();
    }

    let mut forall = FORALL_CHAIN.find(&text).unwrap();

    while forall.is_some() {
        let range = forall.unwrap().range();
        let vars = get_vars(&text[range.clone()]);
        let replacement = format!("for all {}, ", vars.join(" and "));
        text.replace_range(range, &replacement);
        forall = FORALL_CHAIN.find(&text).unwrap();
    }

    let mut for_no = NOT_EXISTS_CHAIN.find(&text).unwrap();

    while for_no.is_some() {
        let range = for_no.unwrap().range();
        let vars = get_vars(&text[range.clone()]);
        let replacement = format!("there is no {}, such that ", vars.join(" or "));
        text.replace_range(range, &replacement);
        for_no = NOT_EXISTS_CHAIN.find(&text).unwrap();
    }
    
    let mut not_all = NOT_FORALL_CHAIN.find(&text).unwrap();

    while not_all.is_some() {
        let range = not_all.unwrap().range();
        let vars = get_vars(&text[range.clone()]);
        let replacement = format!("it is not true that for all {}, ", vars.join(" and "));
        text.replace_range(range, &replacement);
        not_all = NOT_FORALL_CHAIN.find(&text).unwrap();
    }

    text
}

pub fn english_num(text: String) -> String {
    let mut text = text;
    let mut n = NUM_GEQ_ONE.find(&text).unwrap();
    while n.is_some() {
        let lo = n.unwrap().start();
        let hi = n.unwrap().end();
        text.replace_range(lo..hi, &format!("{}",hi-lo-1));
        n = NUM_GEQ_ONE.find(&text).unwrap();
    }
    text
}

pub fn english_var_successor(text: String) -> String {
    let mut text = text;
    let mut n = SUCC_VAR_MIN_ONE.find(&text).unwrap();
    while n.is_some() {
        let lo = n.unwrap().start();
        let hi = n.unwrap().end();
        let substr = &text.clone()[lo..hi];
        let addend = substr.matches("S").count();
        let var = &substr[addend..];
        text.replace_range(lo..hi, &format!("({} + {})",var,addend));
        n = SUCC_VAR_MIN_ONE.find(&text).unwrap();
    }
    text
}

// Final implementation should consume from left to right finding all parenthesized expressions
pub fn english_expr(text: String) -> String {
    if is_num(&text) {
        return format!("{}",text.len()-1)
    } else if is_var(&text) {
        let addend = text.matches("S").count();
        if addend == 0 {
            return text
        } else {
            return format!("({} + {})",&text[addend..],addend)
        }
    } else if is_expression(&text) {
        // Calculate the addend for the whole expression
        let mut out = text.clone();
        let prelen = out.len();
        out = out.trim_start_matches('S').to_string();
        let addend = prelen-out.len();

        // Replace the left and right sides
        out = match split_arithmetic(&out) {
            Some((lhs, rhs, op)) => format!("({} {} {})",english_expr(lhs.to_string()),op,english_expr(rhs.to_string())),
            None => out,
        };

        // Use the addend if needed
        if addend != 0 {
            out = format!("({} + {})",out,addend);
        }
        return out
    } else {
        return text
    }

}

pub fn english_expressions(text: String) -> String {
    let mut text = text;
    let mut prev = "".to_string();
    let mut m = left_match(&text, vec!['('], vec![')']);
    while m.is_some() {
        let (lo, hi) = m.unwrap();
        prev.push_str(&text[..lo]);
        prev.push_str(&english_expr(text[lo..hi+1].to_string()));
        let sp = text.split_at(hi+1);
        text = sp.1.to_string();
        m = left_match(&text, vec!['('], vec![')']);
    }
    prev.push_str(&text);
    prev
}

pub fn to_english(text: String) -> String {
    let mut text = text;
    text = english_expressions(text);
    text = text.replace("="," = ");
    text = text.replace("*","×");
    text = text.replace(">"," implies that ");
    text = text.replace("&"," and ");
    text = text.replace("|"," or ");
    text = english_quant_chains(text);
    text = english_num(text);
    text = english_var_successor(text);
    text
}

/// Each symbol could be represented with 6 bits instead of eight but this is much easier
pub fn arithmetize(text: String) -> BigUint {
    BigUint::from_bytes_be(&text.into_bytes())
}

pub fn dearithmetize(number: &BigUint) -> String {
    match from_utf8(&number.to_bytes_be()) {
        Ok(s) => s.to_string(),
        Err(e) => panic!("{}",e), 
    }
}

pub fn to_austere(text: String) -> String {
    let mut out = text.clone();
    let vars = get_vars(&text);
    let len = vars.len();
    
    let mut mask = "#".repeat(len);
    for v in vars {
        let re = Regex::new(&format!("{}(?!')",v)).unwrap();
        out = replace_all_re(&out, &re, &mask);
        mask.remove(0);
    }

    let mut a_var = "a".to_string();
    let mut mask = "#".repeat(len);
    for _ in 0..len {
        out = out.replace(&mask, &a_var);
        a_var.push_str("'");
        mask.remove(0);
    }

    out
}



#[test]
fn test_to_english() {
    let s1 = "Az:~Eb:(z+b)=SSS0".to_string();
    let s2 = "[~Ao':(o'*SS0)=0>Eb:Ec:(0*S(b+SSc'))=S0]".to_string();
    let s3 = "Aa:Ab:Ec:[(a+S0)=c&(b+0)=c]".to_string();
    let s4 = "S(0+a)=(SS0*S(b+b))".to_string();
    assert_eq!(to_english(s1.clone()),"for all z, there is no b, such that (z + b) = 3");
    assert_eq!(to_english(s2.clone()),"[it is not true that for all o\', (o\' × 2) = 0 implies that there exist b and c, such that (0 × ((b + (c\' + 2)) + 1)) = 1]");
    assert_eq!(to_english(s3.clone()),"for all a and b, there exists c, such that [(a + 1) = c and (b + 0) = c]");
    assert_eq!(to_english(s4.clone()),"((0 + a) + 1) = (2 × ((b + b) + 1))");
}

#[test]
fn test_arithmetize() {

}

#[test]
fn test_to_austere() {
    let s1 = "Aa':Ez'':[(z+0)=a'|(a'*z'')=SSa]".to_string();
    assert_eq!(to_austere(s1.clone()),"Aa:Ea':[(a''+0)=a|(a*a')=SSa''']");
}

#[test]
fn test_english_expr_successor() {
    let s1 = "SS(S0*Sa')".to_string();
    assert_eq!(english_expr(s1),"((1 * (a' + 1)) + 2)");

    let s2 = "[~Ao':(o'*SS0)=0>Eb:Ec:(0*S(b+SSc'))=S0]".to_string();
    println!("{}",english_expressions(s2));
}