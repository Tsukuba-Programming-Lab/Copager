use serde::{Serialize, Deserialize};

use copager_core::{Generator, Processor};
use copager_cfl::token::TokenTag;
use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::{CFL, CFLToken, CFLRule};
use copager_lex_regex::RegexLexer;
use copager_parse_lr_lr1::LR1;
use copager_ir_void::Void;

#[derive(CFL, Serialize, Deserialize)]
struct ExprLang (
    #[tokenset] ExprToken,
    #[ruleset]  ExprRule,
);

#[derive(Clone, Hash, PartialEq, Eq, CFLToken, Serialize, Deserialize)]
enum ExprToken {
    #[token(r"\+")]
    Plus,
    #[token(r"-")]
    Minus,
    #[token(r"\*")]
    Mul,
    #[token(r"/")]
    Div,
    #[token(r"\(")]
    BracketL,
    #[token(r"\)")]
    BracketR,
    #[token(r"[1-9][0-9]*")]
    Num,
    #[token(r"[ \t\n]+", trivia)]
    _Whitespace,
}

#[derive(Clone, Hash, PartialEq, Eq, CFLRule, Serialize, Deserialize)]
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

type MyGenerator<T> = Generator<T, RegexLexer<T>, LR1<T>>;
type MyProcessor = Processor<MyGenerator<ExprLang>>;

#[test]
fn simple_success() -> anyhow::Result<()> {
    MyProcessor::new()
        .build()?
        .process::<Void>("1 + 2 * 3")?;

    Ok(())
}
