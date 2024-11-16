use copager_cfl::token::{TokenTag, Token};
use copager_cfl::rule::{RuleTag, RuleSet};
use copager_lex::LexSource;
#[cfg(feature = "derive")]
pub use copager_parse_derive::ParseSource;

pub trait ParseSource<T: TokenTag> {
    type Tag: RuleTag<T>;

    fn iter(&self) -> impl Iterator<Item = Self::Tag>;

    fn into_ruleset(&self) -> RuleSet<T, Self::Tag> {
        let set_id_for_all = |(id, tag): (usize, Self::Tag)| {
            tag.as_rules()
                .into_iter()
                .map(move |rule| {
                    let mut rule = rule.clone();
                    rule.id = id;
                    rule
                })
        };
        self.iter()
            .enumerate()
            .flat_map(set_id_for_all)
            .collect::<RuleSet<_, _>>()
    }
}

pub trait BaseParser<Sl, Sp>
where
    Self: Sized,
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
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
