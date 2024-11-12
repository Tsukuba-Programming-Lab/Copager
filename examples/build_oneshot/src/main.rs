use std::io::{stdin, stdout, Write};

use copager::lex::{LexSource, RegexLexer};
use copager::parse::{ParseSource, LR1};
use copager::ir::SExp;
use copager::prelude::*;
use copager::{Language, Processor};

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, LexSource)]
enum ExprToken {
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

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, ParseSource)]
enum ExprRule {
    #[default]
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

type MyLanguage = Language<ExprToken, ExprRule>;
type MyLexer = RegexLexer<ExprToken>;
type MyParser = LR1<ExprToken, ExprRule>;
type MyProcessor = Processor<MyLanguage, MyLexer, MyParser>;

fn main() -> anyhow::Result<()> {
    println!("Example <one-shot>");
    print!("Input: ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let sexp = MyProcessor::new()
        .build_lexer()?
        .build_parser()?
        .process::<SExp<_, _>>(&input)?;
    println!("Success: {}", sexp);

    Ok(())
}
