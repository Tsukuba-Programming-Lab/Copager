use core::cfg::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, TokenSet)]
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

#[derive(Debug, Clone, Copy, Syntax)]
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
fn check_compile() {
    let _ = TestTokenSet::to_regex(&self::TestTokenSet::Plus);
    let _ = TestSyntax::to_rule(&self::TestSyntax::ExprPlus);
}
