use std::collections::HashSet;
use std::rc::Rc;
use std::marker::PhantomData;

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{Rule, RuleElem, RuleSet, RuleTag};

use crate::automaton::Automaton;
use crate::lr0::item::{LR0Item, LR0ItemSet};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LR0DFANode<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub id: usize,
    pub itemset: LR0ItemSet<'a, T, R>,
    pub next: Vec<(&'a RuleElem<T>, Rc<Self>)>,  // (cond, next_node)
}

impl<'a, T, R> LR0DFANode<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn find_all(&self, rule: &Rule<T, R>) -> impl Iterator<Item = &'a Rule<T, R>> {
        self.find_all_by(move |item| item.rule == rule)
    }

    pub fn find_all_by<F>(&self, cond: F) -> impl Iterator<Item = &'a Rule<T, R>>
    where
        F: Fn(&&LR0Item<'a, T, R>) -> bool
    {
        self.itemset
            .items
            .iter()
            .filter(cond)
            .map(|item| item.rule)
    }
}

#[derive(Debug)]
pub struct LR0DFA<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub nodes: Vec<Rc<LR0DFANode<'a, T, R>>>,
    pub edges: Vec<(usize, usize, &'a RuleElem<T>)>,
}

impl<'a, T, R> From<&'a RuleSet<T, R>> for LR0DFA<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(ruleset: &'a RuleSet<T, R>) -> Self {
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

impl<'a: 'b, 'b, T, R> Automaton<'a, 'b, T> for LR0DFA<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn edges(&'b self) -> impl Iterator<Item = &'b (usize, usize, &'a RuleElem<T>)> {
        self.edges.iter()
    }
}

#[derive(Debug)]
struct LR0DFABuilder<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    itemsets: HashSet<LR0ItemSet<'a, T, R>>,
    _phantom_t: PhantomData<T>,
    _phantom_r: PhantomData<R>,
}

impl<'a, T, R> LR0DFABuilder<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn new() -> Self {
        LR0DFABuilder {
            itemsets: HashSet::new(),
            _phantom_t: PhantomData,
            _phantom_r: PhantomData,
        }
    }

    fn start(mut self, ruleset: &'a RuleSet<T, R>) -> LR0DFANode<'a, T, R> {
        let top = RuleElem::NonTerm(ruleset.top.clone());
        let top = ruleset.rules
            .iter()
            .find(|rule| rule.lhs == top)
            .unwrap();
        let top = LR0ItemSet::from(ruleset).init(top);

        self.gen_recursive(top).unwrap()
    }

    fn gen_recursive(&mut self, mut itemset: LR0ItemSet<'a, T, R>) -> Option<LR0DFANode<'a, T, R>>
    where
        T: TokenTag,
    {
        if self.itemsets.contains(&itemset) {
            return None;
        }

        let id = self.itemsets.len();
        self.itemsets.insert(itemset.clone());

        let next = itemset
            .gen_next_sets()
            .filter_map(|(cond, next_items) | {
                let next_node = self.gen_recursive(next_items);
                next_node.map(|next_node| (cond, Rc::new(next_node)))
            })
            .collect();

        Some(LR0DFANode { id, itemset, next })
    }
}

#[cfg(test)]
mod test {
    // TODO
}
