use copager_cfg::token::{TokenTag, Token};
use copager_cfg::rule::{RuleElem, RuleTag};
use copager_parse::ParseEvent;

use crate::table::{LRAction, LRTable};

pub struct LRDriver<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    table: LRTable<T, R>,
    stack: Vec<usize>,
}

impl<T, R> From<LRTable<T, R>> for LRDriver<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn from(table: LRTable<T, R>) -> Self {
        LRDriver {
            table,
            stack: vec![0],
        }
    }
}

impl<'input, T, R> LRDriver<T, R>
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
            match (self.table.get_action(top, token), token) {
                (LRAction::Shift(new_state), Some(token)) => {
                    self.stack.push(*new_state);
                    yield ParseEvent::Read(token);
                    break;
                },
                (LRAction::Reduce(tag, rule), _) => {
                    let lhs = lhs_as_str(&rule.lhs);
                    let rhs_len = rule.rhs.len();
                    self.stack.truncate(self.stack.len() - rhs_len);
                    self.stack.push(self.table.get_goto(self.stack.len()-1, lhs).unwrap());
                    yield ParseEvent::Parse { rule: *tag, len: rhs_len };
                },
                (LRAction::Accept, _) => {
                    return;
                }
                (LRAction::None, Some(_)) => {
                    // TODO
                    // yield ParseEvent::Err(ParseError::new_unexpected_token(token).into());
                    yield ParseEvent::Err(anyhow::anyhow!("unexpected token").into());
                    return;
                }
                (LRAction::None, None) => {
                    // TODO
                    // yield ParseEvent::Err(ParseError::UnexpectedEOF.into());
                    yield ParseEvent::Err(anyhow::anyhow!("unexpected EOF").into());
                    return;
                }
                _ => unreachable!(),
            }
        }
    }
}

fn lhs_as_str<T: TokenTag>(lhs: &RuleElem<T>) -> &str {
    if let RuleElem::NonTerm(nt) = lhs {
        nt.as_str()
    } else {
        unreachable!()
    }
}
