use std::io::{stdin, stdout, Write};

use copager::ir::SExp;
use copager::Processor;

use language::Arithmetic;

type MyProcessor = Processor<Arithmetic>;

#[copager::load]
fn main(processor: MyProcessor) -> anyhow::Result<()> {
    println!("Example <pre-build>");
    print!("Input: ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let sexp = processor
        .build_lexer()?
        .restore_parser_by_cache()
        .process::<SExp<_>>(&input)?;
    println!("Success: {}", sexp);

    Ok(())
}
