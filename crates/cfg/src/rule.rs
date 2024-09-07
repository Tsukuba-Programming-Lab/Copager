use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::token::TokenTag;

pub trait RuleTag<T: TokenTag>
where
    Self: Debug + Copy + Clone + Hash + Eq,
{
    fn len(&self) -> usize;
    fn as_rules(&self) -> Vec<Rule<T>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule<T: TokenTag> {
    pub id: usize,
    pub lhs: RuleElem<T>,
    pub rhs: Vec<RuleElem<T>>,
}

impl<T: TokenTag> From<(RuleElem<T>, Vec<RuleElem<T>>)> for Rule<T> {
    fn from((lhs, rhs): (RuleElem<T>, Vec<RuleElem<T>>)) -> Self {
        Rule { id: 0, lhs, rhs }
    }
}

impl<T: TokenTag> Rule<T> {
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

#[derive(Debug, Clone, Eq)]
pub enum RuleElem<T: TokenTag> {
    NonTerm(String),
    Term(T),
    EOF,
}

impl<T: TokenTag> Hash for RuleElem<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            RuleElem::NonTerm(s) => s.hash(state),
            RuleElem::Term(t) => t.hash(state),
            RuleElem::EOF => 0.hash(state),
        }
    }
}

impl<T: TokenTag> PartialEq for RuleElem<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuleElem::NonTerm(s1), RuleElem::NonTerm(s2)) => s1 == s2,
            (RuleElem::Term(t1), RuleElem::Term(t2)) => t1 == t2,
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
pub struct RuleSet<T: TokenTag> {
    pub top: String,
    pub rules: Vec<Rule<T>>,
}

impl<T: TokenTag> FromIterator<Rule<T>> for RuleSet<T> {
    fn from_iter<I>(rules: I) -> Self
    where
        I: IntoIterator<Item = Rule<T>>,
    {
        let rules = rules.into_iter().collect::<Vec<_>>();
        let top = match &rules[0].lhs {
            RuleElem::NonTerm(s) => s.clone(),
            _ => unreachable!(),
        };
        RuleSet { top, rules }
    }
}

impl<T: TokenTag> RuleSet<T> {
    pub fn nonterms<'a>(&'a self) -> Vec<&'a RuleElem<T>> {
        self.rules.iter().flat_map(|rule| rule.nonterms()).collect()
    }

    pub fn terms<'a>(&'a self) -> Vec<&'a RuleElem<T>> {
        self.rules.iter().flat_map(|rule| rule.terms()).collect()
    }

    pub fn find_rule<'a>(&'a self, target: &RuleElem<T>) -> Vec<&'a Rule<T>> {
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

// #[cfg(test)]
// mod test {
//     use std::collections::HashMap;

//     use crate::token::TokenTag;
//     use crate::RuleKind;

//     use super::{Rule, RuleElem};

//     #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
//     enum TestToken {
//         Num,
//         Plus,
//         Minus,
//         Mul,
//         Div,
//         BracketA,
//         BracketB,
//     }

//     impl TokenKind<'_> for TestToken {
//         fn as_str(&self) -> &'static str {
//             match self {
//                 TestToken::Num => r"^[1-9][0-9]*",
//                 TestToken::Plus => r"^\+",
//                 TestToken::Minus => r"^-",
//                 TestToken::Mul => r"^\*",
//                 TestToken::Div => r"^/",
//                 TestToken::BracketA => r"^\(",
//                 TestToken::BracketB => r"^\)",
//             }
//         }

//         fn ignore_str() -> &'static str {
//             r"^[ \t\n]+"
//         }

//         fn into_iter() -> impl Iterator<Item = Self> {
//             vec![
//                 TestToken::Num,
//                 TestToken::Plus,
//                 TestToken::Minus,
//                 TestToken::Mul,
//                 TestToken::Div,
//                 TestToken::BracketA,
//                 TestToken::BracketB,
//             ]
//             .into_iter()
//         }
//     }

//     #[derive(Debug, Clone, Hash, PartialEq, Eq)]
//     enum TestRule {
//         ExprPlus,
//         ExprMinus,
//         Expr2Term,
//         TermMul,
//         TermDiv,
//         Term2Fact,
//         Fact2Expr,
//         Fact2Num,
//     }

//     impl<'a> RuleKind<'a> for TestRule {
//         type TokenKind = TestToken;

