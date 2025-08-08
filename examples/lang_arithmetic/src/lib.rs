use copager::lang::{Lang, RuleSet, TokenSet};
use copager::template::LALR1;
use copager::prelude::*;

pub type Arithmetic = LALR1<ArithmeticLang>;

#[derive(Lang)]
pub struct ArithmeticLang (
    #[tokenset] ArithmeticToken,
    #[ruleset]  ArithmeticRule,
);

#[derive(Debug, Clone, Hash, PartialEq, Eq, TokenSet)]
pub enum ArithmeticToken {
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, RuleSet)]
pub enum ArithmeticRule {
    #[tokenset(ArithmeticToken)]
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
