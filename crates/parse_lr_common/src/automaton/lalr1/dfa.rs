use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::RwLock;

use copager_lang::token::TokenTag;
use copager_lang::rule::{Rule, RuleElem, RuleTag};

use crate::automaton::lr1::dfa::{LR1DFA, LR1DFANode};
use crate::lalr1::item::{LALR1Item, LALR1ItemSet};
use crate::automaton::Automaton;

#[derive(Debug)]
pub struct LALR1DFANode<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub id: usize,
    pub itemset: LALR1ItemSet<'a, T, R>,
}

impl<'a, T, R> LALR1DFANode<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from_lr1_nodes<'b>(id: usize, lr1_dfa_nodes: Vec<Rc<RwLock<LR1DFANode<'a, 'b, T, R>>>>) -> Self {
        let mut rule_la_tokens_map = HashMap::new();
        for lr1_dfa_node in &lr1_dfa_nodes {
            for rule in &lr1_dfa_node.read().unwrap().itemset.items {
                rule_la_tokens_map
                    .entry((rule.rule, rule.dot_pos))
                    .or_insert_with(HashSet::new)
                    .insert(rule.la_token);
            }
        }

        let grouped_items = rule_la_tokens_map
            .into_iter()
            .map(|((rule, dot_pos), la_tokens)| {
                LALR1Item::new(rule, dot_pos, la_tokens.into_iter().collect())
            })
            .collect();
        let itemset = LALR1ItemSet::new(grouped_items);

        LALR1DFANode { id, itemset }
    }
}

impl<'a, T, R> LALR1DFANode<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn find_all(&self, rule: &Rule<T, R>) -> impl Iterator<Item = (&'a Rule<T, R>, &[&'a RuleElem<T>])> {
        self.find_all_by(move |item| item.rule == rule)
    }

    pub fn find_all_by<F>(&self, cond: F) -> impl Iterator<Item = (&'a Rule<T, R>, &[&'a RuleElem<T>])>
    where
        F: Fn(&&LALR1Item<'a, T, R>) -> bool
    {
        self.itemset
            .items
            .iter()
            .filter(cond)
            .map(|item| (item.rule, item.la_tokens.as_slice()))
    }
}

#[derive(Debug)]
pub struct LALR1DFA<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub nodes: Vec<LALR1DFANode<'a, T, R>>,
    pub edges: Vec<(usize, usize, &'a RuleElem<T>)>,
}

impl<'a, 'b, T, R> From<LR1DFA<'a, 'b, T, R>> for LALR1DFA<'a, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(lr1_dfa: LR1DFA<'a, 'b, T, R>) -> Self {
        let lalr1_dfa_keys = lr1_dfa.nodes
            .into_iter()
            .map(LALR1DFAKey::from)
            .collect::<Vec<_>>();

        let mut managed_keys = vec![];
        let mut managed_lr1_nodes: Vec<Vec<_>> = vec![];
        for lalr1_dfa_key in lalr1_dfa_keys {
            let managed_idx = managed_keys.iter().position(|key| key == &lalr1_dfa_key);
            if let Some(managed_idx) = managed_idx {
                let lr1_node = Rc::clone(&lalr1_dfa_key.0);
                managed_lr1_nodes[managed_idx].push(lr1_node);
            } else {
                managed_lr1_nodes.push(vec![Rc::clone(&lalr1_dfa_key.0)]);
                managed_keys.push(lalr1_dfa_key);
            }
        }

        let mut lalr1_cand_node_sets = managed_keys
            .into_iter()
            .zip(managed_lr1_nodes.into_iter())
            .collect::<Vec<_>>();
        lalr1_cand_node_sets.sort_by_cached_key(|(key, _)| key.0.read().unwrap().id);

        let mut id_map = HashMap::new();
        let mut lalr1_nodes = vec![];
        for (new_id, (_, lalr1_cand_node_set)) in lalr1_cand_node_sets.into_iter().enumerate() {
            for lalr1_cand_node in &lalr1_cand_node_set {
                let old_id = lalr1_cand_node.read().unwrap().id;
                id_map.insert(old_id, new_id);
            }
            lalr1_nodes.push(LALR1DFANode::from_lr1_nodes(new_id, lalr1_cand_node_set));
        }

        let lalr1_edges = lr1_dfa.edges
            .into_iter()
            .map(|(from, to, cond)| {
                let from = id_map.get(&from).unwrap();
                let to = id_map.get(&to).unwrap();
                (*from, *to, cond)
            })
            .collect();

        LALR1DFA {
            nodes: lalr1_nodes,
            edges: lalr1_edges,
        }
    }
}

impl<'a: 'b, 'b, T, R> Automaton<'a, 'b, T> for LALR1DFA<'a, T, R>
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
struct LALR1DFAKey<'a, 'b, T, R> (Rc<RwLock<LR1DFANode<'a, 'b, T, R>>>)
where
    T: TokenTag,
    R: RuleTag<T>;

impl<'a, 'b, T, R> PartialEq for LALR1DFAKey<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn eq(&self, other: &Self) -> bool {
        let self_node = self.0.read().unwrap();
        let other_node = other.0.read().unwrap();

        if self_node.itemset.items.len() != other_node.itemset.items.len() {
            return false;
        }

        'outer: for item in &self_node.itemset.items {
            for other_item in &other_node.itemset.items {
                if item.rule == other_item.rule && item.dot_pos == other_item.dot_pos {
                    continue 'outer;
                }
            }
            return false;
        }

        true
    }
}

impl<'a, 'b, T, R> Eq for LALR1DFAKey<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{}

impl<'a, 'b, T, R> From<Rc<RwLock<LR1DFANode<'a, 'b, T, R>>>> for LALR1DFAKey<'a, 'b, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(lr1_dfa_node: Rc<RwLock<LR1DFANode<'a, 'b, T, R>>>) -> Self {
        Self (lr1_dfa_node)
    }
}
