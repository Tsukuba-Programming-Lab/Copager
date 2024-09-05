use copager_cfg::token::{TokenTag, Token};
use copager_cfg::rule::{RuleTag, RuleSet};
#[cfg(feature = "derive")]
pub use copager_parse_derive::ParseSource;

pub trait ParseSource<T: TokenTag> {
    type Tag: RuleTag<T>;

    fn iter(&self) -> impl Iterator<Item = Self::Tag>;

    fn into_ruleset(&self) -> RuleSet<T> {
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
            .collect::<RuleSet<_>>()
    }
}

pub trait ParseDriver<T, R>
where
    Self: From<Self::From>,
    T: TokenTag,
    R: RuleTag<T>,
{
    type From;

    fn run<'input, Il>(&self, lexer: Il) -> impl Iterator<Item = ()>
    where
        Il: Iterator<Item = Token<'input, T>>;
}
