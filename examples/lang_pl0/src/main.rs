use std::io::{stdin, stdout, Write};

use copager::ir::SExp;
use copager::Processor;

use example_lang_pl0::Pl0;

fn main() -> anyhow::Result<()> {
    println!("Example <pl0>");
    print!("Input: ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let sexp = Processor::<Pl0>::new()
        .build()?
        .process::<SExp<_>>(&input)?;
    println!("Success: {}", sexp);

    Ok(())
}
