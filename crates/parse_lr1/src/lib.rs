#![feature(gen_blocks)]

mod error;
mod builder;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use copager_cfg::token::Token;
use copager_lex::LexSource;
use copager_parse::{ParseSource, ParseDriver, ParseEvent};
use copager_utils::cache::Cacheable;

use builder::{LR1Configure, LRAction};
use error::ParseError;

#[derive(Debug)]
pub struct LR1<Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    tables: LR1Configure<Sl, Sp>,
}

impl<Sl, Sp> Cacheable<(Sl, Sp)> for LR1<Sl, Sp>
where
    Sl: LexSource,
    Sl::Tag: Serialize + for<'de> Deserialize<'de>,
    Sp: ParseSource<Sl::Tag>,
    Sp::Tag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LR1Configure<Sl, Sp>;

    fn new((source_l, source_p): (Sl, Sp)) -> anyhow::Result<Self::Cache> {
        Ok(LR1Configure::new(&source_l, &source_p)?)
    }

    fn restore(tables: Self::Cache) -> Self {
        LR1 { tables }
    }
}

impl<Sl, Sp> ParseDriver<Sl, Sp> for LR1<Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    fn try_from((source_l, source_p): (Sl, Sp)) -> anyhow::Result<Self> {
        let tables = LR1Configure::new(&source_l, &source_p)?;
        Ok(LR1 { tables })
    }

    gen fn run<'input, Il>(&self, mut lexer: Il) -> ParseEvent<'input, Sl::Tag, Sp::Tag>
    where
        Il: Iterator<Item = Token<'input, Sl::Tag>>,
    {
        let mut stack = vec![0];
        loop {
            let token = lexer.next();
            loop {
                let top = stack[stack.len() - 1];
                let action = match token {
                    Some(token) => {
                        let local_action_table: &HashMap<_, _> = &self.tables.action_table[top];
                        (local_action_table.get(&token.kind).unwrap(), Some(token))
                    },
                    None => (&self.tables.eof_action_table[top], None),
                };
                match action {
                    (LRAction::Shift(new_state), Some(token)) => {
                        stack.push(*new_state);
                        yield ParseEvent::Read(token);
                        break;
                    }
                    (LRAction::Reduce(tag, goto, elems_cnt), _) => {
                        stack.truncate(stack.len() - elems_cnt);
                        stack.push(self.tables.goto_table[stack[stack.len() - 1]][*goto]);
                        yield ParseEvent::Parse { rule: *tag, len: *elems_cnt };
                    }
                    (LRAction::Accept, _) => {
                        return;
                    }
                    (LRAction::None, Some(token)) => {
                        yield ParseEvent::Err(ParseError::new_unexpected_token(token).into());
                        return;
                    }
                    (LRAction::None, None) => {
                        yield ParseEvent::Err(ParseError::UnexpectedEOF.into());
                        return;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
