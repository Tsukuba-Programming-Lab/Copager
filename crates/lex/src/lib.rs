use copager_lang::token::Token;
use copager_lang::Lang;
use copager_utils::result::Result as CoResult;

pub trait BaseLexer<L>
where
    Self: Sized,
    L: Lang,
{
    fn init() -> CoResult<Self>;
    fn run<'input>(&self, input: &'input str)
        -> impl Iterator<Item = Token<'input, L::TokenTag>>;
}
