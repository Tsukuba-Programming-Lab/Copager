use copager_core::{Generator, Processor};
use copager_cfl::token::TokenTag;
use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::{CFL, CFLTokens, CFLRules};
use copager_lex_regex::RegexLexer;
use copager_parse_lr_lr1::LR1;
use copager_ir_xml::Xml;

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
            .process::<Xml<_>>(input)
    };

    let ir = parse("1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"<list><tag>Expr</tag><elements><list><tag>Term</tag><elements><list><tag>Num</tag><elements><token><tag>Num</tag><text>"1"</text></token></elements></list></elements></list></elements></list>"#);

    let ir = parse("1 + 1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"<list><tag>Expr</tag><elements><list><tag>Expr</tag><elements><list><tag>Term</tag><elements><list><tag>Num</tag><elements><token><tag>Num</tag><text>"1"</text></token></elements></list></elements></list></elements></list><token><tag>Plus</tag><text>"+"</text></token><list><tag>Term</tag><elements><list><tag>Num</tag><elements><token><tag>Num</tag><text>"1"</text></token></elements></list></elements></list></elements></list>"#);

    let ir = parse("(1 + 1) * 1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"<list><tag>Expr</tag><elements><list><tag>Term</tag><elements><list><tag>Term</tag><elements><list><tag>Num</tag><elements><list><tag>Expr</tag><elements><list><tag>Expr</tag><elements><list><tag>Term</tag><elements><list><tag>Num</tag><elements><token><tag>Num</tag><text>"1"</text></token></elements></list></elements></list></elements></list><token><tag>Plus</tag><text>"+"</text></token><list><tag>Term</tag><elements><list><tag>Num</tag><elements><token><tag>Num</tag><text>"1"</text></token></elements></list></elements></list></elements></list></elements></list></elements></list><token><tag>Mul</tag><text>"*"</text></token><list><tag>Num</tag><elements><token><tag>Num</tag><text>"1"</text></token></elements></list></elements></list></elements></list>"#);
}
