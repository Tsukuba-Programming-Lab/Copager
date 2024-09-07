use std::io::stdin;

use copager::lex::{LexSource, RegexLexer};
use copager::parse::{ParseSource, LR1};
use copager::ir::SExp;
use copager::prelude::*;
use copager::{Grammar, Processor};

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

type MyGrammar = Grammar<ExprToken, ExprRule>;
type MyLexer = RegexLexer<ExprToken>;
type MyParser = LR1<ExprToken, ExprRule>;
type MyProcessor = Processor<MyGrammar, MyLexer, MyParser>;

fn eval(ir: &SExp<'_, ExprToken, ExprRule>) -> i32 {
    macro_rules! match_atom {
        ($term:expr, $($kind:pat => $block:expr),* $(,)?) => {
            match $term {
                SExp::Atom(token) => {
                    match token.kind {
                        $($kind => $block,)*
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    match ir {
        SExp::List { rule, elems } => {
            match rule {
                ExprRule::Expr if elems.len() == 1 => eval(&elems[0]),
                ExprRule::Expr => {
                    let lhs = eval(&elems[0]);
                    let rhs = eval(&elems[2]);
                    match_atom!(elems[1],
                        ExprToken::Plus => lhs + rhs,
                        ExprToken::Minus => lhs - rhs,
                    )
                }
                ExprRule::Term if elems.len() == 1 => eval(&elems[0]),
                ExprRule::Term => {
                    let lhs = eval(&elems[0]);
                    let rhs = eval(&elems[2]);
                    match_atom!(elems[1],
                        ExprToken::Mul => lhs * rhs,
                        ExprToken::Div => lhs / rhs,
                    )
                }
                ExprRule::Num if elems.len() == 1 => eval(&elems[0]),
                ExprRule::Num => eval(&elems[1]),

            }
        }
        SExp::Atom(token) => token.as_str().parse().unwrap(),
    }
}

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let sexp = MyProcessor::new()
        .build_lexer()?
        .build_parser()?
        .process::<SExp<_, _>>(&input)?;
    println!("{} = {}", input.trim(), eval(&sexp));

    Ok(())
}
