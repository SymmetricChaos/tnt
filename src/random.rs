use std::str::from_utf8;
use rand::Rng;

use crate::formula::Formula;
use crate::terms::{Term,Variable,Number,Expression};



pub fn random_variable() -> Variable {
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen_range(97..123);
    let mut s = from_utf8(&[n]).unwrap().to_string();
    while rng.gen_range(0.0..1.0) > 0.75 {
        s.push('\'')
    }

    Variable::new(&s)
}

pub fn random_number() -> Number {
    let mut rng = rand::thread_rng();
    let mut s = "".to_string();
    while rng.gen_range(0.0..1.0) > 0.5 {
        s.push('S')
    }
    s.push('0');

    Number::new(&s)
}



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

}