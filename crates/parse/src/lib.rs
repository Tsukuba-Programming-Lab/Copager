use copager_cfl::token::{TokenTag, Token};
use copager_cfl::rule::RuleTag;
use copager_cfl::{CFLTokens, CFLRules};

pub trait BaseParser<Ts, Rs>
where
    Self: Sized,
    Ts: CFLTokens,
    Rs: CFLRules<<Ts as CFLTokens>::Tag>,
{
    fn try_from(source: (Ts, Rs)) -> anyhow::Result<Self>;
    fn run<'input, Il>(&self, lexer: Il) -> impl Iterator<Item = ParseEvent<'input, Ts::Tag, Rs::Tag>>
    where
        Il: Iterator<Item = Token<'input, Ts::Tag>>;
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
