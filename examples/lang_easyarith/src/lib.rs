use copager::lang::{Lang, RuleSet, TokenSet};
use copager::prelude::*;

#[derive(Lang)]
pub struct EasyArith (
    #[tokenset] EAToken,
    #[ruleset]  EARule,
);

#[derive(Debug, Clone, PartialEq, Eq, Hash, TokenSet)]
pub enum EAToken {
    // 予約語
    #[token(r"var")]
    Var,
    #[token(r"print")]
    Print,

    // 記号
    #[token(r"\+")]
    Plus,
    #[token(r"\*")]
    Mul,
    #[token(r"=")]
    Eql,
    #[token(r"\(")]
    LPar,
    #[token(r"\)")]
    RPar,
    #[token(r";")]
    Semi,

    // 値
    #[token(r"0b[01]+")]
    #[token(r"0[0-7]+")]
    #[token(r"0x[0-9a-fA-F]+")]
    #[token(r"[0-9]+")]
    Num,
    #[token(r"[a-zA-Z]+")]
    Id,

    // 空白文字
    #[token(r"[ \t\n]+", trivia)]
    _Whitespace,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, RuleSet)]
pub enum EARule {
    // 使用する字句集合の指定
    #[tokenset(EAToken)]

    // トップレベル要素
    #[rule("<top> ::= <top> <stmt>")]
    #[rule("<top> ::= <stmt>")]
    Top,

    // 文
    #[rule("<stmt> ::= <decl>")]
    #[rule("<stmt> ::= <assign>")]
    #[rule("<stmt> ::= <print>")]
    Stmt,
    #[rule("<decl> ::= Var Id Semi")]
    Decl,
    #[rule("<assign> ::= Id Eql <expr> Semi")]
    Assign,
    #[rule("<print> ::= Print <expr> Semi")]
    Print,

    // 式
    #[rule("<expr> ::= <expr> Plus <term>")]
    #[rule("<expr> ::= <term>")]
    Expr,
    #[rule("<term> ::= <term> Mul <fact>")]
    #[rule("<term> ::= <fact>")]
    Term,
    #[rule("<fact> ::= Num")]
    #[rule("<fact> ::= Id")]
    #[rule("<fact> ::= LPar <expr> RPar")]
    Fact,
}
