use std::io::{stdin, Read};

use copager::template::LALR1;
use copager::ir::SExp;
use copager::Processor;

use example_lang_pl0::syntax::Pl0;

type Config = LALR1<Pl0>;
type MyProcessor = Processor<Config>;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let sexp = MyProcessor::new()
        .build()?
        .process::<SExp<_>>(&input)?;
    println!("Success: {}", sexp);

    Ok(())
}
