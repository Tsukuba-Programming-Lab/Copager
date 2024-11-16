use copager::cfl::CFLTokens;
use copager::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, CFLTokens)]
pub enum Pl0Token {
    // キーワード
    #[default]
    #[token(text = r"const")]
    Const,
    #[token(text = r"var")]
    Var,
    #[token(text = r"procedure")]
    Procedure,
    #[token(text = r"call")]
    Call,
    #[token(text = r"begin")]
    Begin,
    #[token(text = r"end")]
    End,
    #[token(text = r"if")]
    If,
    #[token(text = r"then")]
    Then,
    #[token(text = r"while")]
    While,
    #[token(text = r"do")]
    Do,
    #[token(text = r"odd")]
    Odd,
    #[token(text = r"write")]
    Write,
    #[token(text = r"read")]
    Read,

    // 識別子と数値
    #[token(text = r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[token(text = r"\d+")]
    Number,

    // 演算子と記号
    #[token(text = r"\+")]
    Plus,
    #[token(text = r"-")]
    Minus,
    #[token(text = r"\*")]
    Times,
    #[token(text = r"/")]
    Slash,
    #[token(text = r"=")]
    Eql,
    #[token(text = r"#")]
    Neq,
    #[token(text = r"<=")]
    Leq,
    #[token(text = r"<")]
    Lss,
    #[token(text = r">=")]
    Geq,
    #[token(text = r">")]
    Gtr,
    #[token(text = r"\(")]
    ParenL,
    #[token(text = r"\)")]
    ParenR,
    #[token(text = r",")]
    Comma,
    #[token(text = r"\.")]
    Period,
    #[token(text = r";")]
    Semicolon,
    #[token(text = r":=")]
    Becomes,

    // 空白
    #[token(text = r"[ \t\n\r]+", ignored)]
    _Whitespace,
}
