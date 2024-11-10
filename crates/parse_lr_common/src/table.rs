use std::collections::HashMap;

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{Rule, RuleElem};

use crate::automaton::Automaton;

#[derive(Debug)]
pub enum LRAction<T: TokenTag> {
    Shift(usize),
    Reduce(usize, usize, Rule<T>), // goto_id, elems_cnt, rule
    Accept,
    None,
}

#[derive(Debug)]
pub struct LRTable<T: TokenTag> {    // R = Rule<T>
    pub action_table: Vec<HashMap<T, LRAction<T>>>,
    pub eof_action_table: Vec<LRAction<T>>,
    pub goto_table: Vec<HashMap<String, usize>>,
}

impl<T: TokenTag> LRTable<T>  {
    pub fn get_action(&self, state: usize, token: T) -> &LRAction<T> {
        self.action_table[state].get(&token).unwrap_or(&LRAction::None)
    }

    pub fn get_eof_action(&self, state: usize) -> &LRAction<T> {
        &self.eof_action_table[state]
    }

    pub fn get_goto(&self, state: usize, nonterm: &str) -> Option<usize> {
        self.goto_table[state].get(nonterm).copied()
    }
}

#[derive(Debug)]
pub struct LRTableBuilder<T: TokenTag> {
    action_table: Vec<HashMap<T, LRAction<T>>>,
    eof_action_table: Vec<LRAction<T>>,
    goto_table: Vec<HashMap<String, usize>>,
}

impl<'a: 'b, 'b, T> LRTableBuilder<T>
where
    T: TokenTag + 'a,
{
    pub fn from<A>(automaton: &'b impl Automaton<'a, 'b, T>) -> Self {
        let size = automaton.len();

        // 初期化
        let mut action_table: Vec<HashMap<T, LRAction<T>>> = Vec::with_capacity(size);
        let mut eof_action_table: Vec<LRAction<T>> = Vec::with_capacity(size);
        let mut goto_table = Vec::with_capacity(size);
        for _ in 0..size {
            action_table.push(HashMap::new());
            eof_action_table.push(LRAction::None);
            goto_table.push(HashMap::new());
        }

        // 表の作成
        for (from, to, elem) in automaton.edges() {
            match elem {
                RuleElem::Term(token) => {
                    action_table[*from].insert(*token, LRAction::Shift(*to));
                }
                RuleElem::NonTerm(name) => {
                    goto_table[*from].insert(name.clone(), *to);
                },
                _ => {}
            }
        }

        LRTableBuilder {
            action_table,
            eof_action_table,
            goto_table,
        }
    }

    pub fn build(self) -> LRTable<T> {
        LRTable {
            action_table: self.action_table,
            eof_action_table: self.eof_action_table,
            goto_table: self.goto_table,
        }
    }
}

#[cfg(test)]
mod test {
    // TODO
}
