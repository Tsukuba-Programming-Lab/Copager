pub mod rule;
pub mod token;

use token::{TokenTag, TokenSet};
use rule::{RuleTag, RuleSetData};

#[cfg(feature = "derive")]
pub use copager_cfl_derive::{CFL, CFLRule};

pub trait CFL {
    type TokenTag: TokenTag;
    type TokenSet: TokenSet<Tag = Self::TokenTag>;
    type RuleTag: RuleTag<Self::TokenTag>;
    type RuleSet: CFLRule<Self::TokenTag, Tag = Self::RuleTag>;
}

pub trait CFLRule<T: TokenTag> {
    type Tag: RuleTag<T>;

    fn instantiate() -> Self;
    fn iter(&self) -> impl Iterator<Item = Self::Tag>;

    fn into_ruleset(&self) -> RuleSetData<T, Self::Tag> {
        let set_id_for_all = |(id, tag): (usize, Self::Tag)| {
            tag.as_rules()
                .into_iter()
                .map(move |mut rule| { rule.id = id; rule })
        };
        self.iter()
            .enumerate()
            .flat_map(set_id_for_all)
            .collect::<RuleSetData<_, _>>()
    }
}
