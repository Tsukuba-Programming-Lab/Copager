// use serde::{Serialize, Deserialize};

// use copager::algorithm::LR1;
// use copager::cfg::*;
// use copager::Parser;

// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, TokenSet)]
// enum TestTokenSet {
//     #[token(regex = r"\+")]
//     Plus,
//     #[token(regex = r"-")]
//     Minus,
//     #[token(regex = r"\*")]
//     Mul,
//     #[token(regex = r"/")]
//     Div,
//     #[token(regex = r"\(")]
//     BracketL,
//     #[token(regex = r"\)")]
//     BracketR,
//     #[token(regex = r"[1-9][0-9]*")]
//     Num,
//     #[token(regex = r"[ \t\n]+", ignored)]
//     _Whitespace,
// }

// #[derive(Debug, Clone, Copy, Serialize, Deserialize, Syntax)]
// enum TestSyntax {
//     #[rule("<expr> ::= <expr> Plus <term>")]
//     #[rule("<expr> ::= <expr> Minus <term>")]
//     #[rule("<expr> ::= <term>")]
//     Expr,
//     #[rule("<term> ::= <term> Mul <num>")]
//     #[rule("<term> ::= <term> Div <num>")]
//     #[rule("<term> ::= <num>")]
//     Term,
//     #[rule("<num> ::= BracketL <expr> BracketR")]
//     #[rule("<num> ::= Num")]
//     Num,
// }

// type TestParser<'a> = Parser::<'a, LR1<'a, TestTokenSet, TestSyntax>>;

// #[test]
// fn check_serde() {
//     // build.rs
//     let parser = TestParser::new().unwrap();
//     let serialized = serde_json::to_string(&parser).unwrap();

//     // main.rs
//     let deserialized: TestParser = serde_json::from_str(&serialized).unwrap();
//     deserialized.parse("10 * (20 - 30)").unwrap();
// }
