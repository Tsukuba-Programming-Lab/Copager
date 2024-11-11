use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Debug};
use std::hash::Hash;

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{Rule, RuleElem, RuleSet, RuleTag};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct LR0Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub rule: &'a Rule<T, R>,
    pub dot_pos: usize,
}

impl<'a, T, R> Display for LR0Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> ", self.rule.lhs)?;
        for (i, elem) in self.rule.rhs.iter().enumerate() {
            if i == self.dot_pos {
                write!(f, "• ")?;
            }
            write!(f, "{} ", elem)?;
        }
        if self.dot_pos == self.rule.rhs.len() {
            write!(f, "•")?;
        }
        write!(f, "")
    }
}

impl<'a, T, R> Debug for LR0Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
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

#[derive(Clone)]
pub struct LR0ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub items: Vec<LR0Item<'a, T, R>>,
    ruleset: &'a RuleSet<T, R>,
}

impl<'a, T, R> Debug for LR0ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.items)
        } else {
            write!(f, "{:?}", self.items)
        }
    }
}

impl<'a, T, R> From<&'a RuleSet<T, R>> for LR0ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(ruleset: &'a RuleSet<T, R>) -> Self {
        LR0ItemSet {
            items: vec![],
            ruleset,
        }
    }
}

impl<'a, T, R> Hash for LR0ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.items.hash(state);
    }
}

impl<'a, T, R> PartialEq for LR0ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl <'a, T, R> Eq for LR0ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{}

impl<'a, T, R> LR0ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn init(mut self, rule: &'a Rule<T, R>) -> Self {
        let new_item = LR0Item::from(rule);
        if !self.items.contains(&new_item) {
            self.items.push(new_item);
        }
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
            .map(|(cond, items)| {
                let items = items.into_iter().collect();
                (cond, LR0ItemSet { items, ruleset: self.ruleset })
            })
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
                if self.items.contains(&item) {
                    continue;
                }
                self.items.push(item);
                modified = true;
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
