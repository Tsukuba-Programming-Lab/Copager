use copager_lang::token::Token;
use copager_lang::Lang;

pub trait BaseLexer<L>
where
    Self: Sized,
    L: Lang,
{
    fn init() -> anyhow::Result<Self>;
    fn run<'input>(&self, input: &'input str)
        -> impl Iterator<Item = Token<'input, L::TokenTag>>;
}
