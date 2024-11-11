use copager_core::{Grammar, Processor};
use copager_cfg::token::TokenTag;
use copager_cfg::rule::{RuleTag, Rule, RuleElem};
use copager_lex::LexSource;
use copager_lex_regex::RegexLexer;
use copager_parse::ParseSource;
use copager_parse_lr_lr0::LR0;
use copager_ir_void::Void;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, LexSource)]
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

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, ParseSource)]
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

type TestGrammar = Grammar<TestToken, TestRule>;
type TestLexer = RegexLexer<TestToken>;
type TestParser = LR0<TestToken, TestRule>;
type TestProcessor = Processor<TestGrammar, TestLexer, TestParser>;

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
        assert!(processor.process::<Void>(input).is_ok(), "input: {}", input);
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
