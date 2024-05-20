use pgen_core::cfg::{TokenSet, Syntax};
use pgen_core::lex::Token;
use pgen_core::parse::{SExp, SExpBuilder};

use crate::error::ParseError;
use crate::builder::{LRAction, LR1Configure};

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
        lexer: &mut impl Iterator<Item = Token<'a, 'c, T>>,
    ) -> anyhow::Result<SExp<'a, 'c, T, S>> {
        let mut stack = vec![0];
        let mut builder = SExpBuilder::new();
        loop {
            let input = lexer.next();
            loop {
                let top = stack[stack.len() - 1];
                let action = match input {
                    Some(token) => (
                        self.0.action_table[top].get(&token.kind).unwrap(),
                        Some(token),
                    ),
                    None => (
                        &self.0.eof_action_table[top],
                        None
                    ),
                };
                match action {
                    (LRAction::Shift(new_state), Some(token)) => {
                        stack.push(*new_state);
                        builder.push(token);
                        break;
                    }
                    (LRAction::Reduce(tag, goto, elems_cnt), _) => {
                        stack.truncate(stack.len() - elems_cnt);
                        stack.push(self.0.goto_table[stack[stack.len() - 1]][*goto]);
                        builder.wrap(*tag, *elems_cnt);
                    }
                    (LRAction::Accept, _) => {
                        return builder.build();
                    }
                    (LRAction::None, Some(token)) => {
                        return Err(ParseError::new_unexpected_token(token).into());
                    }
                    (LRAction::None, None) => {
                        return Err(ParseError::UnexpectedEOF.into());
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
