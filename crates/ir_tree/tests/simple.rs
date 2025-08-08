use copager_core::{Generator, Processor};
use copager_cfl::token::TokenTag;
use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::{CFL, CFLToken, CFLRule};
use copager_lex_regex::RegexLexer;
use copager_parse_lr_lr1::LR1;
use copager_ir_tree::{Tree, TreeOwned};

#[derive(CFL)]
struct TestLang (
    #[tokenset] TestToken,
    #[ruleset]  TestRule,
);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, CFLToken)]
enum TestToken {
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, CFLRule)]
enum TestRule {
    #[tokenset(TestToken)]
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
        .process::<Tree<_>>("1 + 2 * 3")?;

    TestProcessor::new()
        .build()?
        .process::<TreeOwned<_>>("1 + 2 * 3")?;

    Ok(())
}
