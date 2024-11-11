use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use crate::token::TokenTag;

pub trait RuleTag<T: TokenTag>
where
    Self: Debug + Copy + Clone + Hash + Eq,
{
    fn as_rules(&self) -> Vec<Rule<T, Self>>;
}

#[derive(Debug, Clone, Eq)]
pub struct Rule<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub id: usize,
    pub tag: Option<R>,
    pub lhs: RuleElem<T>,
    pub rhs: Vec<RuleElem<T>>,
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

#[derive(Debug, Clone, Hash, Eq)]
pub enum RuleElem<T: TokenTag> {
    NonTerm(String),
    Term(T),
    Epsilon,
    EOF,
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

    pub fn first_set<'a>(&'a self) -> HashMap<&'a RuleElem<T>, Vec<&'a RuleElem<T>>> {
        // 1. Calc a null set
        let nulls_set = self.nulls_set();

        // 2. Initialize a first set
        let mut first_set: HashMap<&RuleElem<T>, Vec<&RuleElem<T>>> = HashMap::new();
        first_set.insert(&RuleElem::EOF, vec![&RuleElem::EOF]);
        self.terms().into_iter().for_each(|relem| {
            first_set.insert(relem, vec![relem]);
        });
        self.nonterms().into_iter().for_each(|relem| {
            first_set.insert(relem, vec![]);
        });

        // 3. List up candidates from a nonterm set
        let mut candidates = vec![];
        for nonterm in self.nonterms() {
            let rules = self.find_rule(nonterm);
            for rule in rules {
                for relem in &rule.rhs {
                    if &rule.lhs != relem {
                        candidates.push((nonterm, relem))
                    }
                    if !nulls_set.contains(&relem) {
                        break;
                    }
                }
            }
        }

        // 4. Find first set with recursive
        let mut updated = true;
        while updated {
            updated = false;
            for (nonterm, candidate) in &candidates {
                let found_elems: Vec<&RuleElem<T>> = first_set
                    .get(candidate)
                    .unwrap()
                    .iter()
                    .filter(|relem| !first_set.get(nonterm).unwrap().contains(relem))
                    .copied()
                    .collect();
                updated = !found_elems.is_empty();
                first_set
                    .get_mut(nonterm)
                    .unwrap()
                    .extend(found_elems.into_iter());
            }
        }

        first_set
    }

    fn nulls_set<'a>(&'a self) -> Vec<&'a RuleElem<T>> {
        // 1. Find null rules
        let mut nulls_set: Vec<&RuleElem<T>> = self
            .rules
            .iter()
            .filter(|rule| rule.rhs.is_empty())
            .map(|rule| &rule.lhs)
            .collect();

        // 2. Find null rules with recursive
        let mut updated = true;
        while updated {
            updated = false;
            for rule in &self.rules {
                if nulls_set.contains(&&rule.lhs) {
                    continue;
                } else if rule.rhs.iter().all(|relem| nulls_set.contains(&relem)) {
                    nulls_set.push(&rule.lhs);
                    updated = true;
                } else {
                    continue;
                }
            }
        }

        nulls_set
    }
}
