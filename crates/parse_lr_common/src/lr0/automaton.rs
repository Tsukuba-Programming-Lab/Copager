use std::marker::PhantomData;

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{RuleElem, RuleSet};

use crate::lr0::item::LR0ItemSet;

#[derive(Debug)]
pub struct LR0DFANode<'a, T: TokenTag> {
    id: usize,
    pub itemset: LR0ItemSet<'a, T>,
    pub next: Vec<(&'a RuleElem<T>, Box<Self>)>,  // (cond, next_node)
}

#[derive(Debug)]
pub struct LR0DFA<'a, T: TokenTag> {
    nodes: usize,
    pub top: LR0DFANode<'a, T>,
}

impl<'a, T: TokenTag> From<&'a RuleSet<T>> for LR0DFA<'a, T> {
    fn from(ruleset: &'a RuleSet<T>) -> Self {
        let (nodes, top) = LR0DFABuilder::new().start(ruleset);
        LR0DFA { nodes, top }
    }
}

impl<'a, T: TokenTag> LR0DFA<'a, T> {
    pub fn all_nodes(&self) -> usize {
        self.nodes
    }

    pub gen fn all_edges(&self) -> (usize, usize, &'a RuleElem<T>) {
        let mut stack = vec![&self.top];
        while let Some(node) = stack.pop() {
            for (cond, next_node) in &node.next {
                stack.push(next_node);
                yield (node.id, next_node.id, *cond);
            }
        }
    }
}

#[derive(Debug)]
struct LR0DFABuilder<T> {
    nodes: usize,
    _phantom: PhantomData<T>,
}

impl<'a, T: TokenTag> LR0DFABuilder<T> {
    fn new() -> Self {
        LR0DFABuilder {
            nodes: 0,
            _phantom: PhantomData,
        }
    }

    fn start(mut self, ruleset: &'a RuleSet<T>) -> (usize, LR0DFANode<'a, T>) {
        let top = RuleElem::NonTerm(ruleset.top.clone());
        let top = ruleset.rules
            .iter()
            .find(|rule| rule.lhs == top)
            .unwrap();
        let top = LR0ItemSet::from(ruleset).init(top);

        (self.nodes, self.gen_recursive(top))
    }

    fn gen_recursive(&mut self, mut itemset: LR0ItemSet<'a, T>) -> LR0DFANode<'a, T>
    where
        T: TokenTag,
    {
        let id = self.nodes;
        let next = itemset
            .gen_next_sets()
            .map(|(cond, next_items) | {
                (cond, Box::new(self.gen_recursive(next_items)))
            })
            .collect();
        self.nodes += 1;

        LR0DFANode { id, itemset, next }
    }
}

#[cfg(test)]
mod test {
    // TODO
}
