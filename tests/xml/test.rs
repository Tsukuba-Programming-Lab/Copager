mod utils;

use copager::template::LALR1;
use copager::ir::Void;
use copager::Processor;

use utils::{Expect, test_dir};

use example_lang_xml::syntax::Xml;

#[test]
fn success() {
    test_dir("tests/xml/success", Expect::Ok, &parse);
}

#[test]
fn fail() {
    test_dir("tests/xml/fail", Expect::Err, &parse);
}

fn parse(input: &str) -> anyhow::Result<()> {
    type Config = LALR1<Xml>;
    type MyProcessor = Processor<Config>;

    MyProcessor::new()
        .build()?
        .process::<Void>(input)
        .and_then(|_| Ok(()))
}
