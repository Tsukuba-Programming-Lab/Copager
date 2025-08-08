use std::marker::PhantomData;

use copager_core::{Generator, Processor};
use copager_cfl::token::{Token, TokenSet, TokenTag};
use copager_cfl::rule::{Rule, RuleElem, RuleSet, RuleTag};
use copager_cfl::CFL;
use copager_lex_regex::RegexLexer;
use copager_parse_lr_lalr1::LALR1;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(CFL)]
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
    #[rule("<Test> ::= <Test> Plus <term>")]
    #[rule("<Test> ::= <Test> Minus <term>")]
    #[rule("<Test> ::= <term>")]
    Test,
    #[rule("<term> ::= <term> Mul <num>")]
    #[rule("<term> ::= <term> Div <num>")]
    #[rule("<term> ::= <num>")]
    Term,
    #[rule("<num> ::= BracketL <Test> BracketR")]
    #[rule("<num> ::= Num")]
    Num,
}

#[derive(Debug, IR, IRBuilder)]
struct TestIR<'input, Lang: CFL> {
    _phantom_input: PhantomData<&'input ()>,
    _phantom_lang: PhantomData<Lang>,
}

impl<'input, Lang: CFL> From<RawIR<'input, Lang>> for TestIR<'input, Lang> {
    fn from(_: RawIR<'input, Lang>) -> Self {
        Self {
            _phantom_input: PhantomData,
            _phantom_lang: PhantomData,
        }
    }
}

#[test]
fn check_compile_builder() {
    type TestGenerator<T> = Generator<T, RegexLexer<T>, LALR1<T>>;
    type TestProcessor = Processor<TestGenerator<TestLang>>;

    TestProcessor::new()
        .build()
        .unwrap()
        .process::<TestIR<_>>("(10 + 20) * 30")
        .unwrap();
}
