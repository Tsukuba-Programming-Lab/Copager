use serde::{Serialize, Deserialize};

use copager_core::{Generator, Processor};
use copager_cfl::token::TokenTag;
use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::{CFL, CFLToken, CFLRule};
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

const OK_INPUTS: [&str; 7] = [
    "1 + 2",
    "1 + 2 * 3",
    "1 + 2 * 3 / 4",
    "1 + 2 * (3 / 4)",
    "1 + 2 * (3 / 4) - 5",
    "1 + 2 * (3 / 4) - 5 * 6",
    "(1 + 2) * ((3 / 4) - 5 * 6 / 7)",
];

const ERR_INPUTS: [&str; 7] = [
    "1 +",
    "1 + 2 *",
    "1 + 2 * 3 /",
    "1 + 2 * (3 /",
    "1 + 2 * (3 / 4",
    "1 + 2 * (3 / 4) -",
    "(1 + 2) * ((3 / 4) - 5 * 6 /",
];

#[test]
fn simple_multiple_only_success() {
    let processor = gen_processor();
    for input in OK_INPUTS {
        assert!(processor.process::<Void>(input).is_ok());
    }
}

#[test]
fn simple_multiple_only_failure() {
    let processor = gen_processor();
    for input in ERR_INPUTS {
        assert!(processor.process::<Void>(input).is_err());
    }
}

#[test]
fn simple_multiple_mix_success_and_failure() {
    let mixed_testcases = OK_INPUTS
        .iter()
        .zip(ERR_INPUTS.iter())
        .flat_map(|(ok, err)| vec![(true, ok), (false, err)]);

    let processor = gen_processor();
    for (is_ok, input) in mixed_testcases {
        if is_ok {
            assert!(processor.process::<Void>(input).is_ok());
        } else {
            assert!(processor.process::<Void>(input).is_err());
        }
    }
}

fn gen_processor() -> MyProcessor {
    MyProcessor::new()
        .build()
        .unwrap()
}
