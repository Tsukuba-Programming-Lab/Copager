mod utils;

use copager::lex::RegexLexer;
use copager::parse::LR1;
use copager::ir::Void;
use copager::Processor;

use utils::{Expect, test_dir};

use example_lang_xml::*;

#[test]
fn success() {
    test_dir("tests/xml/success", Expect::Ok, &parse);
}

#[test]
fn fail() {
    test_dir("tests/xml/fail", Expect::Err, &parse);
}

fn parse(input: &str) -> anyhow::Result<()> {
    type TestLexer = RegexLexer<XmlToken>;
    type TestParser = LR1<XmlToken, XmlRule>;
    type TestProcessor = Processor<Xml, TestLexer, TestParser>;

    TestProcessor::new()
        .build_lexer()?
        .build_parser()?
        .process::<Void>(input)?;

    Ok(())
}
