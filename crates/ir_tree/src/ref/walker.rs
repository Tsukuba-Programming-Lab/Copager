use copager_lang::Lang;

use super::CSTree;

pub struct CSTreeWalker<'src, L: Lang> {
    cst: Option<CSTree<'src, L>>,
}

impl <'src, L: Lang> From<CSTree<'src, L>> for CSTreeWalker<'src, L> {
    fn from(cst: CSTree<'src, L>) -> Self {
        CSTreeWalker { cst: Some(cst) }
    }
}

impl<'src, L: Lang> CSTreeWalker<'src, L> {
    pub fn len(&self) -> usize {
        match &self.cst {
            Some(CSTree::Node { children, .. }) => children.len(),
            Some(CSTree::Leaf { .. }) => 1,
            None => 0,
        }
    }

    pub fn peek(&self) -> (Option<L::TokenTag>, Option<L::RuleTag>) {
        match &self.cst {
            Some(CSTree::Leaf { tag, .. }) => (Some(tag.clone()), None),
            Some(CSTree::Node { children, .. }) => {
                match children.get(0) {
                    Some(CSTree::Leaf { tag, .. }) => (Some(tag.clone()), None),
                    Some(CSTree::Node { tag, .. }) => (None, Some(tag.clone())),
                    None => (None, None),
                }
            },
            None => (None, None),
        }
    }

    pub fn expect_leaf(&mut self) -> (L::TokenTag, &'src str) {
        match self.pop_front() {
            Some(CSTree::Leaf { tag, text }) => (tag, text),
            Some(..) => panic!("Expected a leaf but found a node"),
            None => panic!("No more elements in the CSTreeWalker"),
        }
    }

    pub fn expect_node<T>(&mut self) -> T
    where
        T: From<CSTreeWalker<'src, L>>,
    {
        match self.pop_spawn() {
            Some(node_walker) => T::from(node_walker),
            None => panic!("No more elements in the CSTreeWalker"),
        }
    }

    pub fn expect_nodes<T>(&mut self) -> Vec<T>
    where
        T: From<CSTreeWalker<'src, L>>,
    {
        match self.pop_spawn() {
            Some(mut node_walker) => node_walker.expect_nodes_lrec::<T>(),
            None => vec![],
        }
    }

    fn expect_nodes_lrec<T>(&mut self) -> Vec<T>
    where
        T: From<CSTreeWalker<'src, L>>,
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

    fn pop_front(&mut self) -> Option<CSTree<'src, L>> {
        match &mut self.cst {
            Some(CSTree::Node { children, .. }) => children.pop_front(),
            Some(CSTree::Leaf { .. }) => self.cst.take(),
            None => None,
        }
    }

    fn pop_spawn(&mut self) -> Option<CSTreeWalker<'src, L>> {
        match self.pop_front() {
            Some(tree) => {
                Some(CSTreeWalker { cst: Some(tree) })
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

    use super::{CSTree, CSTreeWalker};

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
            .process::<CSTree<_>>("abc")?;
        let mut walker = CSTreeWalker::from(cst);

        assert_eq!(walker.peek(), (Some(TestToken::A), None));
        assert_eq!(walker.expect_leaf().0, TestToken::A);
        assert_eq!(walker.peek(), (None, Some(TestRule::RuleB)));

        Ok(())
    }

    #[test]
    fn test_walker_expect() -> anyhow::Result<()> {
        #[derive(Debug, PartialEq, Eq)]
        struct AstA;

        impl From<CSTreeWalker<'_, TestLang>> for AstA {
            fn from(mut walker: CSTreeWalker<'_, TestLang>) -> Self {
                assert_eq!(walker.expect_leaf().0, TestToken::A);
                assert_eq!(walker.expect_node::<AstB>(), AstB);
                AstA
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstB;

        impl From<CSTreeWalker<'_, TestLang>> for AstB {
            fn from(mut walker: CSTreeWalker<'_, TestLang>) -> Self {
                assert_eq!(walker.expect_leaf().0, TestToken::B);
                assert_eq!(walker.expect_nodes::<AstC>(), vec![AstC, AstC, AstC]);
                assert_eq!(walker.expect_nodes::<AstD>(), vec![]);
                AstB
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstC;

        impl From<CSTreeWalker<'_, TestLang>> for AstC {
            fn from(mut walker: CSTreeWalker<'_, TestLang>) -> Self {
                assert_eq!(walker.expect_leaf().0, TestToken::C);
                AstC
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstD;

        impl From<CSTreeWalker<'_, TestLang>> for AstD {
            fn from(mut walker: CSTreeWalker<'_, TestLang>) -> Self {
                assert_eq!(walker.expect_leaf().0, TestToken::D);
                AstD
            }
        }

        let cst = MyProcessor::new()
            .build_lexer()?
            .build_parser()?
            .process::<CSTree<_>>("abccc")?;
        let walker = CSTreeWalker::from(cst);

        assert_eq!(AstA::from(walker), AstA);

        Ok(())
    }
}
