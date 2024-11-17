pub mod rule;
pub mod token;

use token::TokenTag;
use rule::{RuleTag, RuleSet};

#[cfg(feature = "derive")]
pub use copager_cfl_derive::{CFL, CFLTokens, CFLRules};

pub trait CFL {
    type TokenTag: TokenTag;
    type Tokens: CFLTokens<Tag = Self::TokenTag>;
    type RuleTag: RuleTag<Self::TokenTag>;
    type Rules: CFLRules<Self::TokenTag, Tag = Self::RuleTag>;

    fn instantiate_tokens(&self) -> Self::Tokens;
    fn instantiate_rules(&self) -> Self::Rules;
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
