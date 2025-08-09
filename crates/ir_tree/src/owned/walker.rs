use copager_lang::Lang;

use super::CSTreeOwned;

pub struct CSTreeOwnedWalker<L: Lang> {
    cst: Option<CSTreeOwned<L>>,
}

impl <'src, L: Lang> From<CSTreeOwned<L>> for CSTreeOwnedWalker<L> {
    fn from(cst: CSTreeOwned<L>) -> Self {
        CSTreeOwnedWalker { cst: Some(cst) }
    }
}

impl<L: Lang> CSTreeOwnedWalker<L> {
    pub fn len(&self) -> usize {
        match &self.cst {
            Some(CSTreeOwned::Node { children, .. }) => children.len(),
            Some(CSTreeOwned::Leaf { .. }) => 1,
            None => 0,
        }
    }

    pub fn peek(&self) -> (Option<L::TokenTag>, Option<L::RuleTag>) {
        match &self.cst {
            Some(CSTreeOwned::Leaf { tag, .. }) => (Some(tag.clone()), None),
            Some(CSTreeOwned::Node { children, .. }) => {
                match children.get(0) {
                    Some(CSTreeOwned::Leaf { tag, .. }) => (Some(tag.clone()), None),
                    Some(CSTreeOwned::Node { tag, .. }) => (None, Some(tag.clone())),
                    None => (None, None),
                }
            },
            None => (None, None),
        }
    }

    pub fn expect_leaf(&mut self) -> L::TokenTag {
        match self.pop_front() {
            Some(CSTreeOwned::Leaf { tag, .. }) => tag,
            Some(..) => panic!("Expected a leaf but found a node"),
            None => panic!("No more elements in the CSTreeOwnedWalker"),
        }
    }

    pub fn expect_node<T>(&mut self) -> T
    where
        T: From<CSTreeOwnedWalker<L>>,
    {
        match self.pop_spawn() {
            Some(node_walker) => T::from(node_walker),
            None => panic!("No more elements in the CSTreeOwnedWalker"),
        }
    }

    pub fn expect_nodes<T>(&mut self) -> Vec<T>
    where
        T: From<CSTreeOwnedWalker<L>>,
    {
        match self.pop_spawn() {
            Some(mut node_walker) => node_walker.expect_nodes_lrec::<T>(),
            None => vec![],
        }
    }

    fn expect_nodes_lrec<T>(&mut self) -> Vec<T>
    where
        T: From<CSTreeOwnedWalker<L>>,
    {
        match (self.pop_spawn(), self.pop_spawn()) {
            (Some(mut lrec_walker), Some(last_walker)) => {
                let mut lrec_elems = lrec_walker.expect_nodes_lrec::<T>();
                let last_elem = T::from(last_walker);
                lrec_elems.push(last_elem);
                lrec_elems
            }
            (Some(last_walker), None) => {
                vec![T::from(last_walker)]
            }
            (None, None) => vec![],
            _ => unreachable!(),
        }
    }

    fn pop_front(&mut self) -> Option<CSTreeOwned<L>> {
        match &mut self.cst {
            Some(CSTreeOwned::Node { children, .. }) => children.pop_front(),
            Some(CSTreeOwned::Leaf { .. }) => self.cst.take(),
            None => None,
        }
    }

    fn pop_spawn(&mut self) -> Option<CSTreeOwnedWalker<L>> {
        match self.pop_front() {
            Some(tree) => {
                Some(CSTreeOwnedWalker { cst: Some(tree) })
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use copager_lang::token::{TokenSet, TokenTag};
    use copager_lang::rule::{Rule, RuleElem, RuleSet, RuleTag};
    use copager_lang::Lang;
    use copager_lex_regex::RegexLexer;
    use copager_parse_lr_lr1::LR1;
    use copager_core::{Generator, Processor};

    use super::{CSTreeOwned, CSTreeOwnedWalker};

    #[derive(Lang)]
    struct TestLang (
        #[tokenset] TestToken,
        #[ruleset]  TestRule,
    );

    #[derive(Debug, Clone, PartialEq, Eq, Hash, TokenSet)]
    enum TestToken {
        #[token(r"a")]
        A,
        #[token(r"b")]
        B,
        #[token(r"c")]
        C,
        #[token(r"d")]
        D,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash, RuleSet)]
    enum TestRule {
        #[tokenset(TestToken)]

        #[rule("<rule_a> ::= A <rule_b>")]
        RuleA,

        #[rule("<rule_b> ::= B <rule_c_list> <rule_d_list>")]
        RuleB,

        #[rule("<rule_c_list> ::= <rule_c_list> <rule_c>")]
        #[rule("<rule_c_list> ::= <rule_c>")]
        RuleCs,

        #[rule("<rule_c> ::= C")]
        RuleC,

        #[rule("<rule_d_list> ::= <rule_d_list> <rule_d>")]
        #[rule("<rule_d_list> ::= ")]
        RuleDs,

        #[rule("<rule_d> ::= D")]
        RuleD,
    }

    type Config<T> = Generator<T, RegexLexer<T>, LR1<T>>;
    type MyProcessor = Processor<Config<TestLang>>;

    #[test]
    fn test_walker_peek() -> anyhow::Result<()> {
        let cst = MyProcessor::new()
            .build_lexer()?
            .build_parser()?
            .process::<CSTreeOwned<_>>("abc")?;
        let mut walker = CSTreeOwnedWalker::from(cst);

        assert_eq!(walker.peek(), (Some(TestToken::A), None));
        assert_eq!(walker.expect_leaf(), TestToken::A);
        assert_eq!(walker.peek(), (None, Some(TestRule::RuleB)));

        Ok(())
    }

    #[test]
    fn test_walker_expect() -> anyhow::Result<()> {
        #[derive(Debug, PartialEq, Eq)]
        struct AstA;

        impl From<CSTreeOwnedWalker<TestLang>> for AstA {
            fn from(mut walker: CSTreeOwnedWalker<TestLang>) -> Self {
                assert_eq!(walker.expect_leaf(), TestToken::A);
                assert_eq!(walker.expect_node::<AstB>(), AstB);
                AstA
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstB;

        impl From<CSTreeOwnedWalker<TestLang>> for AstB {
            fn from(mut walker: CSTreeOwnedWalker<TestLang>) -> Self {
                assert_eq!(walker.expect_leaf(), TestToken::B);
                assert_eq!(walker.expect_nodes::<AstC>(), vec![AstC, AstC, AstC]);
                assert_eq!(walker.expect_nodes::<AstD>(), vec![]);
                AstB
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstC;

        impl From<CSTreeOwnedWalker<TestLang>> for AstC {
            fn from(mut walker: CSTreeOwnedWalker<TestLang>) -> Self {
                assert_eq!(walker.expect_leaf(), TestToken::C);
                AstC
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstD;

        impl From<CSTreeOwnedWalker<TestLang>> for AstD {
            fn from(mut walker: CSTreeOwnedWalker<TestLang>) -> Self {
                assert_eq!(walker.expect_leaf(), TestToken::D);
                AstD
            }
        }

        let cst = MyProcessor::new()
            .build_lexer()?
            .build_parser()?
            .process::<CSTreeOwned<_>>("abccc")?;
        let walker = CSTreeOwnedWalker::from(cst);

        assert_eq!(AstA::from(walker), AstA);

        Ok(())
    }
}
