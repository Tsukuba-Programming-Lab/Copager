use serde::{Serialize, Deserialize};

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{RuleTag, Rule, RuleElem};
use copager_lex::{LexSource, LexDriver};
use copager_lex_regex::RegexLexer;
use copager_parse::{ParseSource, ParseDriver, ParseEvent};
use copager_parse_lr1::LR1;

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

type MyLexer = RegexLexer<ExprToken>;
type MyParser = LR1<ExprToken, ExprRule>;

const OK_INPUTS: [&str; 10] = [
    "10",
    "10 + 20",
    "10 - 20",
    "10 * 20",
    "10 / 20",
    "10 + 20 * 30 - 40",
    "(10)",
    "((((10))))",
    "10 * (20 - 30)",
    "((10 + 20) * (30 / 40)) - 50",
];

const ERR_INPUTS: [&str; 7] = [
    "()",
    "(10 -",
    "10 +",
    "*",
    "10 20 + 30",
    "10 + 20 * 30 / 40 (",
    "(((10))",
];

#[test]
fn simple_success() {
    for input in &OK_INPUTS {
        assert!(parse(input), "{}", input);
    }
}

#[test]
fn simple_failure() {
    for input in &ERR_INPUTS {
        assert!(!parse(input), "{}", input);
    }
}

fn parse<'input>(input: &'input str) -> bool {
    let source = ExprToken::default();
    let lexer = <MyLexer as LexDriver<ExprToken>>::try_from(source).unwrap();

    let source = (ExprToken::default(), ExprRule::default());
    let parser = <MyParser as ParseDriver<ExprToken, ExprRule>>::try_from(source).unwrap();

    let mut parse_itr = parser.run(lexer.run(input));
    let is_err = |state| matches!(state, ParseEvent::Err(_));
    let err_happened = parse_itr.any(is_err);

    !err_happened
}
