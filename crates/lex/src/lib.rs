use copager_cfl::token::Token;
use copager_cfl::CFLTokens;

pub trait BaseLexer<Ts>
where
    Self: Sized,
    Ts: CFLTokens,
{
    fn try_from(tokens: Ts) -> anyhow::Result<Self>;
    fn run<'input>(&self, input: &'input str) -> impl Iterator<Item = Token<'input, Ts::Tag>>;
}
