use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;

use serde::{Serialize, Deserialize};
use itertools::Itertools;

use copager_core::cfg::{TokenSet, Syntax, Rule, RuleElem, RuleSet};

#[derive(Debug, Serialize, Deserialize)]
pub(super) enum LRAction<S> {
    Shift(usize),
    Reduce(S, usize, usize), // syntax, goto_id, elems_cnt
    Accept,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct LR1Configure<'a, T, S>
where
    T: TokenSet<'a>,
    S: Syntax<'a, TokenSet = T>,
{
    // LR Tables
    pub action_table: Vec<HashMap<T, LRAction<S>>>,
    pub eof_action_table: Vec<LRAction<S>>,
    pub goto_table: Vec<Vec<usize>>,

    // PhantomData
    tokenset: PhantomData<&'a T>,
}

impl<'a, T, S> LR1Configure<'a, T, S>
where
    T: TokenSet<'a>,
    S: Syntax<'a, TokenSet = T>,
{
    pub fn setup() -> anyhow::Result<Self> {
        // 1. Pre-process
        let rules = S::into_iter().collect::<Vec<_>>();
        let ruleset = S::into_ruleset();
        let first_set = ruleset.first_set();

        // 2. Generate dummy nonterm
        let top_dummy: Rule<T> = Rule::from((
            RuleElem::new_nonterm("__top_dummy"),
            vec![RuleElem::new_nonterm(&ruleset.top)],
        ));
        let top_dummy = vec![LRItem::new(
            &top_dummy,
            HashSet::from_iter(vec![&RuleElem::EOF]),
        )];
        let lr_items = LRItemSet::new(0, HashSet::from_iter(top_dummy));
        let lr_items = lr_items.expand_closure(&ruleset, &first_set);

        // 3. Generate a DFA
        let dfa = LRItemDFA::gen(lr_items, &ruleset, &first_set);

        // 4. Initialize tables
        let mut idx = 0;
        let mut nonterm_table = HashMap::new();
        for relem in ruleset.nonterms() {
            if let RuleElem::NonTerm(s) = &relem {
                if !nonterm_table.contains_key(s) {
                    nonterm_table.insert(s.to_string(), idx);
                    idx += 1;
                }
            }
        }

        let mut action_table: Vec<HashMap<T, LRAction<S>>> = Vec::with_capacity(dfa.0.len());
        let mut eof_action_table: Vec<LRAction<S>> = Vec::with_capacity(dfa.0.len());
        let mut goto_table: Vec<Vec<usize>> = Vec::with_capacity(dfa.0.len());
        for _ in 0..dfa.0.len() {
            action_table.push(HashMap::from_iter(
                T::into_iter()
                    .map(|token| (token, LRAction::None))
                    .collect::<Vec<(T, LRAction<S>)>>(),
            ));
            eof_action_table.push(LRAction::None);
            goto_table.push(vec![0; nonterm_table.keys().len()]);
        }

        // 5. Setup tables
        for lritem_set in &dfa.0 {
            for (token, next) in &lritem_set.next {
                match &token {
                    RuleElem::NonTerm(s) => {
                        let id = lritem_set.id as usize;
                        let label = *nonterm_table.get(s).unwrap();
                        goto_table[id][label] = *next as usize;
                    }
                    RuleElem::Term(t) => {
                        let id = lritem_set.id as usize;
                        let label = action_table[id].get_mut(&t.0).unwrap();
                        *label = LRAction::Shift(*next as usize);
                    }
                    _ => {}
                }
            }

            for item in &lritem_set.lr_items {
                if item.dot_pos != item.rule.rhs.len() {
                    continue;
                }
                if let RuleElem::NonTerm(lhs) = &item.rule.lhs {
                    for la_token in &item.la_tokens {
                        if let RuleElem::Term(t) = la_token {
                            let id = lritem_set.id as usize;
                            let label = action_table[id].get_mut(&t.0).unwrap();
                            *label = LRAction::Reduce(
                                rules[item.rule.id as usize],
                                *nonterm_table.get(lhs).unwrap(),
                                item.rule.rhs.len(),
                            );
                        }
                        if let RuleElem::EOF = la_token {
                            let id = lritem_set.id as usize;
                            eof_action_table[id] = if lhs == "__top_dummy" {
                                LRAction::Accept
                            } else {
                                LRAction::Reduce(
                                    rules[item.rule.id as usize],
                                    *nonterm_table.get(lhs).unwrap(),
                                    item.rule.rhs.len(),
                                )
                            };
                        }
                    }
                }
            }
        }

        Ok(LR1Configure {
            action_table,
            eof_action_table,
            goto_table,
            tokenset: PhantomData,
        })
    }
}

#[derive(Debug)]
struct LRItemDFA<'a, 'b, T: TokenSet<'a>> (
    Vec<LRItemSet<'a, 'b, T>>
);

