use std::collections::{HashMap, HashSet};

use copager_cfl::token::TokenTag;
use copager_cfl::rule::{RuleElem, RuleSet, RuleTag};

use crate::rule::FirstSet;

#[derive(Debug)]
pub struct FollowSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    map: HashMap<String, Vec<&'a RuleElem<T>>>,
    _ruleset: &'a RuleSet<T, R>,
}

impl<'a, T, R> From<&'a RuleSet<T, R>> for FollowSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(ruleset: &'a RuleSet<T, R>) -> Self {
        let build = FollowSetBuilder::from(ruleset).expand();
        let map = build.map
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        FollowSet {
            map,
            _ruleset: ruleset,
        }
    }
}

impl<'a, T, R> FollowSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn get(&self, nonterm: &str) -> Option<&[&'a RuleElem<T>]> {
        self.map.get(nonterm).map(|terms| terms.as_slice())
    }
}

pub struct FollowSetBuilder<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    map: HashMap<String, HashSet<&'a RuleElem<T>>>,
    ruleset: &'a RuleSet<T, R>,
    first_set: FirstSet<'a, T, R>,
}

impl<'a, T, R> From<&'a RuleSet<T, R>> for FollowSetBuilder<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(ruleset: &'a RuleSet<T, R>) -> Self {
        let mut map = HashMap::new();
        for nonterm in ruleset.nonterms() {
            if let RuleElem::NonTerm(nonterm) = nonterm {
                map.insert(nonterm.clone(), HashSet::new());
            }
        }
        map.get_mut(&ruleset.top).unwrap().insert(&RuleElem::EOF);

        FollowSetBuilder {
            map,
            ruleset,
            first_set: FirstSet::from(ruleset),
        }
    }
}

impl<'a, T, R> FollowSetBuilder<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn expand(mut self) -> Self {
        while self.expand_child() {}
        self
    }

    fn expand_child(&mut self) -> bool {
        let mut modified = false;
        for rule in &self.ruleset.rules {
            let lhs = match &rule.lhs {
                RuleElem::NonTerm(s) => s.as_str(),
                _ => unreachable!(),
            };
            for rhs_idx in 0..rule.rhs.len() {
                let target = &rule.rhs[rhs_idx];
                let follow_symbols = &rule.rhs[rhs_idx+1..];
                let prob_first_symbols = self.first_set.get_by(follow_symbols);
                modified |= self.append_by_first(target, &prob_first_symbols);
                if follow_symbols.is_empty() || prob_first_symbols.contains(&&RuleElem::Epsilon) {
                    modified |= self.append_when_nullable(target, lhs);
                }
            }
        }
        modified
    }

    fn append_by_first(&mut self, target: &RuleElem<T>, first_symbol: &[&'a RuleElem<T>]) -> bool {
        if let RuleElem::NonTerm(nonterm) = target {
            let old_len = self.map.get(nonterm).unwrap().len();
            let first_symbol = first_symbol.iter().filter(|relem| **relem != &RuleElem::Epsilon);
            self.map.get_mut(nonterm).unwrap().extend(first_symbol);
            old_len != self.map.get(nonterm).unwrap().len()
        } else {
            false
        }
    }

    fn append_when_nullable(&mut self, target: &RuleElem<T>, lhs: &str) -> bool {
        if let RuleElem::NonTerm(nonterm) = target {
            let lhs_follow = self.map.get(lhs).unwrap().clone();
            let old_len = self.map.get(nonterm).unwrap().len();
            self.map.get_mut(nonterm).unwrap().extend(lhs_follow);
            old_len != self.map.get(nonterm).unwrap().len()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use copager_cfl::token::TokenTag;
    use copager_cfl::rule::{Rule, RuleTag, RuleElem};
    use copager_cfl::{CFLToken, CFLRule};

    use super::FollowSet;

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, CFLToken)]
    enum TestToken {
        #[token(r"a")]
        A,
        #[token(r"b")]
        B,
    }

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, CFLRule)]
    enum TestRule {
        #[tokenset(TestToken)]
        #[rule("<S> ::= <A> <B>")]
        S,
        #[rule("<A> ::= A")]
        A,
        #[rule("<B> ::= <S> B")]
        B,
        #[rule("<C> ::= ")]
        C,
    }

    fn eq_symbols<T>(lhs: &[&RuleElem<T>], rhs: &[RuleElem<T>]) -> bool
    where
        T: TokenTag,
    {
        if lhs.len() != rhs.len() {
            println!("lhs: {:?}, rhs: {:?}", lhs, rhs);
            return false;
        }
        for lelem in lhs {
            if !rhs.contains(lelem) {
                println!("lhs: {:?}, rhs: {:?}", lhs, rhs);
                return false;
            }
        }
        return true;
    }

    #[test]
    fn follow_set() {
        macro_rules! term {
            ($expr:ident) => { RuleElem::new_term(TestToken::$expr) };
        }

        let ruleset = TestRule::instantiate().into_ruleset();
        let follow_set = FollowSet::from(&ruleset);

        let expected = vec![term!(B), RuleElem::EOF];
        assert!(eq_symbols(follow_set.get("S").unwrap(), expected.as_slice()));

        let expected = vec![term!(A)];
        assert!(eq_symbols(follow_set.get("A").unwrap(), expected.as_slice()));

        let expected = vec![term!(B), RuleElem::EOF];
        assert!(eq_symbols(follow_set.get("B").unwrap(), expected.as_slice()));

        let expected = vec![];
        assert!(eq_symbols(follow_set.get("C").unwrap(), expected.as_slice()));
    }
}
