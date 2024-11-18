use serde::{Serialize, Deserialize};

use copager_core::{Generator, Processor};
use copager_cfl::token::TokenTag;
use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::{CFL, CFLTokens, CFLRules};
use copager_lex_regex::RegexLexer;
use copager_parse_lr_lr1::LR1;
use copager_ir_void::Void;

#[derive(Default, CFL, Serialize, Deserialize)]
struct ExprLang (
    #[tokens] ExprToken,
    #[rules]  ExprRule
);

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    CFLTokens, Serialize, Deserialize
)]
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

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    CFLRules, Serialize, Deserialize
)]
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

type MyGenerator<T> = Generator<T, RegexLexer<T>, LR1<T>>;
type MyProcessor = Processor<MyGenerator<ExprLang>>;

#[test]
fn simple_success() -> anyhow::Result<()> {
    MyProcessor::new()
        .build()?
        .process::<Void>("1 + 2 * 3")?;

    Ok(())
}
