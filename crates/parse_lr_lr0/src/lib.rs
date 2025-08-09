#![feature(gen_blocks)]

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use copager_lang::token::{Token, TokenSet, TokenTag};
use copager_lang::rule::{Rule, RuleElem, RuleSet, RuleTag};
use copager_lang::Lang;
use copager_parse::{BaseParser, ParseEvent};
use copager_parse_lr_common::lr0::item::LR0Item;
use copager_parse_lr_common::lr0::LR0DFA;
use copager_parse_lr_common::{LRDriver, LRAction, LRTable, LRTableBuilder};
use copager_utils::cache::Cacheable;

pub struct LR0<L: Lang> {
    table: LRTable<L::TokenTag, L::RuleTag>,
}

impl<L: Lang> BaseParser<L> for LR0<L>{
    fn init() -> anyhow::Result<Self> {
        Ok(LR0 {
            table: LR0Table::<L>::init()?,
        })
    }

    gen fn run<'input, Il>(&self, mut lexer: Il) -> ParseEvent<'input, L::TokenTag, L::RuleTag>
    where
        Il: Iterator<Item = Token<'input, L::TokenTag>>,
    {
        let mut driver = LRDriver::from(&self.table);
        while !driver.accepted() {
            for event in driver.consume(lexer.next()).collect::<Vec<_>>() {
                yield event;
            }
        }
    }
}

impl<L> Cacheable<()> for LR0<L>
where
    L: Lang,
    L::TokenTag: Serialize + for<'de> Deserialize<'de>,
    L::RuleTag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LRTable<L::TokenTag, L::RuleTag>;

    fn cache(_: ()) -> anyhow::Result<Self::Cache> {
        Ok(LR0Table::<L>::init()?)
    }

    fn restore(table: Self::Cache) -> Self {
        LR0 { table }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LR0Table<L: Lang> {
    _phantom: PhantomData<L>,
}

impl<L: Lang> LR0Table<L> {
    pub fn init() -> anyhow::Result<LRTable<L::TokenTag, L::RuleTag>> {
        // Toks 準備
        let tokenset = L::TokenSet::instantiate();
        let ruleset = L::RuleSet::instantiate();

        // 最上位規則を追加して RuleSet を更新
        let mut ruleset = ruleset.into_ruleset();
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
                for token in tokenset.iter() {
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
