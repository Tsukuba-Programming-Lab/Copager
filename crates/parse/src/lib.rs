use copager_lang::token::{TokenTag, Token};
use copager_lang::rule::RuleTag;
use copager_lang::Lang;
use copager_utils::error::DiagnosticError;
use copager_utils::result::Result as CoResult;

pub trait BaseParser<L>
where
    Self: Sized,
    L: Lang,
{
    fn init() -> CoResult<Self>;
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
    Err(DiagnosticError),
}
