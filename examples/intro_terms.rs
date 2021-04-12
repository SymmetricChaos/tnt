use tnt::terms::{Expression, Number, Variable};

fn main() {

    println!("The grammar of TNT starts with three simple types: Variables, Numbers, and Expressions. Collectively these are called Terms.");
    println!("Number are the simplest of these, each representing a specific natural number. `0` is a Number and any Number prepended by an `S` is also a number. We may think of `S` as being like the successor function. So `0` = 0, `S0` = 1, `SS0` = 2, and so on.");

    println!("\n\n\nSome random Numbers (geometrically distributed):");
    for _ in 0..10 {
        let n = Number::random();
        print!("{}  ",n)
    }

    println!("\n\n\nNext are Variables, each of which represents \"some natural number\". Each lowercase ASCII letter is a valid Variable and so is any Variable with an apostrophe appended. For some purposes an austere Variable is desirable in this case only `a` is a valid letter to use and Variables are distinguished only by the number of apostrophes that follow. Any representation of a statement of TNT can be rendered in its canonical form by using only austere Variables and using the shortest possible Variables from left to right.");

    println!("\nSome random Variables:");
    for _ in 0..10 {
        let v = Variable::random();
        print!("{}  ",v)
    }

    println!("\n\n\nFinally there are Expressions, each representing a statement of arithmetic in TNT. Any Number or Variable is a valid Expression. Two Expressions seperated by `+` or `*` also form a valid Expression as does an Expression prepended by `S`.");

    println!("\nSome random Expressions:");
    for _ in 0..5 {
        let n = Expression::random();
        print!("{}  ",n)
    }

    println!("The intro continues in: cargo example --into_formulas")

}