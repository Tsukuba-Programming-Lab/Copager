pub mod rule;
pub mod token;

use std::hash::Hash;

use rule::{Rule, RuleSet};

pub trait TokenKind<'a>
where
    Self: Copy + Clone + Hash + Eq,
{
    fn as_str(&self) -> &'a str;
    fn ignore_str() -> &'a str;
    fn into_iter() -> impl Iterator<Item = Self>;
}

pub trait RuleKind<'a>
where
    Self: Clone + Hash + Eq,
{
    type TokenKind: crate::TokenKind<'a>;

    fn into_rules(&self) -> Vec<Rule<'a, Self::TokenKind>>;
    fn into_iter() -> impl Iterator<Item = Self>;

    fn into_ruleset() -> RuleSet<'a, Self::TokenKind> {
        Self::into_iter()
            .enumerate()
            .flat_map(|(idx, elem)| {
                let mut rules = Self::into_rules(&elem);
                for rule in &mut rules {
                    rule.id = idx;
                }
                rules
            })
            .collect::<RuleSet<_>>()
    }
}
