use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use super::token::TokenSet;

pub trait Syntax<'a>
where
    Self: Debug + Clone + Copy,
{
    type TokenSet: TokenSet<'a>;

    fn into_iter() -> impl Iterator<Item = Self>;
    fn into_rule(&self) -> Rule<'a, Self::TokenSet>;

    fn into_ruleset() -> RuleSet<'a, Self::TokenSet> {
        let rules = Self::into_iter()
            .map(|elem| Self::into_rule(&elem))
            .collect::<Vec<_>>();

        RuleSet::from(rules)
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Rule<'a, T: TokenSet<'a>> {
    pub id: usize,
    pub lhs: RuleElem<'a, T>,
    pub rhs: Vec<RuleElem<'a, T>>,
    tokenset: PhantomData<&'a T>,
}

impl<'a, T: TokenSet<'a>> From<(RuleElem<'a, T>, Vec<RuleElem<'a, T>>)> for Rule<'a, T> {
    fn from((lhs, rhs): (RuleElem<'a, T>, Vec<RuleElem<'a, T>>)) -> Self {
        Rule {
            id: 0,
            lhs,
            rhs,
            tokenset: PhantomData,
        }
    }
}

impl<'a, T: TokenSet<'a>> Rule<'a, T> {
    pub fn nonterms<'b>(&'b self) -> Vec<&'b RuleElem<'a, T>> {
        let mut l_nonterms = vec![&self.lhs];
        let r_nonterms: Vec<&RuleElem<T>> = self
            .rhs
            .iter()
            .filter(|token| matches!(token, RuleElem::<T>::NonTerm(_)))
            .collect();
        l_nonterms.extend(r_nonterms);
        l_nonterms
    }

    pub fn terms<'b>(&'b self) -> Vec<&'b RuleElem<'a, T>> {
        self.rhs
            .iter()
            .filter(|token| matches!(token, RuleElem::<T>::Term(_)))
            .collect()
    }
}

#[derive(Debug)]
pub enum RuleElem<'a, T: TokenSet<'a>> {
    NonTerm(String),
    Term((T, PhantomData<&'a T>)),
    EOF,
}

impl<'a, T: TokenSet<'a>> Hash for RuleElem<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            RuleElem::NonTerm(s) => s.hash(state),
            RuleElem::Term(t) => t.hash(state),
            RuleElem::EOF => 0.hash(state),
        }
    }
}

impl<'a, T: TokenSet<'a>> PartialEq for RuleElem<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuleElem::NonTerm(s1), RuleElem::NonTerm(s2)) => s1 == s2,
            (RuleElem::Term(t1), RuleElem::Term(t2)) => t1 == t2,
            (RuleElem::EOF, RuleElem::EOF) => true,
            _ => false,
        }
    }
}

impl<'a, T: TokenSet<'a>> Eq for RuleElem<'a, T> {}

impl<'a, T: TokenSet<'a>> RuleElem<'a, T> {
    pub fn new_nonterm<U: Into<String>>(t: U) -> RuleElem<'a, T> {
        RuleElem::NonTerm(t.into())
    }

    pub fn new_term(t: T) -> RuleElem<'a, T> {
        RuleElem::Term((t, PhantomData))
    }
}

#[derive(Debug)]
pub struct RuleSet<'a, T: TokenSet<'a>> {
    pub top: String,
    pub rules: Vec<Rule<'a, T>>,
    tokenset: PhantomData<&'a T>,
}

impl<'a, T: TokenSet<'a>> From<Vec<Rule<'a, T>>> for RuleSet<'a, T> {
    fn from(mut rules: Vec<Rule<'a, T>>) -> Self {
        let top = match &rules[0].lhs {
            RuleElem::NonTerm(s) => s.clone(),
            _ => unreachable!(),
        };

        for (idx, rule) in rules.iter_mut().enumerate() {
            rule.id = idx;
        }

        RuleSet {
            top,
            rules,
            tokenset: PhantomData,
        }
    }
}

