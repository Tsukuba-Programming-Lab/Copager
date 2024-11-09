use std::collections::HashMap;

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{RuleElem, RuleSet};

pub struct FirstSet<'a, T: TokenTag> {
    map: HashMap<String, Vec<&'a T>>,
    ruleset: &'a RuleSet<T>,
}

impl<'a, T: TokenTag> TryFrom<&'a RuleSet<T>> for FirstSet<'a, T> {
    type Error = anyhow::Error;

    fn try_from(ruleset: &'a RuleSet<T>) -> anyhow::Result<Self> {
        let mut map = HashMap::new();
        for nonterm in ruleset.nonterms() {
            if let RuleElem::NonTerm(nonterm) = nonterm {
                let init_terms = rhs_terms(ruleset, nonterm).collect();
                map.insert(nonterm.clone(), init_terms);
            }
        }

        let mut first_set = FirstSet { map, ruleset };
        first_set.expand()?;

        Ok(first_set)
    }
}

impl<'a, T: TokenTag> FirstSet<'a, T> {
    fn expand(&mut self) -> anyhow::Result<bool> {
        let nonterms = &self.ruleset.nonterms();
        let nonterms = nonterms
            .iter()
            .map(|relem| match relem {
                RuleElem::NonTerm(nonterm) => nonterm,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let mut modified = false;
        for &nonterm in &nonterms {
            for rhs_nonterm in rhs_nonterms(self.ruleset, nonterm) {
                let cand_terms = self.map.get(rhs_nonterm).unwrap().clone();
                let lhs_terms = self.map.get_mut(nonterm).unwrap();
                for term in cand_terms {
                    if !lhs_terms.contains(&term) {
                        lhs_terms.push(term);
                        modified = true;
                    }
                }
            }
        }

        Ok(modified)
    }
}

impl<'a, T: TokenTag> FirstSet<'a, T> {
    pub fn get(&self, nonterm: &str) -> Option<&[&T]> {
        self.map.get(nonterm).map(|terms| terms.as_slice())
    }
}

fn cmp_nonterm<T: TokenTag>(relem: &RuleElem<T>, lhs: &str) -> bool {
    match relem {
        RuleElem::NonTerm(nonterm) => nonterm == lhs,
        _ => false,
    }
}

fn rhs_terms<'a, T>(ruleset: &'a RuleSet<T>, nonterm: &str) -> impl Iterator<Item = &'a T>
where
    T: TokenTag,
{
    ruleset.rules
        .iter()
        .filter(move |rule| cmp_nonterm(&rule.lhs, nonterm))
        .flat_map(|rule| &rule.rhs)
        .flat_map(|relem| match relem {
            RuleElem::Term(term) => Some(term),
            _ => None,
        })
}

fn rhs_nonterms<'a, T>(ruleset: &'a RuleSet<T>, nonterm: &str) -> impl Iterator<Item = &'a str>
where
    T: TokenTag,
{
    ruleset.rules
        .iter()
        .filter(move |rule| cmp_nonterm(&rule.lhs, nonterm))
        .flat_map(|rule| &rule.rhs)
        .flat_map(|relem| match relem {
            RuleElem::NonTerm(nonterm) => Some(nonterm.as_str()),
            _ => None,
        })
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
        #[rule("<B> ::= <S> A B")]
        B,
    }

    #[test]
    fn first_set() {
        let ruleset = TestRule::default().into_ruleset();
        let first_set = FirstSet::try_from(&ruleset).unwrap();

        let expected = vec![&TestToken::A, &TestToken::B];
        assert_eq!(first_set.get("S"), Some(expected.as_slice()));

        let expected = vec![&TestToken::A];
        assert_eq!(first_set.get("A"), Some(expected.as_slice()));

        let expected = vec![&TestToken::A, &TestToken::B];
        assert_eq!(first_set.get("B"), Some(expected.as_slice()));
    }
}
