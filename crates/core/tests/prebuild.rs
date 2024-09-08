use serde::{Serialize, Deserialize};
use serde_cbor::ser::to_vec_packed;
use serde_cbor::de::from_slice;

use copager_core::{Grammar, Processor};
use copager_cfg::token::TokenTag;
use copager_cfg::rule::{RuleTag, Rule, RuleElem};
use copager_lex::LexSource;
use copager_lex_regex::RegexLexer;
use copager_parse::ParseSource;
use copager_parse_lr1::LR1;
use copager_ir_void::Void;

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    LexSource, Serialize, Deserialize
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
    ParseSource, Serialize, Deserialize
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

type MyGrammar = Grammar<ExprToken, ExprRule>;
type MyLexer = RegexLexer<ExprToken>;
type MyParser = LR1<ExprToken, ExprRule>;
type MyProcessor = Processor<MyGrammar, MyLexer, MyParser>;

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
        .build_parser_by_cache()
        .process::<Void>("1 + 2 * 3")?;

    Ok(())
}
