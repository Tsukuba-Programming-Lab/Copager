mod utils;

use copager::lex::RegexLexer;
use copager::parse::LR1;
use copager::ir::Void;
use copager::Processor;

use utils::{Expect, test_dir};

use example_lang_arithmetic::*;

#[test]
fn success() {
    test_dir("tests/arithmetic/success", Expect::Ok, &parse);
}

#[test]
fn fail() {
    test_dir("tests/arithmetic/fail", Expect::Err, &parse);
}

fn parse(input: &str) -> anyhow::Result<()> {
    type TestLexer = RegexLexer<ArithmeticToken>;
    type TestParser = LR1<ArithmeticToken, ArithmeticRule>;
    type TestProcessor = Processor<Arithmetic, TestLexer, TestParser>;

    TestProcessor::new()
        .build_lexer()?
        .build_parser()?
        .process::<Void>(input)?;

    Ok(())
}
