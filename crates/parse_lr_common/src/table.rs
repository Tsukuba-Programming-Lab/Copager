use std::collections::HashMap;

use copager_cfg::token::{Token, TokenTag};
use copager_cfg::rule::{Rule, RuleElem, RuleTag};

use crate::automaton::Automaton;

#[derive(Debug)]
pub enum LRAction<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    Shift(usize),
    Reduce(R, Rule<T>), // elems_cnt, rule
    Accept,
    None,
}

#[derive(Debug)]
pub struct LRTable<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    action_table: Vec<HashMap<T, LRAction<T, R>>>,
    eof_action_table: Vec<LRAction<T, R>>,
    goto_table: Vec<HashMap<String, usize>>,
}

impl<T, R> LRTable<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn get_action(&self, state: usize, token: Option<Token<T>>) -> &LRAction<T, R> {
        if let Some(token) = token {
            return &self.action_table[state].get(&token.kind).unwrap_or(&LRAction::None)
        } else {
            return &self.eof_action_table[state]
        }
    }

    pub fn get_goto(&self, state: usize, nonterm: &str) -> Option<usize> {
        self.goto_table[state].get(nonterm).copied()
    }
}

#[derive(Debug)]
pub struct LRTableBuilder<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub action_table: Vec<HashMap<T, LRAction<T, R>>>,
    pub eof_action_table: Vec<LRAction<T, R>>,
    pub goto_table: Vec<HashMap<String, usize>>,
}

impl<'a: 'b, 'b, T, R> LRTableBuilder<T, R>
where
    T: TokenTag + 'a,
    R: RuleTag<T>,
{
    pub fn from(automaton: &'b impl Automaton<'a, 'b, T>) -> Self {
        let size = automaton.len();

        // 初期化
        let mut action_table: Vec<HashMap<T, LRAction<T, R>>> = Vec::with_capacity(size);
        let mut eof_action_table = Vec::with_capacity(size);
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

    pub fn build(self) -> LRTable<T, R> {
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