//         fn into_iter() -> impl Iterator<Item = Self> {
//             Box::new(
//                 vec![
//                     TestRule::ExprPlus,
//                     TestRule::ExprMinus,
//                     TestRule::Expr2Term,
//                     TestRule::TermMul,
//                     TestRule::TermDiv,
//                     TestRule::Term2Fact,
//                     TestRule::Fact2Expr,
//                     TestRule::Fact2Num,
//                 ]
//                 .into_iter(),
//             )
//         }

//         fn into_rules(&self) -> Vec<Rule<Self::TokenKind>> {
//             let expr_plus = Rule::from((
//                 RuleElem::new_nonterm("expr"),
//                 vec![
//                     RuleElem::new_nonterm("expr"),
//                     RuleElem::new_term(TestToken::Plus),
//                     RuleElem::new_nonterm("term"),
//                 ],
//             ));

//             let expr_minus = Rule::from((
//                 RuleElem::new_nonterm("expr"),
//                 vec![
//                     RuleElem::new_nonterm("expr"),
//                     RuleElem::new_term(TestToken::Minus),
//                     RuleElem::new_nonterm("term"),
//                 ],
//             ));

//             let expr_2_term = Rule::<TestToken>::from((
//                 RuleElem::new_nonterm("expr"),
//                 vec![RuleElem::new_nonterm("term")],
//             ));

//             let term_mul = Rule::from((
//                 RuleElem::new_nonterm("term"),
//                 vec![
//                     RuleElem::new_nonterm("term"),
//                     RuleElem::new_term(TestToken::Mul),
//                     RuleElem::new_nonterm("fact"),
//                 ],
//             ));

//             let term_div = Rule::from((
//                 RuleElem::new_nonterm("term"),
//                 vec![
//                     RuleElem::new_nonterm("term"),
//                     RuleElem::new_term(TestToken::Div),
//                     RuleElem::new_nonterm("fact"),
//                 ],
//             ));

//             let term_2_fact = Rule::<TestToken>::from((
//                 RuleElem::new_nonterm("term"),
//                 vec![RuleElem::new_nonterm("fact")],
//             ));

//             let fact_2_expr = Rule::from((
//                 RuleElem::new_nonterm("fact"),
//                 vec![
//                     RuleElem::new_term(TestToken::BracketA),
//                     RuleElem::new_nonterm("expr"),
//                     RuleElem::new_term(TestToken::BracketB),
//                 ],
//             ));

//             let fact_2_num = Rule::from((RuleElem::new_nonterm("fact"), vec![]));

//             match self {
//                 TestRule::ExprPlus => vec![expr_plus],
//                 TestRule::ExprMinus => vec![expr_minus],
//                 TestRule::Expr2Term => vec![expr_2_term],
//                 TestRule::TermMul => vec![term_mul],
//                 TestRule::TermDiv => vec![term_div],
//                 TestRule::Term2Fact => vec![term_2_fact],
//                 TestRule::Fact2Expr => vec![fact_2_expr],
//                 TestRule::Fact2Num => vec![fact_2_num],
//             }
//         }
//     }

//     fn check<T: Into<String>>(
//         first_set: &HashMap<&RuleElem<TestToken>, Vec<&RuleElem<TestToken>>>,
//         nonterm: T,
//         exp_terms: Vec<TestToken>,
//     ) {
//         let nonterms = RuleElem::<TestToken>::new_nonterm(nonterm);
//         let exp_terms: Vec<RuleElem<TestToken>> = exp_terms
//             .into_iter()
//             .map(|term| RuleElem::new_term(term))
//             .collect();
//         assert!(first_set.get(&nonterms).unwrap().len() == exp_terms.len());

//         let result = first_set
//             .get(&nonterms)
//             .unwrap()
//             .into_iter()
//             .zip(exp_terms.into_iter())
//             .any(|(a, b)| a == &&b);
//         assert!(result);
//     }

//     #[test]
//     fn first_set() {
//         let ruleset = <TestRule as RuleKind>::into_ruleset();
//         let first_set = ruleset.first_set();

//         check(
//             &first_set,
//             "expr",
//             vec![
//                 TestToken::Plus,
//                 TestToken::Minus,
//                 TestToken::Mul,
//                 TestToken::Div,
//                 TestToken::BracketA,
//             ],
//         );
//         check(
//             &first_set,
//             "term",
//             vec![TestToken::Mul, TestToken::Div, TestToken::BracketA],
//         );
//         check(&first_set, "fact", vec![TestToken::BracketA]);
//     }
// }
