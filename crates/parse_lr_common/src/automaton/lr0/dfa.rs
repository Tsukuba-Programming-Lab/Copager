use std::collections::{HashMap, BTreeMap};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::RwLock;
use std::marker::PhantomData;

use copager_lang::token::TokenTag;
use copager_lang::rule::{Rule, RuleElem, RuleSetData, RuleTag};

use crate::automaton::Automaton;
use crate::lr0::item::{LR0Item, LR0ItemSet};

#[derive(Clone)]
pub struct LR0DFANode<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub id: usize,
    pub itemset: LR0ItemSet<'a, T, R>,
    pub next: Vec<(&'a RuleElem<T>, Rc<RwLock<Self>>)>,  // (cond, next_node)
}

impl<'a, T, R> Debug for LR0DFANode<'a, T, R>
where
    T: TokenTag + Debug,
    R: RuleTag<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct LR0DFANode<'a, 'b, T, R>
        where
            T: TokenTag,
            R: RuleTag<T>,
        {
            id: usize,
            itemset: &'b LR0ItemSet<'a, T, R>,
            next: Vec<(&'a RuleElem<T>, usize)>,
        }

        let id = self.id;
        let itemset = &self.itemset;
        let next = self.next
            .iter()
            .map(|(cond, next_node)| (*cond, next_node.read().unwrap().id))
            .collect::<Vec<_>>();

        if f.alternate() {
            return write!(f, "{:#?}", LR0DFANode { id, itemset, next });
        } else {
            write!(f, "{:?}", LR0DFANode { id, itemset, next })
        }
    }
}

impl<'a, T, R> Hash for LR0DFANode<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.itemset.hash(state);
    }
}

impl<'a, T, R> PartialEq for LR0DFANode<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.itemset == other.itemset
    }
}

impl<'a, T, R> Eq for LR0DFANode<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{}

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
    pub nodes: Vec<Rc<RwLock<LR0DFANode<'a, T, R>>>>,
    pub edges: Vec<(usize, usize, &'a RuleElem<T>)>,
}

impl<'a, T, R> From<&'a RuleSetData<T, R>> for LR0DFA<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(ruleset: &'a RuleSetData<T, R>) -> Self {
        let dfa_top = LR0DFABuilder::new().start(ruleset);

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
    itemsets: HashMap<LR0ItemSet<'a, T, R>, Rc<RwLock<LR0DFANode<'a, T, R>>>>,
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
            itemsets: HashMap::new(),
            _phantom_t: PhantomData,
            _phantom_r: PhantomData,
        }
    }

    fn start(mut self, ruleset: &'a RuleSetData<T, R>) -> Rc<RwLock<LR0DFANode<'a, T, R>>> {
        let top = RuleElem::NonTerm(ruleset.top.clone());
        let top = ruleset.rules
            .iter()
            .find(|rule| rule.lhs == top)
            .unwrap();
        let top = LR0ItemSet::from(ruleset).init(top);

        self.gen_recursive(top)
    }

    fn gen_recursive(&mut self, mut itemset: LR0ItemSet<'a, T, R>) -> Rc<RwLock<LR0DFANode<'a, T, R>>>
    where
        T: TokenTag,
    {
        if let Some(node) = self.itemsets.get(&itemset) {
            return Rc::clone(node);
        }

        let id = self.itemsets.len();
        let node = LR0DFANode { id, itemset: itemset.clone(), next: vec![] };
        let node = Rc::new(RwLock::new(node));
        self.itemsets.insert(itemset.clone(), Rc::clone(&node));

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
