use copager_cfl::token::{TokenTag, Token};
use copager_cfl::rule::RuleTag;
use copager_cfl::{CFLTokens, CFLRules};

pub trait BaseParser<Sl, Sp>
where
    Self: Sized,
    Sl: CFLTokens,
    Sp: CFLRules<Sl::Tag>,
{
    fn try_from(source: (Sl, Sp)) -> anyhow::Result<Self>;
    fn run<'input, Il>(&self, lexer: Il) -> impl Iterator<Item = ParseEvent<'input, Sl::Tag, Sp::Tag>>
    where
        Il: Iterator<Item = Token<'input, Sl::Tag>>;
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
