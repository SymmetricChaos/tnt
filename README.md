# tnt
An implementation of Hofstader's "Typographical Number Theory" from the book Gödel, Escher, and Bach. This crate allows the creation of simple proofs of number theory in propositional logic. It is not a proof assistant, it cannot create a proof or disproof of a given formula, rather the Deduction struct enforces the rules of inference at run time to prevent an invalid proof from being created. Certain nonsensical constructions are also caught at compile time.

A brief explanation is available in [the primer](https://github.com/SymmetricChaos/tnt/blob/master/primer.pdf).

Once a Deduction is created it can be output in a variety of ways.

Consider the following short proof that 1 + 1 = 2.

```
use tnt::{Term,Deduction};

let a = &Term::var("a");
let b = &Term::var("b");
let zero = &Term::zero();
let one = &Term::one();

let mut d = Deduction::new("One Plus One Equals Two");
d.add_axiom(2)?; // Aa:Ab:(a+Sb)=S(a+b)
d.specification(0, a, one)?;
d.specification(1, b, zero)?;
d.add_axiom(1)?; //"Aa:(a+0)=a
d.specification(3, a, one)?;
d.successor(4)?;
d.transitivity(2, 5)?;
```

We can print this in several ways.

```
println!("{}", d);
// 0) Aa:Ab:(a+Sb)=S(a+b)
// 1) Ab:(S0+Sb)=S(S0+b)
// 2) (S0+S0)=S(S0+0)   
// 3) Aa:(a+0)=a        
// 4) (S0+0)=S0
// 5) S(S0+0)=SS0       
// 6) (S0+S0)=SS0

println!("{}", d.pretty_string());
// 0) ∀a:∀b:(a + Sb) = S(a+b)
// 1) ∀b:(S0 + Sb) = S(S0+b) 
// 2) (S0 + S0) = S(S0+0)    
// 3) ∀a:(a + 0) = a
// 4) (S0 + 0) = S0
// 5) S(S0+0) = SS0
// 6) (S0 + S0) = SS0 

println!("{}", d.english_annotated());
// 0) for all a, for all b, (a + Sb) = S(a+b) [axiom]
// 1) for all b, (S0 + Sb) = S(S0+b) [specification of a to S0 in theorem 0]
// 2) (S0 + S0) = S(S0+0) [specification of b to 0 in theorem 1]     
// 3) for all a, (a + 0) = a [axiom]
// 4) (S0 + 0) = S0 [specification of a to S0 in theorem 3]
// 5) S(S0+0) = SS0 [successor of theorem 4]
// 6) (S0 + S0) = SS0 [transitivity of theorem 2 and theorem 5] 
```

Using .latex("addition") we get the file addition.tex which renders as:

![one and one is two](https://github.com/SymmetricChaos/tnt/blob/master/examples/addition_snip.PNG?raw=true)


The Deduction can also be crudely translated to English with automatic annotations using the .english() method.

Finally a Deduction may be rendered as an (extremely large) integer by using the .arithmetize() method which reads each theorem as bytes seperated by spaces into a BigUint. This is of no practical use the author is aware of but is relevant to the production of Gödel statements.

The previous theorem corresponds to the number: 1050341303275422378657768361784977847949672579265786753539438511912991722480303393035798790127074435266152493569318923916943104808524883160525578709392832871799037992734723295688338121067685795026664762465244602602804284806503763532291647776