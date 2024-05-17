use crate::cfg::{TokenSet, Syntax};
use crate::lex::Token;

pub trait ParserImpl<'a>
where
    Self: Sized,
{
    type TokenSet: TokenSet<'a> + 'a;
    type Syntax: Syntax<'a, TokenSet = Self::TokenSet>;
    type Output;

    fn setup() -> anyhow::Result<Self>;
    fn parse<'b>(
        &self,
        lexer: impl Iterator<Item = Token<'a, 'b, Self::TokenSet>>,
    ) -> anyhow::Result<Self::Output>;
}
