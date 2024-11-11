use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{RuleElem, RuleSet, RuleTag};

pub struct FirstSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    map: HashMap<String, Vec<&'a RuleElem<T>>>,
    _phantom: PhantomData<R>,
}

impl<'a, T, R> From<&'a RuleSet<T, R>> for FirstSet<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(ruleset: &'a RuleSet<T, R>) -> Self {
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
    pub fn get(&self, nonterm: &str) -> Option<&[&'a RuleElem<T>]> {
        self.map.get(nonterm).map(|terms| terms.as_slice())
    }
}

struct FirstSetBuilder<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    map: HashMap<String, HashSet<&'a RuleElem<T>>>,
    ruleset: &'a RuleSet<T, R>,
    nonterms: Vec<&'a str>,
}

impl<'a, T, R> From<&'a RuleSet<T, R>> for FirstSetBuilder<'a, T, R>
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

        let nonterms = ruleset.nonterms();
        let nonterms = nonterms
            .iter()
            .map(|relem| match relem {
                RuleElem::NonTerm(nonterm) => nonterm.as_str(),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

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
                match first_symbol {
                    RuleElem::NonTerm(first_nonterm) => {
                        let cand_terms = self.map.get(first_nonterm).unwrap().clone();
                        self.map.get_mut(nonterm).unwrap().extend(cand_terms);
                    },
                    _ => { self.map.get_mut(nonterm).unwrap().insert(first_symbol); }
                }
            }
            modified |= old_len != self.map.get(nonterm).unwrap().len();
        }
        modified
    }
}

fn rhs_first_symbol<'a, T, R>(ruleset: &'a RuleSet<T, R>, nonterm: &str) -> impl Iterator<Item = &'a RuleElem<T>>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    let cmp_nonterm = |relem: &RuleElem<T>, lhs: &str| match relem {
        RuleElem::NonTerm(nonterm) => nonterm == lhs,
        _ => false,
    };

    ruleset.rules
        .iter()
        .filter(move |rule| cmp_nonterm(&rule.lhs, nonterm))
        .flat_map(|rule| rule.rhs.first())
}

#[cfg(test)]
mod test {
    use copager_cfg::token::TokenTag;
    use copager_cfg::rule::{Rule, RuleTag, RuleElem};
    use copager_lex::LexSource;
    use copager_parse::ParseSource;

    use super::FirstSet;

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, LexSource)]
    enum TestToken {
        #[token(r"a")]
        A,
        #[token(r"b")]
        B,
    }

    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, ParseSource)]
    enum TestRule {
        #[default]
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
            ($expr:ident) => { RuleElem::new_term(TestToken::$expr) };
        }

        let ruleset = TestRule::default().into_ruleset();
        let first_set = FirstSet::from(&ruleset);

        let expected = vec![term!(A)];
        assert!(eq_symbols(first_set.get("S").unwrap(), expected.as_slice()));

        let expected = vec![term!(A)];
        assert!(eq_symbols(first_set.get("A").unwrap(), expected.as_slice()));

        let expected = vec![term!(A)];
        assert!(eq_symbols(first_set.get("B").unwrap(), expected.as_slice()));

        let expected = vec![RuleElem::Epsilon];
        assert!(eq_symbols(first_set.get("C").unwrap(), expected.as_slice()));
    }
}
