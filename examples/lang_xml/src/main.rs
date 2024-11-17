use std::io::{stdin, stdout, Write};

use copager::ir::SExp;
use copager::Processor;

use example_lang_xml::Xml;

fn main() -> anyhow::Result<()> {
    println!("Example <xml>");
    print!("Input: ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let sexp = Processor::<Xml>::new()
        .build_lexer()?
        .build_parser()?
        .process::<SExp<_, _>>(&input)?;
    println!("Success: {}", sexp);

    Ok(())
}
