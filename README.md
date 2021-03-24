# tnt
An implementation of Hofstader's "Typographical Number Theory" in Rust. This allows the production of formal proofs that disallow nonsenical construction at compile time are checked for correctness at runtime. Proofs and their constituent formulas have a variety of readable outputs.

Consider the following short proof that 1+1 = 2.

```
use tnt::types::{Term, Variable, Number};
use tnt::deduction::Deduction;
use tnt::axioms::PEANO;

let a = &Variable::new("a");
let b = &Variable::new("b");
let zero = &Number::zero();
let one = &Number::one();

let mut d = Deduction::new("One Plus One Equals Two", PEANO.clone());
d.add_axiom(PEANO[2].clone(), "");
d.specification(0, a, one, "");
d.specification(1, b, zero, "");
d.add_axiom(PEANO[1].clone(), "");
d.specification(3, a, one, "");
d.successor(4, "");
d.transitivity(2,5,"");
```

A simple ASCII representation:

```
d.pretty_print()
0) Aa:Ab:(a+Sb)=S(a+b)
1) Ab:(S0+Sb)=S(S0+b)
2) (S0+S0)=S(S0+0)   
3) Aa:(a+0)=a        
4) (S0+0)=S0
5) S(S0+0)=SS0       
6) (S0+S0)=SS0
```

Formatted for LaTeX:

```
\documentclass[fleqn,11pt]{article}
\usepackage{amsmath}
\allowdisplaybreaks
\begin{document}
\section*{One Plus One Equals Two}
\begin{flalign*}
&\hspace{0em}0)\hspace{1em}\forall a:\forall b:(a+Sb)=S(a+b)\\
&\hspace{0em}1)\hspace{1em}\forall b:(S0+Sb)=S(S0+b)\\
&\hspace{0em}2)\hspace{1em}(S0+S0)=S(S0+0)\\
&\hspace{0em}3)\hspace{1em}\forall a:(a+0)=a\\
&\hspace{0em}4)\hspace{1em}(S0+0)=S0\\
&\hspace{0em}5)\hspace{1em}S(S0+0)=SS0\\
&\hspace{0em}6)\hspace{1em}(S0+S0)=SS0\\
\end{flalign*}
```

Which produces:

![one and one is two](https://github.com/SymmetricChaos/tnt/blob/master/examples/addition_snip.PNG?raw=true)