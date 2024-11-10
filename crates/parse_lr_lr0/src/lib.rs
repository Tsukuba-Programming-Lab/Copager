#![feature(gen_blocks)]

use copager_cfg::token::{Token, TokenTag};
use copager_cfg::rule::RuleTag;
use copager_lex::LexSource;
use copager_parse::{ParseDriver, ParseSource, ParseEvent};
use copager_parse_lr_common::lr0::LR0DFA;
use copager_parse_lr_common::table::{LRTable, LRTableBuilder};
use copager_parse_lr_common::driver::LRDriver;

pub struct LR0<T, R>
where
    T: TokenTag,
    R: RuleTag<T>
{
    table: LRTable<T, R>,
}

impl<Sl, Sp> ParseDriver<Sl, Sp> for LR0<Sl::Tag, Sp::Tag>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    fn try_from((_, source_p): (Sl, Sp)) -> anyhow::Result<Self> {
        let ruleset = source_p.into_ruleset();
        let lr0_dfa = LR0DFA::from(&ruleset);
        let lr_table = LRTableBuilder::from(&lr0_dfa).build();

        Ok(LR0 { table: lr_table })
    }

    gen fn run<'input, Il>(&self, mut lexer: Il) -> ParseEvent<'input, Sl::Tag, Sp::Tag>
    where
        Il: Iterator<Item = Token<'input, Sl::Tag>>,
    {
        let mut driver = LRDriver::from(&self.table);
        for event in driver.consume(lexer.next()).collect::<Vec<_>>() {
            yield event;
        }
    }
}
