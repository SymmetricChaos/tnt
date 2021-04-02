

pub fn scan_text(text: String) -> String {
    let mut used = "".to_string();
    let mut text = text.clone();

    let q_chars = "abcdefghijklmnopqrstuvwxyz'AE:~";
    let e_chars = "abcdefghijklmnopqrstuvwxyz'+*S()";


    let split = left_quant(&text);

    let (l,r) = match split {
        Some(n) => (&text[..n],&text[n..]),
        None => (&text[..],"")
    };

    println!("{}  {}",l,r);


    used
}


fn left_quant(s: &str) -> Option<usize> {
    let q_chars = "abcdefghijklmnopqrstuvwxyzAE':~";
    let sym = s.char_indices();
    for (pos,c) in sym {
        if !q_chars.contains(c) {
            return Some(pos)
        }
    }
    None
}

fn left_expr(s: &str) -> Option<usize> {
    let e_chars = "abcdefghijklmnopqrstuvwxyz+*S()";
    let sym = s.char_indices();
    for (pos,c) in sym {
        if !e_chars.contains(c) {
            return Some(pos)
        }
    }
    None
}




#[test]
fn test_scan() {
    let s1 = "~Ab':Ez:S(S0*a)=(b'+z)".to_string();
    scan_text(s1);

}