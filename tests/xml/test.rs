mod utils;

use copager::ir::Void;
use copager::Processor;

use utils::{Expect, test_dir};

use example_lang_xml::Xml;

#[test]
fn success() {
    test_dir("tests/xml/success", Expect::Ok, &parse);
}

#[test]
fn fail() {
    test_dir("tests/xml/fail", Expect::Err, &parse);
}

fn parse(input: &str) -> anyhow::Result<()> {
    Processor::<Xml>::new()
        .build_lexer()?
        .build_parser()?
        .process::<Void>(input)
        .and_then(|_| Ok(()))
}
