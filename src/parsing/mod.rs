//! Parse strings of TNT and build an Abstract Syntax Tree

/*
Backus-Naur Form for the TNT Language

<num> ::= { "0" | "S" <num> }
<var> ::= { <lowercase_letter> | <var> "'" }
<arith_op> ::= { "+" | "*" }
<expr> ::= { num | var | "(" <expr> <arith_op> <expr> ")" | "S" expr }
<quant> ::= { "A" <var> ":" | "E" <var> ":" | "~" <quant> }
<logical_op> ::= { "&" | "|" | ">" }
<formula> ::= {  <expr> "=" <expr> | "[" <formula> <logical_op> <formula> "]" | <quant> <formula> }
*/

pub mod syntax_tree;
pub mod parser;
