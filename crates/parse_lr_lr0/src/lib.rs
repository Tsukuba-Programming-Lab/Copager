#![feature(gen_blocks)]

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::rule::{Rule, RuleElem, RuleTag};
use copager_cfl::{CFL, CFLTokens, CFLRules};
use copager_parse::{BaseParser, ParseEvent};
use copager_parse_lr_common::lr0::item::LR0Item;
use copager_parse_lr_common::lr0::LR0DFA;
use copager_parse_lr_common::{LRDriver, LRAction, LRTable, LRTableBuilder};
use copager_utils::cache::Cacheable;

pub struct LR0<Lang: CFL> {
    table: LRTable<Lang::TokenTag, Lang::RuleTag>,
}

impl<Lang: CFL> BaseParser<Lang> for LR0<Lang>{
    fn try_from(cfl: &Lang) -> anyhow::Result<Self> {
        Ok(LR0 {
            table: LR0Table::try_from(cfl)?,
        })
    }

    gen fn run<'input, Il>(&self, mut lexer: Il) -> ParseEvent<'input, Lang::TokenTag, Lang::RuleTag>
    where
        Il: Iterator<Item = Token<'input, Lang::TokenTag>>,
    {
        let mut driver = LRDriver::from(&self.table);
        while !driver.accepted() {
            for event in driver.consume(lexer.next()).collect::<Vec<_>>() {
                yield event;
            }
        }
    }
}

impl<Lang> Cacheable<Lang> for LR0<Lang>
where
    Lang: CFL,
    Lang::TokenTag: Serialize + for<'de> Deserialize<'de>,
    Lang::RuleTag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LRTable<Lang::TokenTag, Lang::RuleTag>;

    fn new(cfl: Lang) -> anyhow::Result<Self::Cache> {
        Ok(LR0Table::try_from(&cfl)?)
    }

    fn restore(table: Self::Cache) -> Self {
        LR0 { table }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LR0Table<Lang: CFL> {
    _phantom: PhantomData<Lang>,
}

impl<Lang: CFL> LR0Table<Lang> {
    fn try_from(cfl: &Lang) -> anyhow::Result<LRTable<Lang::TokenTag, Lang::RuleTag>> {
        // Tokens，Rules 準備
        let tokens = cfl.instantiate_tokens();
        let rules = cfl.instantiate_rules();

        // 最上位規則を追加して RuleSet を更新
        let mut ruleset = rules.into_ruleset();
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
                for token in tokens.iter() {
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
