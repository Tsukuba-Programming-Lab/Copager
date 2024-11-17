use serde::{Deserialize, Serialize};

use copager::cfl::{CFL, CFLRules, CFLTokens};
use copager::lex::RegexLexer;
use copager::parse::LR1;
use copager::prelude::*;
use copager::Generator;

type Configure<T> = Generator<T, RegexLexer<T>, LR1<T>>;
pub type Arithmetic = Configure<ArithmeticLang>;

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
