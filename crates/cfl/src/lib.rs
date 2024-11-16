pub mod rule;
pub mod token;

use std::marker::PhantomData;

use token::TokenTag;
use rule::{RuleTag, RuleSet};

#[cfg(feature = "derive")]
pub use copager_cfl_derive::{CFLTokens, CFLRules};

pub struct CFL<Ts, Rs>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    _phantom_ts: PhantomData<Ts>,
    _phantom_rs: PhantomData<Rs>,
}

pub trait CFLTokens {
    type Tag: TokenTag;

    fn ignore_token(&self) -> &str;
    fn iter(&self) -> impl Iterator<Item = Self::Tag>;
}

pub trait CFLRules<T: TokenTag> {
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
