use crate::cfg::{TokenSet, Syntax};
use super::lex::LexIterator;

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
        lexer: impl LexIterator<'a, 'b, Self::TokenSet>
    ) -> anyhow::Result<Self::Output>;
}
