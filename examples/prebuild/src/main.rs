use std::io::stdin;

use copager::ir::SExp;

use grammar::MyProcessor;

#[copager::load]
fn main(processor: MyProcessor) -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let sexp = processor
        .build_lexer()?
        .build_parser_by_cache()
        .process::<SExp<_, _>>(&input)?;
    println!("{}", sexp);

    Ok(())
}