impl<'a, 'b, T: TokenSet<'a>> LRItemDFA<'a, 'b, T> {
    fn gen(
        init_set: LRItemSet<'a, 'b, T>,
        ruleset: &'b RuleSet<'a, T>,
        first_set: &HashMap<&'b RuleElem<'a, T>, Vec<&'b RuleElem<'a, T>>>,
    ) -> LRItemDFA<'a, 'b, T> {
        let issue_id = |old_sets: &Vec<LRItemSet<'a, 'b, T>>, set: &LRItemSet<'a, 'b, T>| {
            if let Some(ex_set) = old_sets.iter().find(|&set0| set0.strict_eq(set)) {
                Err(ex_set.id)
            } else {
                Ok(old_sets.len() as i32)
            }
        };

        // "Expand a closure" <--> "Generate next nodes" loop
        let mut loop_idx = (0, 1);
        let mut lritem_sets = vec![init_set];
        while loop_idx.0 != loop_idx.1 {
            let mut new_found_cnt = 0;
            for idx in loop_idx.0..loop_idx.1 {
                let next_sets = lritem_sets[idx].gen_next_sets(ruleset, first_set);
                for (bef_token, mut next_set) in next_sets {
                    match issue_id(&lritem_sets, &next_set) {
                        Ok(id) => {
                            next_set.id = id;
                            lritem_sets[idx].next.insert(bef_token, id);
                            lritem_sets.push(next_set);
                            new_found_cnt += 1;
                        }
                        Err(id) => {
                            lritem_sets[idx].next.insert(bef_token, id);
                        }
                    }
                }
            }
            loop_idx = (loop_idx.1, loop_idx.1 + new_found_cnt);
        }

        LRItemDFA(lritem_sets)
    }
}

#[derive(Clone, Debug)]
struct LRItemSet<'a, 'b, T: TokenSet<'a>> {
    id: i32,
    next: HashMap<&'b RuleElem<'a, T>, i32>,
    lr_items: HashSet<LRItem<'a, 'b, T>>,
}

impl<'a, 'b, T: TokenSet<'a>> PartialEq for LRItemSet<'a, 'b, T> {
    fn eq(&self, other: &LRItemSet<'a, 'b, T>) -> bool {
        self.lr_items == other.lr_items
    }
}

impl<'a, 'b, T: TokenSet<'a>> PartialEq<HashSet<LRItem<'a, 'b, T>>> for LRItemSet<'a, 'b, T> {
    fn eq(&self, other: &HashSet<LRItem<'a, 'b, T>>) -> bool {
        &self.lr_items == other
    }
}

impl<'a, 'b, T: TokenSet<'a>> Eq for LRItemSet<'a, 'b, T> {}

impl<'a, 'b, T: TokenSet<'a>> LRItemSet<'a, 'b, T> {
    fn new(id: i32, lr_items: HashSet<LRItem<'a, 'b, T>>) -> Self {
        LRItemSet {
            id,
            next: HashMap::new(),
            lr_items,
        }
    }

    fn strict_eq(&self, other: &Self) -> bool {
        if self.lr_items.len() != other.lr_items.len() {
            return false;
        }
        self.lr_items
            .iter()
            .all(|item| other.lr_items.iter().any(|item_b| item_b.strict_eq(item)))
    }

    fn expand_closure<'c>(
        mut self,
        ruleset: &'b RuleSet<'a, T>,
        first_set: &'c HashMap<&'b RuleElem<'a, T>, Vec<&'b RuleElem<'a, T>>>,
    ) -> LRItemSet<'a, 'b, T> {
        let mut lr_items = self.lr_items.clone();
        let mut lr_items_fetched = self.lr_items;
        loop {
            let new_items: Vec<LRItem<'_, '_, _>> = lr_items_fetched
                .iter()
                .flat_map(|item| item.expand_closure(ruleset, first_set))
                .collect();
            let new_items = LRItem::<'_, '_, _>::unify_all(new_items);
            let new_items = HashSet::from_iter(new_items);

            let bef_len = lr_items.len();
            lr_items = LRItem::<'_, '_, _>::unity_set(lr_items, new_items.clone());
            let af_len = lr_items.len();
            if bef_len == af_len {
                break;
            }
            lr_items_fetched = new_items;
        }
        self.lr_items = lr_items;

        self
    }

    fn gen_next_sets<'c>(
        &self,
        ruleset: &'b RuleSet<'a, T>,
        first_set: &'c HashMap<&'b RuleElem<'a, T>, Vec<&'b RuleElem<'a, T>>>,
    ) -> HashMap<&'b RuleElem<'a, T>, LRItemSet<'a, 'b, T>> {
        let new_items: Vec<(&'b RuleElem<'a, T>, LRItem<'a, 'b, T>)> = self
            .lr_items
            .iter()
            .filter_map(|lr_item| lr_item.next_dot())
            .collect();

        let mut new_sets: HashMap<&RuleElem<T>, HashSet<LRItem<'_, '_, _>>> = HashMap::new();
        for (bef_token, lr_item) in new_items {
            if new_sets.get(&bef_token).is_none() {
                new_sets.insert(bef_token, HashSet::new());
            }
            new_sets.get_mut(&bef_token).unwrap().insert(lr_item);
        }

        let mut new_sets_expanded: HashMap<&'b RuleElem<'a, T>, LRItemSet<'_, '_, _>> = HashMap::new();
        for (ktoken, new_set) in new_sets {
            let new_set = LRItemSet::new(0, new_set);
            let new_set = new_set.expand_closure(ruleset, first_set);
            new_sets_expanded.insert(ktoken, new_set);
        }

        new_sets_expanded
    }
}

