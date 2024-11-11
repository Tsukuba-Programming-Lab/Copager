use std::collections::{HashMap, HashSet};

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{Rule, RuleElem, RuleSet, RuleTag};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct LR0Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub rule: &'a Rule<T, R>,
    pub dot_pos: usize,
}

impl<'a, T, R> From<&'a Rule<T, R>> for LR0Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(rule: &'a Rule<T, R>) -> Self {
        if rule.rhs[0] == RuleElem::Epsilon {
            LR0Item { rule, dot_pos: 1 }
        } else {
            LR0Item { rule, dot_pos: 0 }
        }
    }
}

impl<'a, T, R> LR0Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn gen_next(&self) -> Self {
        assert!(self.dot_pos + 1 <= self.rule.rhs.len());
        LR0Item {
            rule: self.rule,
            dot_pos: self.dot_pos + 1,
        }
    }

    pub fn check_next_elem(&self) -> Option<&'a RuleElem<T>> {
        if self.dot_pos < self.rule.rhs.len() {
            Some(&self.rule.rhs[self.dot_pos])
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct LR0ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub items: HashSet<LR0Item<'a, T, R>>,
    ruleset: &'a RuleSet<T, R>,
}

impl<'a, T, R> From<&'a RuleSet<T, R>> for LR0ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(ruleset: &'a RuleSet<T, R>) -> Self {
        LR0ItemSet {
            items: HashSet::new(),
            ruleset,
        }
    }
}

impl<'a, T, R> LR0ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn init(mut self, rule: &'a Rule<T, R>) -> Self {
        self.items.insert(LR0Item::from(rule));
        self
    }

    pub fn gen_next_sets(&mut self) -> impl Iterator<Item = (&'a RuleElem<T>, LR0ItemSet<'a, T, R>)> {
        self.expand();

        let mut next_set_candidates = HashMap::new();
        self.items
            .iter()
            .filter_map(|item| item.check_next_elem().map(|nelem| (nelem, item)))
            .for_each(|(nelem, item) | {
                next_set_candidates
                    .entry(nelem)
                    .or_insert_with(HashSet::new)
                    .insert(item.gen_next());
            });

        next_set_candidates
            .into_iter()
            .map(|(cond, items)|
                (cond, LR0ItemSet { items, ruleset: self.ruleset })
            )
    }

    fn expand(&mut self) {
        let mut modified = true;
        while modified {
            modified = false;
            let new_expaned = self.items
                .iter()
                .flat_map(|item| self.expand_once(item))
                .flatten()
                .collect::<Vec<_>>();
            for item in new_expaned {
                modified |= self.items.insert(item);
            }
        }
    }

    fn expand_once(&self, item: &LR0Item<'a, T, R>) -> Option<impl Iterator<Item = LR0Item<'a, T, R>>> {
        if let Some(nonterm@RuleElem::NonTerm(..)) = item.check_next_elem() {
            Some(self.ruleset
                .find_rule(nonterm)
                .into_iter()
                .map(|rule| LR0Item::from(rule)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    // TODO
}
