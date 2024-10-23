use std::io::{stdin, stdout, Write};

use copager::lex::RegexLexer;
use copager::parse::LR1;
use copager::ir::SExp;
use copager::Processor;

use example_lang_pl0::*;

type MyLexer = RegexLexer<Pl0Token>;
type MyParser = LR1<Pl0Token, Pl0Rule>;
type MyProcessor = Processor<Pl0, MyLexer, MyParser>;

fn main() -> anyhow::Result<()> {
    println!("Example <pl0>");
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
