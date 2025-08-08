use copager_lang::token::{TokenTag, Token};
use copager_lang::rule::RuleTag;
use copager_lang::Lang;

pub trait BaseParser<L>
where
    Self: Sized,
    L: Lang,
{
    fn init() -> anyhow::Result<Self>;
    fn run<'input, Il>(&self, lexer: Il)
        -> impl Iterator<Item = ParseEvent<'input, L::TokenTag, L::RuleTag>>
    where
        Il: Iterator<Item = Token<'input, L::TokenTag>>;
}

pub enum ParseEvent<'input, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    // Parsing Event
    Read(Token<'input, T>),
    Parse {
        rule: R,
        len: usize,
    },

    // Control
    Err(anyhow::Error),
}
