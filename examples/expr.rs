use std::io::stdin;

use parsergen::algorithm::LR1;
use parsergen::cfg::*;
use parsergen::Parser;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, TokenSet)]
enum ExprTokenSet {
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
enum ExprSyntax {
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

type ExprParser<'a> = Parser::<'a, LR1<'a, ExprTokenSet, ExprSyntax>>;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    match ExprParser::new()?.parse(&input) {
        Ok(_) => println!("Accepted"),
        Err(e) => println!("Rejected: {}", e),
    };

    Ok(())
}