impl<'a, T: TokenSet<'a>> RuleSet<'a, T> {
    pub fn nonterms<'b>(&'b self) -> Vec<&'b RuleElem<'a, T>> {
        self.rules.iter().flat_map(|rule| rule.nonterms()).collect()
    }

    pub fn terms<'b>(&'b self) -> Vec<&'b RuleElem<'a, T>> {
        self.rules.iter().flat_map(|rule| rule.terms()).collect()
    }

    pub fn find_rule<'b>(&'b self, target: &RuleElem<'a, T>) -> Vec<&'b Rule<'a, T>> {
        self.rules
            .iter()
            .filter(|rule| &rule.lhs == target)
            .collect()
    }

    pub fn first_set<'b>(&'b self) -> HashMap<&'b RuleElem<'a, T>, Vec<&'b RuleElem<'a, T>>> {
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

    fn nulls_set<'b>(&'b self) -> Vec<&'b RuleElem<'a, T>> {
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{TokenSet, Syntax, Rule, RuleElem};

    #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
    enum TestToken {
        Num,
        Plus,
        Minus,
        Mul,
        Div,
        BracketA,
        BracketB,
    }

    impl TokenSet<'_> for TestToken {
        fn into_iter() -> impl Iterator<Item = Self> {
            Box::new(
                vec![
                    TestToken::Num,
                    TestToken::Plus,
                    TestToken::Minus,
                    TestToken::Mul,
                    TestToken::Div,
                    TestToken::BracketA,
                    TestToken::BracketB,
                ]
                .into_iter(),
            )
        }

        fn into_regex_str(&self) -> &'static str {
            match self {
                TestToken::Num => r"^[1-9][0-9]*",
                TestToken::Plus => r"^\+",
                TestToken::Minus => r"^-",
                TestToken::Mul => r"^\*",
                TestToken::Div => r"^/",
                TestToken::BracketA => r"^\(",
                TestToken::BracketB => r"^\)",
            }
        }

        fn ignore_str() -> &'static str {
            r"^[ \t\n]+"
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum TestSyntax {
        ExprPlus,
        ExprMinus,
        Expr2Term,
        TermMul,
        TermDiv,
        Term2Fact,
        Fact2Expr,
        Fact2Num,
    }

    impl<'a> Syntax<'a> for TestSyntax {
        type TokenSet = TestToken;

        fn into_iter() -> impl Iterator<Item = Self> {
            Box::new(
                vec![
                    TestSyntax::ExprPlus,
                    TestSyntax::ExprMinus,
                    TestSyntax::Expr2Term,
                    TestSyntax::TermMul,
                    TestSyntax::TermDiv,
                    TestSyntax::Term2Fact,
                    TestSyntax::Fact2Expr,
                    TestSyntax::Fact2Num,
                ]
                .into_iter(),
            )
        }

        fn into_rule(&self) -> Rule<'a, Self::TokenSet> {
            let expr_plus = Rule::from((
                RuleElem::new_nonterm("expr"),
                vec![
                    RuleElem::new_nonterm("expr"),
                    RuleElem::new_term(TestToken::Plus),
                    RuleElem::new_nonterm("term"),
                ],
            ));

            let expr_minus = Rule::from((
                RuleElem::new_nonterm("expr"),
                vec![
                    RuleElem::new_nonterm("expr"),
                    RuleElem::new_term(TestToken::Minus),
                    RuleElem::new_nonterm("term"),
                ],
            ));

            let expr_2_term = Rule::<TestToken>::from((
                RuleElem::new_nonterm("expr"),
                vec![RuleElem::new_nonterm("term")],
            ));

            let term_mul = Rule::from((
                RuleElem::new_nonterm("term"),
                vec![
                    RuleElem::new_nonterm("term"),
                    RuleElem::new_term(TestToken::Mul),
                    RuleElem::new_nonterm("fact"),
                ],
            ));

            let term_div = Rule::from((
                RuleElem::new_nonterm("term"),
                vec![
                    RuleElem::new_nonterm("term"),
                    RuleElem::new_term(TestToken::Div),
                    RuleElem::new_nonterm("fact"),
                ],
            ));

            let term_2_fact = Rule::<TestToken>::from((
                RuleElem::new_nonterm("term"),
                vec![RuleElem::new_nonterm("fact")],
            ));

            let fact_2_expr = Rule::from((
                RuleElem::new_nonterm("fact"),
                vec![
                    RuleElem::new_term(TestToken::BracketA),
                    RuleElem::new_nonterm("expr"),
                    RuleElem::new_term(TestToken::BracketB),
                ],
            ));

            let fact_2_num = Rule::from((RuleElem::new_nonterm("fact"), vec![]));

            match self {
                TestSyntax::ExprPlus => expr_plus,
                TestSyntax::ExprMinus => expr_minus,
                TestSyntax::Expr2Term => expr_2_term,
                TestSyntax::TermMul => term_mul,
                TestSyntax::TermDiv => term_div,
                TestSyntax::Term2Fact => term_2_fact,
                TestSyntax::Fact2Expr => fact_2_expr,
                TestSyntax::Fact2Num => fact_2_num,
            }
        }
    }

    fn check<T: Into<String>>(
        first_set: &HashMap<&RuleElem<TestToken>, Vec<&RuleElem<TestToken>>>,
        nonterm: T,
        exp_terms: Vec<TestToken>,
    ) {
        let nonterms = RuleElem::<TestToken>::new_nonterm(nonterm);
        let exp_terms: Vec<RuleElem<TestToken>> = exp_terms
            .into_iter()
            .map(|term| RuleElem::new_term(term))
            .collect();
        assert!(first_set.get(&nonterms).unwrap().len() == exp_terms.len());

        let result = first_set
            .get(&nonterms)
            .unwrap()
            .into_iter()
            .zip(exp_terms.into_iter())
            .any(|(a, b)| a == &&b);
        assert!(result);
    }

    #[test]
    fn first_set() {
        let ruleset = <TestSyntax as Syntax>::into_ruleset();
        let first_set = ruleset.first_set();

        check(
            &first_set,
            "expr",
            vec![
                TestToken::Plus,
                TestToken::Minus,
                TestToken::Mul,
                TestToken::Div,
                TestToken::BracketA,
            ],
        );
        check(
            &first_set,
            "term",
            vec![TestToken::Mul, TestToken::Div, TestToken::BracketA],
        );
        check(&first_set, "fact", vec![TestToken::BracketA]);
    }
}
