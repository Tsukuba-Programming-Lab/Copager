use std::collections::HashSet;
use std::fmt::{Display, Debug};
use std::hash::Hash;

use serde::{Serialize, Deserialize};

use crate::token::TokenTag;

pub trait RuleTag<T: TokenTag>
where
    Self: Clone + Hash + Eq,
{
    fn as_rules(&self) -> Vec<Rule<T, Self>>;
}

#[derive(Clone, Eq, Serialize, Deserialize)]
pub struct Rule<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    #[serde(bound(
        serialize = "T: Serialize, R: Serialize",
        deserialize = "T: Deserialize<'de>, R: Deserialize<'de>",
    ))]
    pub id: usize,
    pub tag: Option<R>,
    pub lhs: RuleElem<T>,
    pub rhs: Vec<RuleElem<T>>,
}

impl<T, R> Display for Rule<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ->", self.lhs)?;
        for elem in &self.rhs {
            write!(f, " {}", elem)?;
        }
        write!(f, "")
    }
}

impl<T, R> Debug for Rule<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self, self.id)
    }
}

impl<T, R> PartialEq for Rule<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag && self.lhs == other.lhs && self.rhs == other.rhs
    }
}

impl<T, R> Hash for Rule<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
        self.lhs.hash(state);
        self.rhs.hash(state);
    }
}

impl<T, R> Rule<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn new(tag: Option<R>, lhs: RuleElem<T>, rhs: Vec<RuleElem<T>>) -> Self {
        Rule { id: 0, tag, lhs, rhs }
    }

    pub fn nonterms<'a>(&'a self) -> Vec<&'a RuleElem<T>> {
        let mut l_nonterms = vec![&self.lhs];
        let r_nonterms: Vec<&RuleElem<T>> = self
            .rhs
            .iter()
            .filter(|token| matches!(token, RuleElem::<T>::NonTerm(_)))
            .collect();
        l_nonterms.extend(r_nonterms);
        l_nonterms
    }

    pub fn terms<'a>(&'a self) -> Vec<&'a RuleElem<T>> {
        self.rhs
            .iter()
            .filter(|token| matches!(token, RuleElem::<T>::Term(_)))
            .collect()
    }
}

#[derive(Clone, Hash, Eq, Serialize, Deserialize)]
pub enum RuleElem<T: TokenTag> {
    #[serde(bound(
        serialize = "T: Serialize",
        deserialize = "T: Deserialize<'de>",
    ))]
    NonTerm(String),
    Term(T),
    Epsilon,
    EOF,
}

impl<T: TokenTag> Display for RuleElem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleElem::NonTerm(s) => write!(f, "<{}>", s),
            RuleElem::Term(t) => write!(f, "{:?}", t.as_str_list()),
            RuleElem::Epsilon => write!(f, "ε"),
            RuleElem::EOF => write!(f, "$"),
        }
    }
}

impl<T: TokenTag> Debug for RuleElem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T: TokenTag> PartialEq for RuleElem<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuleElem::NonTerm(s1), RuleElem::NonTerm(s2)) => s1 == s2,
            (RuleElem::Term(t1), RuleElem::Term(t2)) => t1 == t2,
            (RuleElem::Epsilon, RuleElem::Epsilon) => true,
            (RuleElem::EOF, RuleElem::EOF) => true,
            _ => false,
        }
    }
}

impl<T: TokenTag> RuleElem<T> {
    pub fn new_nonterm<U: Into<String>>(t: U) -> RuleElem<T> {
        RuleElem::NonTerm(t.into())
    }

    pub fn new_term(t: T) -> RuleElem<T> {
        RuleElem::Term(t)
    }
}

#[derive(Debug, Clone)]
pub struct RuleSet<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub top: String,
    pub rules: Vec<Rule<T, R>>,
}

impl<T, R> FromIterator<Rule<T, R>> for RuleSet<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from_iter<I>(rules: I) -> Self
    where
        I: IntoIterator<Item = Rule<T, R>>,
    {
        let rules = rules.into_iter().collect::<Vec<_>>();
        let top = match &rules[0].lhs {
            RuleElem::NonTerm(s) => s.clone(),
            _ => unreachable!(),
        };
        RuleSet { top, rules }
    }
}

impl<T, R> RuleSet<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn update_top(&mut self, rule: Rule<T, R>) {
        if let RuleElem::NonTerm(top) = &rule.lhs {
            self.top = top.to_string();
        }
        self.rules.push(rule);
    }

    pub fn nonterms<'a>(&'a self) -> HashSet<&'a RuleElem<T>> {
        self.rules.iter().flat_map(|rule| rule.nonterms()).collect()
    }

    pub fn terms<'a>(&'a self) -> HashSet<&'a RuleElem<T>> {
        self.rules.iter().flat_map(|rule| rule.terms()).collect()
    }

    pub fn find_rule<'a>(&'a self, target: &RuleElem<T>) -> Vec<&'a Rule<T, R>> {
        self.rules
            .iter()
            .filter(|rule| &rule.lhs == target)
            .collect()
    }
}
