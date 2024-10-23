mod utils;

use copager::lex::RegexLexer;
use copager::parse::LR1;
use copager::ir::Void;
use copager::Processor;

use utils::{Expect, test_dir};

use example_lang_pl0::*;

#[test]
fn success() {
    test_dir("tests/pl0/success", Expect::Ok, &parse);
}

#[test]
fn fail() {
    test_dir("tests/pl0/fail", Expect::Err, &parse);
}

fn parse(input: &str) -> anyhow::Result<()> {
    type TestLexer = RegexLexer<Pl0Token>;
    type TestParser = LR1<Pl0Token, Pl0Rule>;
    type TestProcessor = Processor<Pl0, TestLexer, TestParser>;

    TestProcessor::new()
        .build_lexer()?
        .build_parser()?
        .process::<Void>(input)?;

    Ok(())
}
