use copager_cfl::token::{TokenTag, Token};
use copager_cfl::rule::{RuleElem, RuleTag};
use copager_parse::ParseEvent;

use crate::error::LRError;
use crate::table::{LRAction, LRTable};

pub struct LRDriver<'table, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    table: &'table LRTable<T, R>,
    stack: Vec<usize>,
    accepted: bool,
}

impl<'table, T, R> From<&'table LRTable<T, R>> for LRDriver<'table, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(table: &'table LRTable<T, R>) -> Self {
        LRDriver {
            table,
            stack: vec![0],
            accepted: false,
        }
    }
}

impl<'table, 'input, T, R> LRDriver<'table, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    pub fn reset(&mut self) {
        self.stack = vec![0];
    }

    pub gen fn consume(&mut self, token: Option<Token<'input, T>>) -> ParseEvent<'input, T, R> {
        loop {
            let top = self.stack[self.stack.len() - 1];
            let action = self.table.get_action(top, token);
            match (action, token) {
                (LRAction::Shift(new_state), Some(token)) => {
                    self.stack.push(*new_state);
                    yield ParseEvent::Read(token);
                    break;
                },
                (LRAction::Reduce(rule), _) => {
                    let tag = rule.tag.unwrap();
                    let lhs = lhs_as_str(&rule.lhs);
                    let rhs_len = rule.rhs.len();
                    self.stack.truncate(self.stack.len() - rhs_len);
                    self.stack.push(self.table.get_goto(self.stack[self.stack.len()-1], lhs).unwrap());
                    yield ParseEvent::Parse { rule: tag, len: rhs_len };
                },
                (LRAction::Accept, _) => {
                    self.accepted = true;
                    return;
                }
                (LRAction::None, Some(token)) => {
                    yield ParseEvent::Err(LRError::new_unexpected_token(token).into());
                    return;
                }
                (LRAction::None, None) => {
                    yield ParseEvent::Err(LRError::new_unexpected_eof().into());
                    return;
                }
                _ => unreachable!(),
            }
        }
    }

    pub fn accepted(&self) -> bool {
        self.accepted
    }
}

fn lhs_as_str<T: TokenTag>(lhs: &RuleElem<T>) -> &str {
    if let RuleElem::NonTerm(nt) = lhs {
        nt.as_str()
    } else {
        unreachable!()
    }
}
