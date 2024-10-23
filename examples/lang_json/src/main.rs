use std::io::{stdin, stdout, Write};

use copager::lex::RegexLexer;
use copager::parse::LR1;
use copager::ir::SExp;
use copager::Processor;

use example_lang_json::*;

type MyLexer = RegexLexer<JsonToken>;
type MyParser = LR1<JsonToken, JsonRule>;
type MyProcessor = Processor<Json, MyLexer, MyParser>;

fn main() -> anyhow::Result<()> {
    println!("Example <json>");
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
