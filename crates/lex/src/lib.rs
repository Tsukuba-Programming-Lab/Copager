use copager_cfl::token::Token;
use copager_cfl::CFLTokens;

pub trait BaseLexer<S>
where
    Self: Sized,
    S: CFLTokens,
{
    fn try_from(source: S) -> anyhow::Result<Self>;
    fn run<'input>(&self, input: &'input str) -> impl Iterator<Item = Token<'input, S::Tag>>;
}
