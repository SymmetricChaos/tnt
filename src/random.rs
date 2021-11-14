use std::str::from_utf8;
use rand::Rng;

pub fn random_variable() -> String {
    let mut rng = rand::thread_rng();
    // Get a u8 corresponding to an ASCII lowercase letter and make it into a String
    let n: u8 = rng.gen_range(97..123);
    let mut s = from_utf8(&[n]).unwrap().to_string();
    while rng.gen_range(0.0..1.0) > 0.65 {
        s.push('\'')
    }
    s
}



pub fn random_number() -> String {
    let mut rng = rand::thread_rng();
    let mut s = "".to_string();
    while rng.gen_range(0.0..1.0) > 0.6 {
        s.push('S')
    }
    s.push('0');
    s
}



pub fn random_expression() -> String {

    fn simple_expr(n: i32) -> String {
        let mut out = "$".to_string();
        if n < 2 {
            out.push_str(&random_variable())
        } else if n < 4 {
            return random_number()
        } else if n == 4 {
            out.push_str("(#+#)")
        } else if n == 5 {
            out.push_str("(#*#)")
        }
        out
    }

    fn simple_succ(n: i32) -> String {
        match n {
            0 => "S".to_string(),
            1 => "".to_string(),
            2 => "".to_string(),
            3 => "$S".to_string(),
            _ => unreachable!(),
        }
    }

    let mut rng = rand::thread_rng();
    let mut s = simple_expr(rng.gen_range(0..6));

    while s.contains("$") {
        let n2 = rng.gen_range(0..4);
        s = s.replacen("$", &simple_succ(n2), 1);
    }
    while s.contains("#") {
        let n1 = rng.gen_range(0..6);
        s = s.replacen("#", &simple_expr(n1), 1);
        while s.contains("$") {
            let n2 = rng.gen_range(0..4);
            s = s.replacen("$", &simple_succ(n2), 1);
        }
    }

    s
}

// Creating random Formula is very hard because the restrictions on what kind of quantification is valid are more complex than a context free language.
// Although the Backus-Naur Form and procedures for parsing ignore it quantifications can only include variables that exist elsewhere in the Formula and also have
// restrictions on quantification on opposite sides of a logical operation
/* 
pub fn random_simple_formula_str() -> String {
    format!("{}={}",random_expression_str(),random_expression_str())
}

pub fn random_simple_formula() -> Formula {
    Formula::new_simple(&random_simple_formula_str())
}



pub fn random_open_formula_str() -> String {
    fn gen_formula(n: i32) -> String {
        if n == 0 {
            return "[#|#]".to_string()
        } else if n == 1 {
            return "[#&#]".to_string()
        } else if n == 2 {
            return "[#>#]".to_string()
        } else {
            return random_simple_formula_str()
        }
    }

    let mut rng = rand::thread_rng();
    let mut s = "#".to_string();

    while s.contains("#") {
        let n = rng.gen_range(0..10);
        s = s.replacen("#", &gen_formula(n), 1);
    }

    s
}

pub fn random_open_formula() -> Formula {
    Formula::new(&random_open_formula_str())
}



pub fn random_quantification(s: &str) -> String {
    let mut out = s.to_string();
    let mut vs = get_unquant_vars(s);
    let mut rng = rand::thread_rng();
    vs.shuffle(&mut rng);
    for v in vs {
        let n = rng.gen_range(0..6);
        if n < 2 {
            out = format!("E{}:",v) + &out;
        } else if n < 4 {
            out = format!("A{}:",v) + &out;
        }
        if n % 2 == 0 {
            out = "~".to_string() + &out;
        }
    }
    out
}



pub fn random_formula_str() -> String {
    let mut rng = rand::thread_rng();
    let mut s = random_simple_formula_str();
    s = random_quantification(&s);
    let op = rng.gen_range(0..6);
    if op >= 3 {
        return s
    } else if op == 1 {
        return format!("[{}&{}]",s,random_formula_str())
    }
    println!("{}",s);
    s
}

pub fn random_formula() -> Formula {
    Formula::new(&random_formula_str())
}
*/




#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_random_variable() {
        for _ in 0..10 {
            random_variable();
        }
    }

    #[test]
    fn test_random_number() {
        for _ in 0..10 {
            random_number();
        }
    }

    #[test]
    fn test_random_expression() {
        for _ in 0..10 {
            random_expression();
        }
    }
}