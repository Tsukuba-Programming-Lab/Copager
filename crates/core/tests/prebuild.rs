use serde::{Serialize, Deserialize};
use serde_cbor::ser::to_vec_packed;
use serde_cbor::de::from_slice;

use copager_core::{Generator, Processor};
use copager_cfl::token::TokenTag;
use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::{CFL, CFLToken, CFLRule};
use copager_lex_regex::RegexLexer;
use copager_parse_lr_lr1::LR1;
use copager_ir_void::Void;

#[derive(Default, Clone, CFL, Serialize, Deserialize)]
struct ExprLang (
    #[tokenset] ExprToken,
    #[ruleset]  ExprRule,
);

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    CFLToken, Serialize, Deserialize
)]
enum ExprToken {
    #[default]
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

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    CFLRule, Serialize, Deserialize
)]
enum ExprRule {
    #[tokenset(ExprToken)]
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
fn prebuild() -> anyhow::Result<()> {
    // in build.rs
    let prebuiled_processor = build_rs()?;
    let serialized = to_vec_packed(&prebuiled_processor)?;

    // in main.rs
    let deserialized: MyProcessor = from_slice(&serialized)?;
    main_rs(deserialized)?;

    Ok(())
}

fn build_rs() -> anyhow::Result<MyProcessor> {
    MyProcessor::new().prebuild_parser()
}

fn main_rs(processor: MyProcessor) -> anyhow::Result<()> {
    processor
        .build_lexer()?
        .restore_parser_by_cache()
        .process::<Void>("1 + 2 * 3")?;

    Ok(())
}
