use std::io::{stdin, Read};

use copager::template::LALR1;
use copager::ir::SExp;
use copager::Processor;

use example_lang_xml::Xml;

type Config = LALR1<Xml>;
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
