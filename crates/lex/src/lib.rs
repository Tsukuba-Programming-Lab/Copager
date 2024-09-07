use copager_cfg::token::{TokenTag, Token};
#[cfg(feature = "derive")]
pub use copager_lex_derive::LexSource;

pub trait LexSource {
    type Tag: TokenTag;

    fn ignore_token(&self) -> &str;
    fn iter(&self) -> impl Iterator<Item = Self::Tag>;
}

pub trait LexDriver<S>
where
    Self: Sized,
    S: LexSource,
{
    fn try_from(source: S) -> anyhow::Result<Self>;
    fn run<'input>(&self, input: &'input str) -> impl Iterator<Item = Token<'input, S::Tag>>;
}
