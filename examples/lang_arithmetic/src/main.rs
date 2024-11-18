use std::io::{stdin, stdout, Write};

use copager::ir::SExp;
use copager::Processor;

use example_lang_arithmetic::Arithmetic;

fn main() -> anyhow::Result<()> {
    println!("Example <arithmetic>");
    print!("Input: ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let sexp = Processor::<Arithmetic>::new()
        .build()?
        .process::<SExp<_, _>>(&input)?;
    println!("Success: {}", sexp);

    Ok(())
}
