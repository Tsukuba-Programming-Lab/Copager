use copager_cfl::token::{TokenTag, Token};
use copager_cfl::rule::RuleTag;
use copager_cfl::CFL;

pub trait BaseParser<Lang>
where
    Self: Sized,
    Lang: CFL,
{
    fn try_from(cfl: &Lang) -> anyhow::Result<Self>;
    fn run<'input, Il>(&self, lexer: Il)
        -> impl Iterator<Item = ParseEvent<'input, Lang::TokenTag, Lang::RuleTag>>
    where
        Il: Iterator<Item = Token<'input, Lang::TokenTag>>;
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
