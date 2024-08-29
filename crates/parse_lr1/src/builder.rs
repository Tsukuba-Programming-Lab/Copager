use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use itertools::Itertools;
use serde::ser::SerializeStruct;
use serde::{Serialize, Deserialize};

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{Rule, RuleElem, RuleSet};
use copager_lex::LexSource;
use copager_parse::ParseSource;

#[derive(Debug, Serialize, Deserialize)]
pub enum LRAction<R> {
    Shift(usize),
    Reduce(R, usize, usize), // tag, goto_id, elems_cnt
    Accept,
    None,
}

#[derive(Debug)]
pub struct LR1Configure<Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    pub action_table: Vec<HashMap<Sl::Tag, LRAction<Sp::Tag>>>,
    pub eof_action_table: Vec<LRAction<Sp::Tag>>,
    pub goto_table: Vec<Vec<usize>>,
}

impl<Sl, Sp> Serialize for LR1Configure<Sl, Sp>
where
    Sl: LexSource,
    Sl::Tag: Serialize,
    Sp: ParseSource<Sl::Tag>,
    Sp::Tag: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("LR1Configure", 3)?;
        state.serialize_field("action_table", &self.action_table)?;
        state.serialize_field("eof_action_table", &self.eof_action_table)?;
        state.serialize_field("goto_table", &self.goto_table)?;
        state.end()
    }
}

impl<'de, Sl, Sp> Deserialize<'de> for LR1Configure<Sl, Sp>
where
    Sl: LexSource,
    Sl::Tag: for<'de_o> Deserialize<'de_o>,
    Sp: ParseSource<Sl::Tag>,
    Sp::Tag: for<'de_o> Deserialize<'de_o>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct LR1ConfigureHelper<Sl, Sp>
        where
            Sl: LexSource,
            Sl::Tag: for<'de_h> Deserialize<'de_h>,
            Sp: ParseSource<Sl::Tag>,
            Sp::Tag: for<'de_h> Deserialize<'de_h>,
        {
            action_table: Vec<HashMap<Sl::Tag, LRAction<Sp::Tag>>>,
            eof_action_table: Vec<LRAction<Sp::Tag>>,
            goto_table: Vec<Vec<usize>>,
        }

        let helper = LR1ConfigureHelper::<Sl, Sp>::deserialize(deserializer)?;
        Ok(LR1Configure {
            action_table: helper.action_table,
            eof_action_table: helper.eof_action_table,
            goto_table: helper.goto_table,
        })
    }
}

impl<Sl, Sp> LR1Configure<Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    pub fn new(source_l: &Sl, source_p: &Sp) -> anyhow::Result<Self> {
        // 1. Pre-process
        let ruleset = source_p.into_ruleset();
        let first_set = ruleset.first_set();

        // 2. Generate dummy nonterm
        let top_dummy: Rule<Sl::Tag> = Rule::from((
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

        let mut action_table: Vec<HashMap<Sl::Tag, LRAction<Sp::Tag>>> = Vec::with_capacity(dfa.0.len());
        let mut eof_action_table: Vec<LRAction<Sp::Tag>> = Vec::with_capacity(dfa.0.len());
        let mut goto_table: Vec<Vec<usize>> = Vec::with_capacity(dfa.0.len());
        for _ in 0..dfa.0.len() {
            action_table.push(HashMap::from_iter(
                source_l.iter()
                    .map(|token| (token, LRAction::None))
                    .collect::<Vec<(Sl::Tag, LRAction<Sp::Tag>)>>(),
            ));
            eof_action_table.push(LRAction::None);
            goto_table.push(vec![0; nonterm_table.keys().len()]);
        }

        // 5. Setup tables
        let rule_tags = source_p.iter().collect::<Vec<_>>();
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
                        let label = action_table[id].get_mut(t).unwrap();
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
                            let label = action_table[id].get_mut(t).unwrap();
                            *label = LRAction::Reduce(
                                rule_tags[item.rule.id as usize],
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
                                    rule_tags[item.rule.id as usize],
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
        })
    }
}

#[derive(Debug)]
struct LRItemDFA<'a, T: TokenTag> (
    Vec<LRItemSet<'a, T>>
);

impl<'a, T: TokenTag> LRItemDFA<'a, T> {
    fn gen(
        init_set: LRItemSet<'a, T>,
        ruleset: &'a RuleSet<T>,
        first_set: &HashMap<&'a RuleElem<T>, Vec<&'a RuleElem<T>>>,
    ) -> LRItemDFA<'a, T> {
        let issue_id = |old_sets: &Vec<LRItemSet<'a, T>>, set: &LRItemSet<'a, T>| {
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

#[derive(Clone, Debug, Eq)]
struct LRItemSet<'a, T: TokenTag> {
    id: i32,
    next: HashMap<&'a RuleElem<T>, i32>,
    lr_items: HashSet<LRItem<'a, T>>,
}

impl<'a, T: TokenTag> PartialEq for LRItemSet<'a, T> {
    fn eq(&self, other: &LRItemSet<'a, T>) -> bool {
        self.lr_items == other.lr_items
    }
}

impl<'a, T: TokenTag> PartialEq<HashSet<LRItem<'a, T>>> for LRItemSet<'a, T> {
    fn eq(&self, other: &HashSet<LRItem<'a, T>>) -> bool {
        &self.lr_items == other
    }
}

