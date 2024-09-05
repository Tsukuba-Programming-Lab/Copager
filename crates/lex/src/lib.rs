use copager_cfg::token::{TokenTag, Token};
#[cfg(feature = "derive")]
pub use copager_lex_derive::LexSource;

pub trait LexSource {
    type Tag: TokenTag;

    fn ignore_token(&self) -> &str;
    fn iter(&self) -> impl Iterator<Item = Self::Tag>;
}

pub trait LexDriver<T>
where
    Self: Sized + From<Self::From>,
    T: TokenTag,
{
    type From;

    fn run<'input>(&self, input: &'input str) -> impl Iterator<Item = Token<'input, T>>;
}
