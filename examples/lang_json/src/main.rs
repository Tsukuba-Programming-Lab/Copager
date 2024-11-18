use std::io::{stdin, stdout, Write};

use copager::ir::SExp;
use copager::Processor;

use example_lang_json::Json;

fn main() -> anyhow::Result<()> {
    println!("Example <json>");
    print!("Input: ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let sexp = Processor::<Json>::new()
        .build()?
        .process::<SExp<_, _>>(&input)?;
    println!("Success: {}", sexp);

    Ok(())
}
