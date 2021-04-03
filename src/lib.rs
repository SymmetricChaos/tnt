pub mod deduction;
pub mod types;
mod properties;
pub mod operations;
mod string_manip;
mod translate;
pub mod axioms;
pub mod logic_errors;

/*
Bakus-Naur Form for the TNT Language

<num> ::= { "0" | "S" <num> }
<var> ::= { <lowercase_letter> | <var> "'" }
<arith_op> ::= { "+" | "*" }
<expr> ::= { num | var | "(" <expr> <arith_op> <expr> ")" | "S" expr }
<quant> ::= { "A" <var> ":" | "E" <var> ":" | "~" <quant> }
<logical_op> ::= { "&" | "|" | ">" }
<formula> ::= {  <expr> "=" <expr> |  "[" <formula> <logical_op> <formula> "]" | <quant> <formula> }
*/