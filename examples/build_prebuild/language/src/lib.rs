use serde::{Deserialize, Serialize};

use copager::cfl::{CFL, CFLRules, CFLTokens};
use copager::template::LALR1;
use copager::prelude::*;

pub type Arithmetic = LALR1<ArithmeticLang>;

#[derive(Debug, Default, Clone, CFL, Serialize, Deserialize)]
pub struct ArithmeticLang (
    #[tokens] ArithmeticToken,
    #[rules] ArithmeticRule,
);

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    CFLTokens, Serialize, Deserialize,
)]
pub enum ArithmeticToken {
    #[default]
    #[token(r"\+")]
    Plus,
    #[token(r"-")]
    Minus,
    #[token(r"\*")]
    Mul,
    #[token(r"/")]
    Div,
    #[token(r"\(", ir_omit)]
    BracketL,
    #[token(r"\)", ir_omit)]
    BracketR,
    #[token(r"[1-9][0-9]*")]
    Num,
    #[token(r"[ \t\n]+", trivia)]
    _Whitespace,
}

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    CFLRules, Serialize, Deserialize,
)]
pub enum ArithmeticRule {
    #[default]
    #[rule("<expr> ::= <expr> Plus <term>")]
    #[rule("<expr> ::= <expr> Minus <term>")]
    #[rule("<expr> ::= <term>")]
    Arithmetic,
    #[rule("<term> ::= <term> Mul <num>")]
    #[rule("<term> ::= <term> Div <num>")]
    #[rule("<term> ::= <num>")]
    Term,
    #[rule("<num> ::= BracketL <expr> BracketR")]
    #[rule("<num> ::= Num")]
    Num,
}
