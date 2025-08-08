use copager_cfl::token::Token;
use copager_cfl::CFL;

pub trait BaseLexer<Lang>
where
    Self: Sized,
    Lang: CFL,
{
    fn init() -> anyhow::Result<Self>;
    fn run<'input>(&self, input: &'input str)
        -> impl Iterator<Item = Token<'input, Lang::TokenTag>>;
}