#[derive(Clone, Debug)]
struct LRItem<'a, 'b, T: TokenSet<'a>> {
    rule: &'b Rule<'a, T>,
    dot_pos: usize,
    la_tokens: HashSet<&'b RuleElem<'a, T>>,
}

impl<'a, 'b, T: TokenSet<'a>> Hash for LRItem<'a, 'b, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rule.hash(state);
        self.dot_pos.hash(state);
    }
}

impl<'a, 'b, T: TokenSet<'a>> PartialEq for LRItem<'a, 'b, T> {
    fn eq(&self, other: &Self) -> bool {
        self.rule == other.rule && self.dot_pos == other.dot_pos
    }
}

impl<'a, 'b, T: TokenSet<'a>> Eq for LRItem<'a, 'b, T> {}

impl<'a, 'b, T: TokenSet<'a>> LRItem<'a, 'b, T> {
    fn new(rule: &'b Rule<'a, T>, la_tokens: HashSet<&'b RuleElem<'a, T>>) -> LRItem<'a, 'b, T> {
        LRItem {
            rule,
            dot_pos: 0,
            la_tokens,
        }
    }

    fn strict_eq(&self, other: &Self) -> bool {
        self.rule == other.rule
            && self.dot_pos == other.dot_pos
            && self.la_tokens == other.la_tokens
    }

    fn expand_closure<'c>(
        &self,
        ruleset: &'b RuleSet<'a, T>,
        first_set: &'c HashMap<&'b RuleElem<'a, T>, Vec<&'b RuleElem<'a, T>>>,
    ) -> HashSet<LRItem<'a, 'b, T>> {
        let af_la_tokens = if self.dot_pos + 1 < self.rule.rhs.len() {
            HashSet::from_iter(
                first_set
                    .get(&self.rule.rhs[self.dot_pos + 1])
                    .unwrap()
                    .clone(),
            )
        } else {
            self.la_tokens.clone()
        };

        if self.dot_pos < self.rule.rhs.len()
            && matches!(self.rule.rhs[self.dot_pos], RuleElem::NonTerm(_))
        {
            ruleset
                .find_rule(&self.rule.rhs[self.dot_pos])
                .into_iter()
                .map(|rule| LRItem::<'_, '_, _>::new(rule, af_la_tokens.clone()))
                .collect()
        } else {
            HashSet::new()
        }
    }

    #[allow(clippy::int_plus_one)]
    fn next_dot(&self) -> Option<(&'b RuleElem<'a, T>, LRItem<'a, 'b, T>)> {
        if self.dot_pos + 1 <= self.rule.rhs.len() {
            let bef_token = &self.rule.rhs[self.dot_pos];
            let item = LRItem {
                rule: self.rule,
                dot_pos: self.dot_pos + 1,
                la_tokens: self.la_tokens.clone(),
            };
            Some((bef_token, item))
        } else {
            None
        }
    }

    fn unify(&mut self, other: LRItem<'a, 'b, T>) {
        if self != &other {
            return;
        }
        other.la_tokens.into_iter().for_each(|la_token| {
            if !self.la_tokens.contains(&la_token) {
                self.la_tokens.insert(la_token);
            }
        });
    }

    fn unify_all(mut items: Vec<LRItem<'a, 'b, T>>) -> Vec<LRItem<'a, 'b, T>> {
        for idx in (0..items.len()).permutations(2) {
            let (a_idx, b_idx) = (idx[0], idx[1]);
            let tmp = items[b_idx].clone();
            items[a_idx].unify(tmp);
        }
        items
    }

    fn unity_set(
        items_a: HashSet<LRItem<'a, 'b, T>>,
        items_b: HashSet<LRItem<'a, 'b, T>>,
    ) -> HashSet<LRItem<'a, 'b, T>> {
        let mut items_a = Vec::from_iter(items_a);
        let items_b = Vec::from_iter(items_b);
        items_a.extend(items_b);
        HashSet::from_iter(Self::unify_all(items_a))
    }
}
