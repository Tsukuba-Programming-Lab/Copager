pub mod rule;
pub mod token;

use std::hash::Hash;

use token::TokenTag;
use rule::{Rule, RuleSet};

pub trait RuleKind<T>
where
    Self: Clone + Hash + Eq,
    T: TokenTag,
{

    fn into_rules(&self) -> Vec<Rule<T>>;
    fn into_iter() -> impl Iterator<Item = Self>;

    fn into_ruleset() -> RuleSet<T> {
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
