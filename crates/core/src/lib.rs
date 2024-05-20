pub mod cfg;
pub mod error;
pub mod parse;
pub mod lex;

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use lex::Lexer;
use parse::{ParserImpl, SExp};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parser<'a, Algorithm>
where
    Algorithm: ParserImpl<'a>,
{
    r#impl: Algorithm,
    phantom: PhantomData<&'a ()>,
}

#[allow(clippy::new_without_default)]
impl<'a, Algorithm> Parser<'a, Algorithm>
where
    Algorithm: ParserImpl<'a>,
{
    pub fn new() -> anyhow::Result<Parser<'a, Algorithm>> {
        Ok(Parser {
            r#impl: Algorithm::setup()?,
            phantom: PhantomData,
        })
    }

    pub fn parse<'b>(
        &self,
        input: &'b str,
    ) -> anyhow::Result<SExp<'a, 'b, Algorithm::TokenSet, Algorithm::Syntax>> {
        let lexer = Lexer::new::<Algorithm::TokenSet>(input)?;
        self.r#impl.parse(lexer)
    }
}
