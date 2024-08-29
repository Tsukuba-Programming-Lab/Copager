// use copager::cfg::*;

// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, TokenSet)]
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

// #[derive(Debug, Clone, Copy, Syntax)]
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

// #[test]
// fn check_compile() {
//     let _ = TestTokenSet::into_regex(&self::TestTokenSet::Plus);
//     let _ = TestSyntax::into_rules(&self::TestSyntax::Expr);
// }
