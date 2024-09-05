use copager_cfg::token::{TokenTag, Token};
#[cfg(feature = "derive")]
pub use copager_lex_derive::LexSource;

pub trait LexSource {
    type Tag: TokenTag;

    fn ignore_token(&self) -> &str;
    fn iter(&self) -> impl Iterator<Item = Self::Tag>;
}

pub trait LexIterator<'input, T>
where
    Self: Sized + From<Self::From>,
    T: TokenTag,
{
    type From;

    fn init(&self, input: &'input str) -> Self;
    fn next(&mut self) -> Option<Token<'input, T>>;
}
