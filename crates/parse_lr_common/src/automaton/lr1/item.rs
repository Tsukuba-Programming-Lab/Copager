use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Debug};
use std::hash::Hash;

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{Rule, RuleElem, RuleSet, RuleTag};
use copager_parse_common::rule::FirstSet;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct LR1Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub rule: &'a Rule<T, R>,
    pub dot_pos: usize,
    pub la_token: &'a RuleElem<T>,
}

impl<'a, T, R> Display for LR1Item<'a, T, R>
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
        write!(f, "[{}]", self.la_token)
    }
}

impl<'a, T, R> Debug for LR1Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<'a, T, R> From<(&'a Rule<T, R>, &'a RuleElem<T>)> for LR1Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from((rule, la_token): (&'a Rule<T, R>, &'a RuleElem<T>)) -> Self {
        if rule.rhs[0] == RuleElem::Epsilon {
            LR1Item { rule, dot_pos: 1, la_token: &RuleElem::EOF }
        } else {
            LR1Item { rule, dot_pos: 0, la_token }
        }
    }
}

impl<'a, T, R> LR1Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn gen_next(&self) -> Self {
        assert!(self.dot_pos + 1 <= self.rule.rhs.len());
        LR1Item {
            rule: self.rule,
            dot_pos: self.dot_pos + 1,
            la_token: self.la_token,
        }
    }

    pub fn check_next_elem(&self) -> Option<&'a RuleElem<T>> {
        if self.dot_pos < self.rule.rhs.len() {
            Some(&self.rule.rhs[self.dot_pos])
        } else {
            None
        }
    }

    pub fn check_next_elems<'b>(&'b self) -> Vec<RuleElem<T>> {
        let mut next_elems = Vec::from(&self.rule.rhs[self.dot_pos..]);
        next_elems.push(self.la_token.clone());
        next_elems
    }
}

#[derive(Clone)]
pub struct LR1ItemSet<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub items: Vec<LR1Item<'a, T, R>>,
    ruleset: &'a RuleSet<T, R>,
    first_set: &'b FirstSet<'a, T, R>,
}

impl<'a, 'b, T, R> Debug for LR1ItemSet<'a, 'b, T, R>
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

impl<'a, 'b, T, R> From<(&'a RuleSet<T, R>, &'b FirstSet<'a, T, R>)> for LR1ItemSet<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from((ruleset, first_set): (&'a RuleSet<T, R>, &'b FirstSet<'a, T, R>)) -> Self {
        LR1ItemSet {
            items: vec![],
            ruleset,
            first_set,
        }
    }
}

impl<'a, 'b, T, R> Hash for LR1ItemSet<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.items.hash(state);
    }
}

impl<'a, 'b, T, R> PartialEq for LR1ItemSet<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl <'a, 'b, T, R> Eq for LR1ItemSet<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{}

impl<'a, 'b, T, R> LR1ItemSet<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn init(mut self, rule: &'a Rule<T, R>) -> Self {
        self.items = vec![LR1Item::from((rule, &RuleElem::EOF))];
        self
    }

    pub fn gen_next_sets(&mut self) -> impl Iterator<Item = (&'a RuleElem<T>, LR1ItemSet<'a, 'b, T, R>)> {
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
                (cond, LR1ItemSet { items, ruleset: self.ruleset, first_set: self.first_set })
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

    fn expand_once(&self, item: &LR1Item<'a, T, R>) -> Option<impl Iterator<Item = LR1Item<'a, T, R>>> {
        if let Some(nonterm@RuleElem::NonTerm(..)) = item.check_next_elem() {
            Some(self.ruleset
                .find_rule(nonterm)
                .into_iter()
                .flat_map(|rule| {
                    let next_elems = item.check_next_elems();
                    self.first_set
                        .get_by(&next_elems[1..])
                        .into_iter()
                        .map(move |la_token| LR1Item::from((rule, la_token)))
                }))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    // TODO
}
