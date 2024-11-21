use copager_core::{Generator, Processor};
use copager_cfl::token::TokenTag;
use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::{CFL, CFLTokens, CFLRules};
use copager_lex_regex::RegexLexer;
use copager_parse_lr_slr1::SLR1;
use copager_ir_void::Void;

#[derive(Debug, Default, CFL)]
struct TestLang (
    #[tokens] TestToken,
    #[rules] TestRule,
);

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLTokens)]
enum TestToken {
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
    #[token(r"[ \t\n]+", ignored)]
    _Whitespace,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLRules)]
enum TestRule {
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

type TestGenerator<T> = Generator<T, RegexLexer<T>, SLR1<T>>;
type TestProcessor = Processor<TestGenerator<TestLang>>;

#[test]
fn simple_success() {
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

    let processor = TestProcessor::new().build().unwrap();
    for input in &OK_INPUTS {
        println!("input: {}", input);
        processor.process::<Void>(input).unwrap();
    }
}

#[test]
fn simple_failure() {
    const ERR_INPUTS: [&str; 7] = [
        "()",
        "(10 -",
        "10 +",
        "*",
        "10 20 + 30",
        "10 + 20 * 30 / 40 (",
        "(((10))",
    ];

    let processor = TestProcessor::new().build().unwrap();
    for input in &ERR_INPUTS {
        assert!(processor.process::<Void>(input).is_err(), "input: {}", input);
    }
}
