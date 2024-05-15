use core::cfg::{TokenSet, Syntax};
use core::lex::LexIterator;

use super::builder::{LRAction, LR1Configure};

pub(super) struct LR1Driver<'a, 'b, T, S> (&'b LR1Configure<'a, T, S>)
where
    T: TokenSet<'a> + 'a,
    S: Syntax<'a, TokenSet = T>;

impl<'a, 'b, T, S> LR1Driver<'a, 'b, T, S>
where
    T: TokenSet<'a> + 'a,
    S: Syntax<'a, TokenSet = T>,
{
    pub fn new(configure: &'b LR1Configure<'a, T, S>) -> LR1Driver<'a, 'b, T, S> {
        LR1Driver(configure)
    }

    pub fn run<'c>(
        &self,
        lexer: &mut impl LexIterator<'a, 'c, T>,
    ) -> anyhow::Result<()> {
        let mut stack = vec![0];
        loop {
            let input = lexer.next();
            loop {
                let top = stack[stack.len() - 1];
                let action = match input {
                    Some(token) => (
                        self.0.action_table[top].get(&token.kind).unwrap(),
                        Some(token.as_str()),
                    ),
                    None => (
                        &self.0.eof_action_table[top],
                        None
                    ),
                };
                match action.0 {
                    LRAction::Shift(new_state) => {
                        stack.push(*new_state);
                        break;
                    }
                    LRAction::Reduce(_, goto, elems_cnt) => {
                        stack.truncate(stack.len() - elems_cnt);
                        stack.push(self.0.goto_table[stack[stack.len() - 1]][*goto]);
                    }
                    LRAction::None => {
                        let pos = lexer.pos();
                        let pos = match action.1 {
                            Some(raw) => (pos.0, pos.1 - (raw.len() as u32)),
                            None => pos,
                        };
                        return Err(anyhow::anyhow!("Error at {:?}", pos).into());
                    }
                    LRAction::Accept => return Ok(()),
                }
            }
        }
    }
}
