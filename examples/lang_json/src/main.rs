use std::io::{stdin, stdout, Write};

use copager::template::LALR1;
use copager::ir::SExp;
use copager::Processor;

use example_lang_json::Json;

type Config = LALR1<Json>;
type MyProcessor = Processor<Config>;

fn main() -> anyhow::Result<()> {
    println!("Example <json>");
    print!("Input: ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let sexp = MyProcessor::new()
        .build()?
        .process::<SExp<_>>(&input)?;
    println!("Success: {}", sexp);

    Ok(())
}
