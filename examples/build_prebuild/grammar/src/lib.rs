use serde::{Deserialize, Serialize};

use copager::lex::{LexSource, RegexLexer};
use copager::parse::{ParseSource, LR1};
use copager::prelude::*;
use copager::{Grammar, Processor};

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    LexSource, Serialize, Deserialize,
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
    ParseSource, Serialize, Deserialize,
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

pub type MyGrammar = Grammar<ExprToken, ExprRule>;
pub type MyLexer = RegexLexer<ExprToken>;
pub type MyParser = LR1<ExprToken, ExprRule>;
pub type MyProcessor = Processor<MyGrammar, MyLexer, MyParser>;
