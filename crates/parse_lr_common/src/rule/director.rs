use std::collections::{HashMap, HashSet};

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{Rule, RuleElem, RuleSet};

use crate::rule::{FirstSet, FollowSet};

pub struct DirectorSet<'a, T: TokenTag> {
    map: HashMap<&'a Rule<T>, Vec<&'a RuleElem<T>>>,
    _ruleset: &'a RuleSet<T>,
}

impl<'a, T: TokenTag> From<&'a RuleSet<T>> for DirectorSet<'a, T> {
    fn from(ruleset: &'a RuleSet<T>) -> Self {
        let build = DirectorSetBuilder::from(ruleset).calc();
        let map = build.map
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        DirectorSet {
            map,
            _ruleset: ruleset,
        }
    }
}

impl <'a, T: TokenTag> DirectorSet<'a, T> {
    pub fn get(&self, rule: &Rule<T>) -> Option<&[&'a RuleElem<T>]> {
        self.map.get(rule).map(|elems| elems.as_slice())
    }
}

struct DirectorSetBuilder<'a, T: TokenTag> {
    map: HashMap<&'a Rule<T>, HashSet<&'a RuleElem<T>>>,
    ruleset: &'a RuleSet<T>,
    first_set: FirstSet<'a, T>,
    follow_set: FollowSet<'a, T>,
}

impl<'a, T: TokenTag> From<&'a RuleSet<T>> for DirectorSetBuilder<'a, T> {
    fn from(ruleset: &'a RuleSet<T>) -> Self {
        let first_set = FirstSet::from(ruleset);
        let follow_set = FollowSet::from(ruleset);

        DirectorSetBuilder {
            map: HashMap::new(),
            ruleset,
            first_set,
            follow_set,
        }
    }
}

impl<'a, T: TokenTag> DirectorSetBuilder<'a, T> {
    fn calc(mut self) -> Self {
        for rule in &self.ruleset.rules {
            self.calc_once(rule);
        }
        self
    }

    fn calc_once(&mut self, rule: &'a Rule<T>) {
        let lhs = match &rule.lhs {
            RuleElem::NonTerm(s) => s.as_str(),
            _ => unreachable!(),
        };

        let rhs_firsts = self.first_by(&rule.rhs);
        let cand_elems = if !rhs_firsts.contains(&&RuleElem::Epsilon) {
            rhs_firsts
        } else {
            let mut cand_elems = rhs_firsts;
            cand_elems.extend_from_slice(self.follow_set.get(&lhs).unwrap());
            cand_elems
        };

        let director_elems = cand_elems
            .into_iter()
            .filter(|&e| *e != RuleElem::Epsilon)
            .collect();
        self.map.insert(rule, director_elems);
    }

    fn first_by(&self, relems: &'a [RuleElem<T>]) -> Vec<&'a RuleElem<T> > {
        if relems.is_empty() {
            vec![&RuleElem::Epsilon]
        } else {
            match &relems[0] {
                RuleElem::NonTerm(s) => self.first_set.get(s.as_str()).unwrap().to_vec(),
                t@RuleElem::Term(_) => vec![t],
                _ => vec![],
            }
        }
    }
}

#[cfg(test)]
mod test {
    use copager_cfg::token::TokenTag;
    use copager_cfg::rule::{Rule, RuleTag, RuleElem};
    use copager_lex::LexSource;
    use copager_parse::ParseSource;

    use super::DirectorSet;

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
    fn follow_set() {
        macro_rules! term {
            ($expr:ident) => { RuleElem::new_term(TestToken::$expr) };
        }

        let ruleset = TestRule::default().into_ruleset();
        let director_set = DirectorSet::from(&ruleset);

        let rule = &TestRule::S.as_rules()[0];
        let expected = vec![term!(A)];
        assert!(eq_symbols(director_set.get(rule).unwrap(), expected.as_slice()));

        let rule = &TestRule::A.as_rules()[0];
        let expected = vec![term!(A)];
        assert!(eq_symbols(director_set.get(rule).unwrap(), expected.as_slice()));

        let rule = &TestRule::B.as_rules()[0];
        let expected = vec![term!(A)];
        assert!(eq_symbols(director_set.get(rule).unwrap(), expected.as_slice()));

        let rule = &TestRule::C.as_rules()[0];
        let expected = vec![];
        assert!(eq_symbols(director_set.get(rule).unwrap(), expected.as_slice()));
    }
}
