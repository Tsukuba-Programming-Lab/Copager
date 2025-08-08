use std::io::{stdin, stdout, Write};

use copager::cfl::{CFL, CFLRule, CFLToken};
use copager::ir::SExp;
use copager::template::LALR1;
use copager::prelude::*;
use copager::Processor;

#[derive(Debug, CFL)]
struct ExprLang (
    #[tokenset] ExprToken,
    #[ruleset] ExprRule,
);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, CFLToken)]
enum ExprToken {
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, CFLRule)]
enum ExprRule {
    #[tokenset(ExprToken)]
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

fn main() -> anyhow::Result<()> {
    println!("Example <one-shot>");
    print!("Input: ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let sexp = Processor::<LALR1<ExprLang>>::new()
        .build()?
        .process::<SExp<_>>(&input)?;
    println!("Success: {}", sexp);

    Ok(())
}
