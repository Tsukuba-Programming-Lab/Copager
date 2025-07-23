use copager_core::{Generator, Processor};
use copager_cfl::token::TokenTag;
use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::{CFL, CFLToken, CFLRule};
use copager_lex_regex::RegexLexer;
use copager_parse_lr_lr1::LR1;
use copager_ir_sexp::{SExp, SExpOwned};

#[derive(Default, CFL)]
struct TestLang (
    #[tokens] TestToken,
    #[rules]  TestRule
);

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLToken)]
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

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLRule)]
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
fn check_compile() -> anyhow::Result<()> {
    type TestGenerator<T> = Generator<T, RegexLexer<T>, LR1<T>>;
    type TestProcessor = Processor<TestGenerator<TestLang>>;

    TestProcessor::new()
        .build()?
        .process::<SExp<_>>("1 + 2 * 3")?;

    TestProcessor::new()
        .build()?
        .process::<SExpOwned<_>>("1 + 2 * 3")?;

    Ok(())
}

#[test]
fn check_display() {
    let parse = |input| {
        type TestGenerator<T> = Generator<T, RegexLexer<T>, LR1<T>>;
        type TestProcessor = Processor<TestGenerator<TestLang>>;
        TestProcessor::new()
            .build()?
            .process::<SExp<_>>(input)
    };

    let ir = parse("1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"(Expr (Term (Num "1")))"#);

    let ir = parse("1 + 1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"(Expr (Expr (Term (Num "1"))) "+" (Term (Num "1")))"#);

    let ir = parse("(1 + 1) * 1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"(Expr (Term (Term (Num (Expr (Expr (Term (Num "1"))) "+" (Term (Num "1"))))) "*" (Num "1")))"#);
}
