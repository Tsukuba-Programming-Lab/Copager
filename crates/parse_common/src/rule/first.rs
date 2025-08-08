use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use copager_lang::token::TokenTag;
use copager_lang::rule::{RuleElem, RuleSetData, RuleTag};

#[derive(Debug)]
pub struct FirstSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    map: HashMap<&'a RuleElem<T>, Vec<&'a RuleElem<T>>>,
    _phantom: PhantomData<R>,
}

impl<'a, T, R> From<&'a RuleSetData<T, R>> for FirstSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(ruleset: &'a RuleSetData<T, R>) -> Self {
        let build = FirstSetBuilder::from(ruleset).expand();
        let map = build.map
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        FirstSet {
            map,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, R> FirstSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn get(&self, relem: &RuleElem<T>) -> Option<&[&'a RuleElem<T>]> {
        self.map.get(relem).map(|terms| terms.as_slice())
    }

    pub fn get_by(&self, relems: &[RuleElem<T>]) -> Vec<&'a RuleElem<T>> {
        if relems.is_empty() {
            vec![&RuleElem::EOF]
        } else {
            let mut firsts: HashSet<&'a RuleElem<T>> = HashSet::new();
            for relem in relems {
                let first_candidates = self.map.get(relem).unwrap();
                firsts.extend(first_candidates);
                if firsts.contains(&RuleElem::Epsilon) {
                    firsts.remove(&RuleElem::Epsilon);
                    continue
                }
                return firsts.into_iter().collect();
            }
            firsts.insert(&RuleElem::EOF);
            firsts.into_iter().collect()
        }
    }
}

struct FirstSetBuilder<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    map: HashMap<&'a RuleElem<T>, HashSet<&'a RuleElem<T>>>,
    ruleset: &'a RuleSetData<T, R>,
    nonterms: Vec<&'a RuleElem<T>>,
}

impl<'a, T, R> From<&'a RuleSetData<T, R>> for FirstSetBuilder<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(ruleset: &'a RuleSetData<T, R>) -> Self {
        let mut map = HashMap::new();
        ruleset.nonterms().iter().for_each(|&nonterm| {
            map.insert(nonterm, HashSet::new());
        });
        ruleset.terms().iter().for_each(|&term| {
            map.insert(term, HashSet::new());
            map.get_mut(term).unwrap().insert(term);
        });
        map.insert(&RuleElem::Epsilon, HashSet::new());
        map.get_mut(&RuleElem::Epsilon).unwrap().insert(&RuleElem::Epsilon);
        map.insert(&RuleElem::EOF, HashSet::new());
        map.get_mut(&RuleElem::EOF).unwrap().insert(&RuleElem::EOF);

        let nonterms = ruleset.nonterms().into_iter().collect();

        FirstSetBuilder {
            map,
            ruleset,
            nonterms,
        }
    }
}

impl<'a, T, R> FirstSetBuilder<'a, T, R>
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
        for &nonterm in &self.nonterms {
            let old_len = self.map.get(nonterm).unwrap().len();
            for first_symbol in rhs_first_symbol(self.ruleset, nonterm) {
                if matches!(first_symbol, RuleElem::NonTerm(_)) {
                    let cand_terms = self.map.get(first_symbol).unwrap().clone();
                    self.map.get_mut(nonterm).unwrap().extend(cand_terms);
                } else {
                    self.map.get_mut(nonterm).unwrap().insert(first_symbol);
                }
            }
            modified |= old_len != self.map.get(nonterm).unwrap().len();
        }
        modified
    }
}

fn rhs_first_symbol<'a, T, R>(ruleset: &'a RuleSetData<T, R>, nonterm: &RuleElem<T>) -> impl Iterator<Item = &'a RuleElem<T>>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    ruleset.rules
        .iter()
        .filter(move |&rule| &rule.lhs == nonterm)
        .flat_map(|rule| rule.rhs.first())
}

#[cfg(test)]
mod test {
    use copager_lang::token::{TokenSet, TokenTag};
    use copager_lang::rule::{Rule, RuleElem, RuleSet, RuleTag};

    use super::FirstSet;

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, TokenSet)]
    enum TestToken {
        #[token(r"a")]
        A,
        #[token(r"b")]
        B,
    }

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, RuleSet)]
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
    fn first_set() {
        macro_rules! term {
            ($ident:ident) => { RuleElem::new_term(TestToken::$ident) };
        }
        macro_rules! nonterm {
            ($expr:expr) => { RuleElem::new_nonterm($expr) };
        }

        let ruleset = TestRule::instantiate().into_ruleset();
        let first_set = FirstSet::from(&ruleset);

        let expected = vec![term!(A)];
        assert!(eq_symbols(first_set.get(&nonterm!("S")).unwrap(), expected.as_slice()));

        let expected = vec![term!(A)];
        assert!(eq_symbols(first_set.get(&nonterm!("A")).unwrap(), expected.as_slice()));

        let expected = vec![term!(A)];
        assert!(eq_symbols(first_set.get(&nonterm!("B")).unwrap(), expected.as_slice()));

        let expected = vec![RuleElem::Epsilon];
        assert!(eq_symbols(first_set.get(&nonterm!("C")).unwrap(), expected.as_slice()));
    }
}
