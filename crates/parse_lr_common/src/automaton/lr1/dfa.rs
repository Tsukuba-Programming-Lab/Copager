use std::collections::BTreeMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::RwLock;
use std::marker::PhantomData;

use copager_cfl::token::TokenTag;
use copager_cfl::rule::{Rule, RuleElem, RuleSet, RuleTag};
use copager_parse_common::rule::FirstSet;

use crate::automaton::Automaton;
use crate::lr1::item::{LR1Item, LR1ItemSet};

#[derive(Clone)]
pub struct LR1DFANode<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub id: usize,
    pub itemset: LR1ItemSet<'a, 'b, T, R>,
    pub next: Vec<(&'a RuleElem<T>, Rc<RwLock<Self>>)>,  // (cond, next_node)
}

impl<'a, 'b, T, R> Debug for LR1DFANode<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct LR1DFANode<'a, 'b, 'c, T, R>
        where
            T: TokenTag,
            R: RuleTag<T>,
        {
            id: usize,
            itemset: &'c LR1ItemSet<'a, 'b, T, R>,
            next: Vec<(&'a RuleElem<T>, usize)>,
        }

        let id = self.id;
        let itemset = &self.itemset;
        let next = self.next
            .iter()
            .map(|(cond, next_node)| (*cond, next_node.read().unwrap().id))
            .collect::<Vec<_>>();

        if f.alternate() {
            return write!(f, "{:#?}", LR1DFANode { id, itemset, next });
        } else {
            write!(f, "{:?}", LR1DFANode { id, itemset, next })
        }
    }
}

impl<'a, 'b, T, R> PartialEq for LR1DFANode<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.itemset == other.itemset
    }
}

impl<'a, 'b, T, R> Eq for LR1DFANode<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{}

impl<'a, 'b, T, R> LR1DFANode<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn find_all(&self, rule: &Rule<T, R>) -> impl Iterator<Item = (&'a Rule<T, R>, &'a RuleElem<T>)> {
        self.find_all_by(move |item| item.rule == rule)
    }

    pub fn find_all_by<F>(&self, cond: F) -> impl Iterator<Item = (&'a Rule<T, R>, &'a RuleElem<T>)>
    where
        F: Fn(&&LR1Item<'a, T, R>) -> bool
    {
        self.itemset
            .items
            .iter()
            .filter(cond)
            .map(|item| (item.rule, item.la_token))
    }
}

#[derive(Debug)]
pub struct LR1DFA<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub nodes: Vec<Rc<RwLock<LR1DFANode<'a, 'b, T, R>>>>,
    pub edges: Vec<(usize, usize, &'a RuleElem<T>)>,
}

impl<'a, 'b, T, R> From<(&'a RuleSet<T, R>, &'b FirstSet<'a, T, R>)> for LR1DFA<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from((ruleset, first_set): (&'a RuleSet<T, R>, &'b FirstSet<'a, T, R>)) -> Self {
        let dfa_top = LR1DFABuilder::new().start(ruleset, &first_set);

        let mut nodes = BTreeMap::new();
        let mut edges = vec![];
        let mut stack = vec![dfa_top];
        while let Some(node) = stack.pop() {
            let from = node.read().unwrap().id;
            if nodes.contains_key(&from) {
                continue;
            }
            for (cond, next_node) in &node.read().unwrap().next {
                let to = next_node.read().unwrap().id;
                edges.push((from, to, *cond));
                stack.push(Rc::clone(next_node));
            }
            nodes.insert(from, Rc::clone(&node));
        }

        let nodes = nodes
            .into_iter()
            .map(|(_, node)| node)
            .collect();

        LR1DFA { nodes, edges }
    }
}

impl<'a: 'b, 'b, T, R> Automaton<'a, 'b, T> for LR1DFA<'a, 'b, T, R>
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
struct LR1DFABuilder<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    managed_itemsets: Vec<LR1ItemSet<'a, 'b, T, R>>,
    managed_nodes: Vec<Rc<RwLock<LR1DFANode<'a, 'b, T, R>>>>,
    _phantom_t: PhantomData<T>,
    _phantom_r: PhantomData<R>,
}

impl<'a, 'b, T, R> LR1DFABuilder<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn new() -> Self {
        LR1DFABuilder {
            managed_itemsets: vec![],
            managed_nodes: vec![],
            _phantom_t: PhantomData,
            _phantom_r: PhantomData,
        }
    }

    fn start(mut self, ruleset: &'a RuleSet<T, R>, first_set: &'b FirstSet<'a, T, R>) -> Rc<RwLock<LR1DFANode<'a, 'b, T, R>>> {
        let top = RuleElem::NonTerm(ruleset.top.clone());
        let top = ruleset.rules
            .iter()
            .find(|rule| rule.lhs == top)
            .unwrap();
        let top = LR1ItemSet::from((ruleset, first_set)).init(top);

        self.gen_recursive(top)
    }

    fn gen_recursive(&mut self, mut itemset: LR1ItemSet<'a, 'b, T, R>) -> Rc<RwLock<LR1DFANode<'a, 'b, T, R>>>
    where
        T: TokenTag,
    {
        let managed_idx = self.managed_itemsets.iter().position(|set| set == &itemset);
        if let Some(managed_idx) = managed_idx {
            return Rc::clone(&self.managed_nodes[managed_idx]);
        }

        let id = self.managed_itemsets.len();
        let node = LR1DFANode { id, itemset: itemset.clone(), next: vec![] };
        let node = Rc::new(RwLock::new(node));
        self.managed_itemsets.push(itemset.clone());
        self.managed_nodes.push(Rc::clone(&node));

        let mut next = vec![];
        for (cond, nextset) in itemset.gen_next_sets() {
            next.push((cond, self.gen_recursive(nextset)));
        }
        node.write().unwrap().next = next;

        Rc::clone(&node)
    }
}

#[cfg(test)]
mod test {
    // TODO
}
