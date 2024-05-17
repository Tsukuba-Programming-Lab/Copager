use serde::{Serialize, Deserialize};

use parsergen::algorithm::LR1;
use parsergen::cfg::*;
use parsergen::Parser;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, TokenSet)]
enum TestTokenSet {
    #[token(regex = r"\+")]
    Plus,
    #[token(regex = r"-")]
    Minus,
    #[token(regex = r"\*")]
    Mul,
    #[token(regex = r"/")]
    Div,
    #[token(regex = r"\(")]
    BracketL,
    #[token(regex = r"\)")]
    BracketR,
    #[token(regex = r"[1-9][0-9]*")]
    Num,
    #[token(regex = r"[ \t\n]+", ignored)]
    _Whitespace,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Syntax)]
enum TestSyntax {
    #[rule("<expr> ::= <expr> Plus <term>")]
    ExprPlus,
    #[rule("<expr> ::= <expr> Minus <term>")]
    ExprMinus,
    #[rule("<expr> ::= <term>")]
    ExprTerm,
    #[rule("<term> ::= <term> Mul <num>")]
    TermMul,
    #[rule("<term> ::= <term> Div <num>")]
    TermDiv,
    #[rule("<term> ::= <num>")]
    TermNum,
    #[rule("<num> ::= BracketL <expr> BracketR")]
    NestedNum,
    #[rule("<num> ::= Num")]
    Num,
}

#[test]
fn serde() {
    type TestParser<'a> = Parser::<'a, LR1<'a, TestTokenSet, TestSyntax>>;

    let parser = TestParser::new().unwrap();
    let serialized = serde_json::to_string(&parser).unwrap();
    let deserialized: TestParser = serde_json::from_str(&serialized).unwrap();

    deserialized.parse("10 * (20 - 30)").unwrap();
}
