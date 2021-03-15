use onig::Regex;
use num::{bigint::BigUint};
use std::str::from_utf8;


pub fn to_latex(text: String, indent: usize) -> String {
    let mut latex = "".to_string();
    latex.push_str(&format!("\\hspace{{{}em}}",1+2*indent));

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
            return format!("for no {}: ", var)
        } else if nth_char(text,1) == "A" {
            return format!("it is not true that for all {}: ", var)
        } else {
            panic!("found invalid quantifier during translation")
        }

    } else {
        let var = &text[1..text.len()-1];
        if nth_char(text,0) == "E" {
            return format!("there exists {}: ", var)
        } else if nth_char(text,0) == "A" {
            return format!("for all {}: ", var)
        } else {
            panic!("found invalid quantifier during translation")
        }
    }
}

// Should handle conjuctions of quantifiers
pub fn english_all_quants(text: String) -> String {
    let mut text = text;
    let quant = Regex::new("~?[AE][a-z]\'*:").unwrap();
    let mut q = quant.find(&text);
    while q.is_some() {
        let (lo,hi) = q.unwrap();
        if hi+1 == text.len() {
            panic!("quantifiers cannot appear at the end of a formula")
        }
        let nice_name = english_quant(&text[lo..hi]);
        text.replace_range(lo..hi, &nice_name);

        q = quant.find(&text);
    }

    text
}

pub fn english_num(text: String) -> String {
    let mut text = text;
    let num = Regex::new("S+0").unwrap();
    let mut n = num.find(&text);
    while n.is_some() {
        let (lo,hi) = n.unwrap();
        text.replace_range(lo..hi, &format!("{}",hi-lo-1));
        n = num.find(&text);
    }
    text
}


pub fn to_english(text: String) -> String {
    let mut text = text;
    text = text.replace("="," equals ");
    text = text.replace("*"," times ");
    text = text.replace("+"," plus ");
    text = text.replace(">"," implies that ");
    text = text.replace("&"," and ");
    text = text.replace("|"," or ");
    text = english_all_quants(text);
    text = english_num(text);
    text
}

// Each symbol could be compressed into 6 bits instead of eight but this is easier
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
    let s1 = "Az:~Eb:(z+b)=0".to_string();
    let s2 = "[~Ao':o'*SS0=0>Eb:Ec:(0*(b+c))=S0]".to_string();
    assert_eq!(to_english(s1),"for all z: for no b: (z plus b) equals 0");
    assert_eq!(to_english(s2),"[it is not true that for all o': o' times 2 equals 0 implies that there exists b: there exists c: (0 times (b plus c)) equals 1]");
}