use copager_core::{Generator, Processor};
use copager_cfl::token::TokenTag;
use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::{CFL, CFLTokens, CFLRules};
use copager_lex_regex::RegexLexer;
use copager_parse_lr_lr0::LR0;
use copager_ir_void::Void;

#[derive(Debug, Default, CFL)]
struct TestLang (
    #[tokens] TestToken,
    #[rules] TestRule,
);

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLTokens)]
enum TestToken {
    #[default]
    #[token(text = r"\+")]
    Plus,
    #[token(text = r"-")]
    Minus,
    #[token(text = r"\(")]
    BracketL,
    #[token(text = r"\)")]
    BracketR,
    #[token(text = r"[1-9][0-9]*")]
    Num,
    #[token(text = r"[ \t\n]+", ignored)]
    _Whitespace,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLRules)]
enum TestRule {
    #[default]
    #[rule("<expr> ::= <expr> Plus <num>")]
    #[rule("<expr> ::= <expr> Minus <num>")]
    #[rule("<expr> ::= <num>")]
    Expr,
    #[rule("<num> ::= BracketL <expr> BracketR")]
    #[rule("<num> ::= Num")]
    Num,
}

type TestGenerator<T> = Generator<T, RegexLexer<T>, LR0<T>>;
type TestProcessor = Processor<TestGenerator<TestLang>>;

#[test]
fn simple_success() {
    const OK_INPUTS: [&str; 8] = [
        "10",
        "10 + 20",
        "10 - 20",
        "10 + 20 + 30",
        "(10)",
        "((((10))))",
        "10 + (20 - 30)",
        "(10 + 20) - 30",
    ];

    let processor = TestProcessor::new()
        .build_lexer()
        .unwrap()
        .build_parser()
        .unwrap();

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
        "+",
        "10 20 + 30",
        "10 + 20 - 30 (",
        "(((10))",
    ];

    let processor = TestProcessor::new()
        .build_lexer()
        .unwrap()
        .build_parser()
        .unwrap();

    for input in &ERR_INPUTS {
        assert!(processor.process::<Void>(input).is_err(), "input: {}", input);
    }
}
