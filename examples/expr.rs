use std::io::stdin;

use copager::algorithm::LR1;
use copager::cfg::*;
use copager::error::ParseError;
use copager::Parser;

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

type ExprParser<'a> = Parser::<'a, LR1<'a, ExprTokenSet, ExprSyntax>>;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    match ExprParser::new()?.parse(&input) {
        Ok(sexp) => println!("Accepted : {}", sexp),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<ParseError>() {
                e.pretty_print();
            }
            println!("Rejected : {}", e);
        }
    };

    Ok(())
}
