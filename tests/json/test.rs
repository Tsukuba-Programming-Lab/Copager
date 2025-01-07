mod utils;

use copager::ir::Void;
use copager::Processor;

use utils::{Expect, test_dir};

use example_lang_json::Json;

#[test]
fn success() {
    test_dir("tests/json/success", Expect::Ok, &parse);
}

#[test]
fn fail() {
    test_dir("tests/json/fail", Expect::Err, &parse);
}

fn parse(input: &str) -> anyhow::Result<()> {
    Processor::<Json>::new()
        .build()?
        .process::<Void>(input)
        .and_then(|_| Ok(()))
}
