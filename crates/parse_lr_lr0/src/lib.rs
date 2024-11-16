#![feature(gen_blocks)]

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::rule::{Rule, RuleElem, RuleTag};
use copager_lex::LexSource;
use copager_parse::{BaseParser, ParseSource, ParseEvent};
use copager_parse_lr_common::lr0::item::LR0Item;
use copager_parse_lr_common::lr0::LR0DFA;
use copager_parse_lr_common::{LRDriver, LRAction, LRTable, LRTableBuilder};
use copager_utils::cache::Cacheable;

pub struct LR0<T, R>
where
    T: TokenTag,
    R: RuleTag<T>
{
    table: LRTable<T, R>,
}

impl<Sl, Sp> BaseParser<Sl, Sp> for LR0<Sl::Tag, Sp::Tag>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    fn try_from((source_l, source_p): (Sl, Sp)) -> anyhow::Result<Self> {
        let table = LR0Table::try_from(source_l, source_p)?;
        Ok(LR0 { table })
    }

    gen fn run<'input, Il>(&self, mut lexer: Il) -> ParseEvent<'input, Sl::Tag, Sp::Tag>
    where
        Il: Iterator<Item = Token<'input, Sl::Tag>>,
    {
        let mut driver = LRDriver::from(&self.table);
        while !driver.accepted() {
            for event in driver.consume(lexer.next()).collect::<Vec<_>>() {
                yield event;
            }
        }
    }
}

impl<Sl, Sp> Cacheable<(Sl, Sp)> for LR0<Sl::Tag, Sp::Tag>
where
    Sl: LexSource,
    Sl::Tag: Serialize + for<'de> Deserialize<'de>,
    Sp: ParseSource<Sl::Tag>,
    Sp::Tag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LRTable<Sl::Tag, Sp::Tag>;

    fn new((source_l, source_p): (Sl, Sp)) -> anyhow::Result<Self::Cache> {
        let table = LR0Table::try_from(source_l, source_p)?;
        Ok(table)
    }

    fn restore(table: Self::Cache) -> Self {
        LR0 { table }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LR0Table<T, R>
where
    T: TokenTag,
    R: RuleTag<T>
{
    _phantom_t: PhantomData<T>,
    _phantom_r: PhantomData<R>,
}

impl<T, R> LR0Table<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn try_from<Sl, Sp>(source_l: Sl, source_p: Sp) -> anyhow::Result<LRTable<T, R>>
    where
        Sl: LexSource<Tag = T>,
        Sp: ParseSource<T, Tag = R>,
    {
        // 最上位規則を追加して RuleSet を更新
        let mut ruleset = source_p.into_ruleset();
        let top_dummy = Rule::new(
            None,
            RuleElem::new_nonterm("__top_dummy"),
            vec![RuleElem::new_nonterm(&ruleset.top)],
        );
        ruleset.update_top(top_dummy.clone());

        // LR(0) オートマトン作成
        let dfa = LR0DFA::from(&ruleset);

        // LR(0) 構文解析表作成
        let mut builder = LRTableBuilder::from(&dfa);
        for node in dfa.nodes {
            let node = node.read().unwrap();
            for rule in node.find_all_by(is_lr0_reduce_state) {
                // S -> Top . を含む場合，EOF 列に対して Accept をマーク
                if rule == &top_dummy {
                    builder.set(node.id, None, LRAction::Accept);
                    continue;
                }

                // A -> α β . を含む場合 全列に Reduce をマーク
                builder.try_set(node.id, None, LRAction::Reduce(rule.clone()))?;
                for token in source_l.iter() {
                    builder.try_set(node.id, Some(token), LRAction::Reduce(rule.clone()))?;
                }
            }
        }
        let table = builder.build();

        Ok(table)
    }
}

fn is_lr0_reduce_state<T, R>(item: &&LR0Item<T, R>) -> bool
where
    T: TokenTag,
    R: RuleTag<T>,
{
    item.check_next_elem().is_none()
}
