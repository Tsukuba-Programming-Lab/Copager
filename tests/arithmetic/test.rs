mod utils;

use copager::ir::Void;
use copager::Processor;

use utils::{Expect, test_dir};

use example_lang_arithmetic::Arithmetic;

#[test]
fn success() {
    test_dir("tests/arithmetic/success", Expect::Ok, &parse);
}

#[test]
fn fail() {
    test_dir("tests/arithmetic/fail", Expect::Err, &parse);
}

fn parse(input: &str) -> anyhow::Result<()> {
    Processor::<Arithmetic>::new()
        .build_lexer()?
        .build_parser()?
        .process::<Void>(input)
        .and_then(|_| Ok(()))
}
