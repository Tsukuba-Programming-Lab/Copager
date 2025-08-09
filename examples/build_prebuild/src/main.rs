use std::io::{stdin, Read};

use copager::template::LALR1;
use copager::ir::SExp;
use copager::Processor;

use language::Arithmetic;

type Config = LALR1<Arithmetic>;
type MyProcessor = Processor<Config>;

#[copager::load]
fn main(processor: MyProcessor) -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let sexp = processor
        .build_lexer()?
        .restore_parser_by_cache()
        .process::<SExp<_>>(&input)?;
    println!("Success: {}", sexp);

    Ok(())
}
