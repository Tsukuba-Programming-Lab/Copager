use copager_core::{Generator, Processor};
use copager_lang::token::{TokenSet, TokenTag};
use copager_lang::rule::{Rule, RuleElem, RuleSet, RuleTag};
use copager_lang::Lang;
use copager_lex_regex::RegexLexer;
use copager_parse_lr_lr1::LR1;
use copager_ir_tree::r#ref::CSTree;
use copager_ir_tree::owned::CSTreeOwned;

#[derive(Lang)]
struct TestLang (
    #[tokenset] TestToken,
    #[ruleset]  TestRule,
);

#[derive(Clone, Hash, PartialEq, Eq, TokenSet)]
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

#[derive(Clone, Hash, PartialEq, Eq, RuleSet)]
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
        .process::<CSTree<_>>("1 + 2 * 3")?;

    TestProcessor::new()
        .build()?
        .process::<CSTreeOwned<_>>("1 + 2 * 3")?;

    Ok(())
}
