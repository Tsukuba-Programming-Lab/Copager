use std::rc::Rc;
use std::marker::PhantomData;

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{Rule, RuleElem, RuleSet};

use crate::lr0::item::{LR0Item, LR0ItemSet};

#[derive(Debug)]
pub struct LR0DFANode<'a, T: TokenTag> {
    id: usize,
    pub itemset: LR0ItemSet<'a, T>,
    pub next: Vec<(&'a RuleElem<T>, Rc<Self>)>,  // (cond, next_node)
}

impl<'a, T: TokenTag> LR0DFANode<'a, T> {
    pub fn contains(&self, rule: &Rule<T>) -> bool {
        self.contains_by(|item| item.rule == rule)
    }

    pub fn contains_by<F>(&self, cond: F) -> bool
    where
        F: Fn(&LR0Item<'a, T>) -> bool
    {
        self.itemset
            .items
            .iter()
            .any(cond)
    }
}

#[derive(Debug)]
pub struct LR0DFA<'a, T: TokenTag> {
    pub nodes: Vec<Rc<LR0DFANode<'a, T>>>,
    pub edges: Vec<(usize, usize, &'a RuleElem<T>)>,
}

impl<'a, T: TokenTag> From<&'a RuleSet<T>> for LR0DFA<'a, T> {
    fn from(ruleset: &'a RuleSet<T>) -> Self {
        let dfa_top = LR0DFABuilder::new().start(ruleset);

        let mut nodes = vec![];
        let mut edges = vec![];
        let mut stack = vec![Rc::new(dfa_top)];
        while let Some(node) = stack.pop() {
            nodes.push(Rc::clone(&node));
            for (cond, next_node) in &node.next {
                edges.push((node.id, next_node.id, *cond));
                stack.push(Rc::clone(next_node));
            }
        }

        LR0DFA { nodes, edges }
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

    fn start(mut self, ruleset: &'a RuleSet<T>) -> LR0DFANode<'a, T> {
        let top = RuleElem::NonTerm(ruleset.top.clone());
        let top = ruleset.rules
            .iter()
            .find(|rule| rule.lhs == top)
            .unwrap();
        let top = LR0ItemSet::from(ruleset).init(top);

        self.gen_recursive(top)
    }

    fn gen_recursive(&mut self, mut itemset: LR0ItemSet<'a, T>) -> LR0DFANode<'a, T>
    where
        T: TokenTag,
    {
        let id = self.nodes;
        let next = itemset
            .gen_next_sets()
            .map(|(cond, next_items) | {
                (cond, Rc::new(self.gen_recursive(next_items)))
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
