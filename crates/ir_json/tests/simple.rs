use copager_core::{Generator, Processor};
use copager_cfl::token::TokenTag;
use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::{CFL, CFLTokens, CFLRules};
use copager_lex_regex::RegexLexer;
use copager_parse_lr_lr1::LR1;
use copager_ir_json::{Json, JsonLite};

#[derive(Debug, Default, CFL)]
struct TestLang (
    #[tokens] TestToken,
    #[rules]  TestRule
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
    #[token(r"\(", ir_omit)]
    BracketL,
    #[token(r"\)", ir_omit)]
    BracketR,
    #[token(r"[1-9][0-9]*")]
    Num,
    #[token(r"[ \t\n]+", trivia)]
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

#[test]
fn normal_display() {
    let parse = |input| {
        type TestGenerator<T> = Generator<T, RegexLexer<T>, LR1<T>>;
        type TestProcessor = Processor<TestGenerator<TestLang>>;
        TestProcessor::new()
            .build()?
            .process::<Json<_>>(input)
    };

    let ir = parse("1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"{"kind": "list", "tag": "Expr", "elements": [{"kind": "list", "tag": "Term", "elements": [{"kind": "list", "tag": "Num", "elements": [{"kind": "token", "tag": "Num", "str": "1"}]}]}]}"#);

    let ir = parse("1 + 1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"{"kind": "list", "tag": "Expr", "elements": [{"kind": "list", "tag": "Expr", "elements": [{"kind": "list", "tag": "Term", "elements": [{"kind": "list", "tag": "Num", "elements": [{"kind": "token", "tag": "Num", "str": "1"}]}]}]}, {"kind": "token", "tag": "Plus", "str": "+"}, {"kind": "list", "tag": "Term", "elements": [{"kind": "list", "tag": "Num", "elements": [{"kind": "token", "tag": "Num", "str": "1"}]}]}]}"#);

    let ir = parse("(1 + 1) * 1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"{"kind": "list", "tag": "Expr", "elements": [{"kind": "list", "tag": "Term", "elements": [{"kind": "list", "tag": "Term", "elements": [{"kind": "list", "tag": "Num", "elements": [{"kind": "list", "tag": "Expr", "elements": [{"kind": "list", "tag": "Expr", "elements": [{"kind": "list", "tag": "Term", "elements": [{"kind": "list", "tag": "Num", "elements": [{"kind": "token", "tag": "Num", "str": "1"}]}]}]}, {"kind": "token", "tag": "Plus", "str": "+"}, {"kind": "list", "tag": "Term", "elements": [{"kind": "list", "tag": "Num", "elements": [{"kind": "token", "tag": "Num", "str": "1"}]}]}]}]}]}, {"kind": "token", "tag": "Mul", "str": "*"}, {"kind": "list", "tag": "Num", "elements": [{"kind": "token", "tag": "Num", "str": "1"}]}]}]}"#);
}

#[test]
fn lite_display() {
    type TestGenerator<T> = Generator<T, RegexLexer<T>, LR1<T>>;
    type TestProcessor = Processor<TestGenerator<TestLang>>;

    let parse = |input| {TestProcessor::new()
        .build()?
        .process::<JsonLite<_>>(input)
    };

    let ir = parse("1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"{"tag": "Expr", "elements": [{"tag": "Term", "elements": [{"tag": "Num", "elements": ["1"]}]}]}"#);

    let ir = parse("1 + 1");
    assert!(ir.is_ok());

    assert_eq!(ir.unwrap().to_string(), r#"{"tag": "Expr", "elements": [{"tag": "Expr", "elements": [{"tag": "Term", "elements": [{"tag": "Num", "elements": ["1"]}]}]}, "+", {"tag": "Term", "elements": [{"tag": "Num", "elements": ["1"]}]}]}"#);

    let ir = parse("(1 + 1) * 1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"{"tag": "Expr", "elements": [{"tag": "Term", "elements": [{"tag": "Term", "elements": [{"tag": "Num", "elements": [{"tag": "Expr", "elements": [{"tag": "Expr", "elements": [{"tag": "Term", "elements": [{"tag": "Num", "elements": ["1"]}]}]}, "+", {"tag": "Term", "elements": [{"tag": "Num", "elements": ["1"]}]}]}]}]}, "*", {"tag": "Num", "elements": ["1"]}]}]}"#);
}
