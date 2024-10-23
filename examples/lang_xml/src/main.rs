use std::io::{stdin, stdout, Write};

use copager::lex::RegexLexer;
use copager::parse::LR1;
use copager::ir::SExp;
use copager::Processor;

use example_lang_xml::*;

type MyLexer = RegexLexer<XmlToken>;
type MyParser = LR1<XmlToken, XmlRule>;
type MyProcessor = Processor<Xml, MyLexer, MyParser>;

fn main() -> anyhow::Result<()> {
    println!("Example <xml>");
    print!("Input: ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let sexp = MyProcessor::new()
        .build_lexer()?
        .build_parser()?
        .process::<SExp<_, _>>(&input)?;
    println!("Success: {}", sexp);

    Ok(())
}