impl<'a, T: TokenTag> LRItemSet<'a, T> {
    fn new(id: i32, lr_items: HashSet<LRItem<'a, T>>) -> Self {
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

    fn expand_closure<'b>(
        mut self,
        ruleset: &'a RuleSet<T>,
        first_set: &'b HashMap<&'a RuleElem<T>, Vec<&'a RuleElem<T>>>,
    ) -> LRItemSet<'a, T> {
        let mut lr_items = self.lr_items.clone();
        let mut lr_items_fetched = self.lr_items;
        loop {
            let new_items: Vec<LRItem<'_, _>> = lr_items_fetched
                .iter()
                .flat_map(|item| item.expand_closure(ruleset, first_set))
                .collect();
            let new_items = LRItem::<'_, _>::unify_all(new_items);
            let new_items = HashSet::from_iter(new_items);

            let bef_len = lr_items.len();
            lr_items = LRItem::<'_, _>::unity_set(lr_items, new_items.clone());
            let af_len = lr_items.len();
            if bef_len == af_len {
                break;
            }
            lr_items_fetched = new_items;
        }
        self.lr_items = lr_items;

        self
    }

    fn gen_next_sets<'b>(
        &self,
        ruleset: &'a RuleSet<T>,
        first_set: &'b HashMap<&'a RuleElem<T>, Vec<&'a RuleElem<T>>>,
    ) -> HashMap<&'a RuleElem<T>, LRItemSet<'a, T>> {
        let new_items: Vec<(&'a RuleElem<T>, LRItem<'a, T>)> = self
            .lr_items
            .iter()
            .filter_map(|lr_item| lr_item.next_dot())
            .collect();

        let mut new_sets: HashMap<&RuleElem<T>, HashSet<LRItem<'_, _>>> = HashMap::new();
        for (bef_token, lr_item) in new_items {
            if new_sets.get(&bef_token).is_none() {
                new_sets.insert(bef_token, HashSet::new());
            }
            new_sets.get_mut(&bef_token).unwrap().insert(lr_item);
        }

        let mut new_sets_expanded: HashMap<&'a RuleElem<T>, LRItemSet<'_, _>> = HashMap::new();
        for (ktoken, new_set) in new_sets {
            let new_set = LRItemSet::new(0, new_set);
            let new_set = new_set.expand_closure(ruleset, first_set);
            new_sets_expanded.insert(ktoken, new_set);
        }

        new_sets_expanded
    }
}

#[derive(Clone, Debug, Eq)]
struct LRItem<'a, T: TokenTag> {
    rule: &'a Rule<T>,
    dot_pos: usize,
    la_tokens: HashSet<&'a RuleElem<T>>,
}

impl<'a, T: TokenTag> Hash for LRItem<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rule.hash(state);
        self.dot_pos.hash(state);
    }
}

impl<'a, T: TokenTag> PartialEq for LRItem<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.rule == other.rule && self.dot_pos == other.dot_pos
    }
}

impl<'a, T: TokenTag> LRItem<'a, T> {
    fn new(rule: &'a Rule<T>, la_tokens: HashSet<&'a RuleElem<T>>) -> LRItem<'a, T> {
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

    fn expand_closure<'b>(
        &self,
        ruleset: &'a RuleSet<T>,
        first_set: &'b HashMap<&'a RuleElem<T>, Vec<&'a RuleElem<T>>>,
    ) -> HashSet<LRItem<'a, T>> {
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
                .map(|rule| LRItem::<'_,  _>::new(rule, af_la_tokens.clone()))
                .collect()
        } else {
            HashSet::new()
        }
    }

    #[allow(clippy::int_plus_one)]
    fn next_dot(&self) -> Option<(&'a RuleElem<T>, LRItem<'a, T>)> {
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

    fn unify(&mut self, other: LRItem<'a, T>) {
        if self != &other {
            return;
        }
        other.la_tokens.into_iter().for_each(|la_token| {
            if !self.la_tokens.contains(&la_token) {
                self.la_tokens.insert(la_token);
            }
        });
    }

    fn unify_all(mut items: Vec<LRItem<'a, T>>) -> Vec<LRItem<'a, T>> {
        for idx in (0..items.len()).permutations(2) {
            let (a_idx, b_idx) = (idx[0], idx[1]);
            let tmp = items[b_idx].clone();
            items[a_idx].unify(tmp);
        }
        items
    }

    fn unity_set(
        items_a: HashSet<LRItem<'a, T>>,
        items_b: HashSet<LRItem<'a, T>>,
    ) -> HashSet<LRItem<'a, T>> {
        let mut items_a = Vec::from_iter(items_a);
        let items_b = Vec::from_iter(items_b);
        items_a.extend(items_b);
        HashSet::from_iter(Self::unify_all(items_a))
    }
}
