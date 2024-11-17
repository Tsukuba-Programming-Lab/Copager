mod utils;

use copager::ir::Void;
use copager::Processor;

use utils::{Expect, test_dir};

use example_lang_pl0::Pl0;

#[test]
fn success() {
    test_dir("tests/pl0/success", Expect::Ok, &parse);
}

#[test]
fn fail() {
    test_dir("tests/pl0/fail", Expect::Err, &parse);
}

fn parse(input: &str) -> anyhow::Result<()> {
    Processor::<Pl0>::new()
        .build_lexer()?
        .build_parser()?
        .process::<Void>(input)?;

    Ok(())
}
