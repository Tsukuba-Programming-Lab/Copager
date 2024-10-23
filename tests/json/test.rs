mod utils;

use copager::lex::RegexLexer;
use copager::parse::LR1;
use copager::ir::Void;
use copager::Processor;

use utils::{Expect, test_dir};

use example_lang_json::*;

#[test]
fn success() {
    test_dir("tests/json/success", Expect::Ok, &parse);
}

#[test]
fn fail() {
    test_dir("tests/json/fail", Expect::Err, &parse);
}

fn parse(input: &str) -> anyhow::Result<()> {
    type TestLexer = RegexLexer<JsonToken>;
    type TestParser = LR1<JsonToken, JsonRule>;
    type TestProcessor = Processor<Json, TestLexer, TestParser>;

    TestProcessor::new()
        .build_lexer()?
        .build_parser()?
        .process::<Void>(input)?;

    Ok(())
}
