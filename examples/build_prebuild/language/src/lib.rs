use serde::{Deserialize, Serialize};

use copager::cfl::{CFLRules, CFLTokens};
use copager::lex::RegexLexer;
use copager::parse::LR1;
use copager::prelude::*;
use copager::{Language, Processor};

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    CFLTokens, Serialize, Deserialize,
)]
pub enum ExprToken {
    #[default]
    #[token(text = r"\+")]
    Plus,
    #[token(text = r"-")]
    Minus,
    #[token(text = r"\*")]
    Mul,
    #[token(text = r"/")]
    Div,
    #[token(text = r"\(")]
    BracketL,
    #[token(text = r"\)")]
    BracketR,
    #[token(text = r"[1-9][0-9]*")]
    Num,
    #[token(text = r"[ \t\n]+", ignored)]
    _Whitespace,
}

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    CFLRules, Serialize, Deserialize,
)]
pub enum ExprRule {
    #[default]
    #[rule("<expr> ::= <expr> Plus <term>")]
    #[rule("<expr> ::= <expr> Minus <term>")]
    #[rule("<expr> ::= <term>")]
    Expr,
    #[rule("<term> ::= <term> Mul <num>")]
    #[rule("<term> ::= <term> Div <num>")]
    #[rule("<term> ::= <num>")]
    Term,
    #[rule("<num> ::= BracketL <expr> BracketR")]
    #[rule("<num> ::= Num")]
    Num,
}

pub type MyLanguage = Language<ExprToken, ExprRule>;
pub type MyLexer = RegexLexer<ExprToken>;
pub type MyParser = LR1<ExprToken, ExprRule>;
pub type MyProcessor = Processor<MyLanguage, MyLexer, MyParser>;
