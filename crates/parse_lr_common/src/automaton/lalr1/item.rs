use std::fmt::{Display, Debug};
use std::hash::Hash;

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{Rule, RuleElem, RuleTag};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct LALR1Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub rule: &'a Rule<T, R>,
    pub dot_pos: usize,
    pub la_tokens: Vec<&'a RuleElem<T>>,
}

impl<'a, T, R> Display for LALR1Item<'a, T, R>
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
        write!(f, "[{:?}]", self.la_tokens)
    }
}

impl<'a, T, R> Debug for LALR1Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<'a, T, R> LALR1Item<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn new(rule: &'a Rule<T, R>, dot_pos: usize, la_tokens: Vec<&'a RuleElem<T>>) -> Self {
        LALR1Item { rule, dot_pos, la_tokens }
    }

    pub fn check_next_elem(&self) -> Option<&'a RuleElem<T>> {
        if self.dot_pos < self.rule.rhs.len() {
            Some(&self.rule.rhs[self.dot_pos])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LALR1ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub items: Vec<LALR1Item<'a, T, R>>,
}

impl <'a, T, R> LALR1ItemSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn new(items: Vec<LALR1Item<'a, T, R>>) -> Self {
        LALR1ItemSet { items }
    }
}
