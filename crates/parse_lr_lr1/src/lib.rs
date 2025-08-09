#![feature(gen_blocks)]

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use copager_lang::token::{Token, TokenTag};
use copager_lang::rule::{Rule, RuleElem, RuleSet, RuleTag};
use copager_lang::Lang;
use copager_parse::{BaseParser, ParseEvent};
use copager_parse_common::rule::FirstSet;
use copager_parse_lr_common::lr1::item::LR1Item;
use copager_parse_lr_common::lr1::LR1DFA;
use copager_parse_lr_common::{LRDriver, LRAction, LRTable, LRTableBuilder};
use copager_utils::cache::Cacheable;

pub struct LR1<L: Lang> {
    table: LRTable<L::TokenTag, L::RuleTag>,
}

impl<L: Lang> BaseParser<L> for LR1<L> {
    fn init() -> anyhow::Result<Self> {
        Ok(LR1 {
            table: LR1Table::<L>::init()?,
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

impl<L> Cacheable<()> for LR1<L>
where
    L: Lang,
    L::TokenTag: Serialize + for<'de> Deserialize<'de>,
    L::RuleTag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LRTable<L::TokenTag, L::RuleTag>;

    fn cache(_: ()) -> anyhow::Result<Self::Cache> {
        Ok(LR1Table::<L>::init()?)
    }

    fn restore(table: Self::Cache) -> Self {
        LR1 { table }
    }
}

pub struct LR1Table<L: Lang> {
    _phantom: PhantomData<L>,
}

impl<L: Lang> LR1Table<L> {
    pub fn init() -> anyhow::Result<LRTable<L::TokenTag, L::RuleTag>> {
        // Rules 準備
        let ruleset = L::RuleSet::instantiate();

        // 最上位規則を追加して RuleSet を更新
        let mut ruleset = ruleset.into_ruleset();
        let top_dummy = Rule::new(
            None,
            RuleElem::new_nonterm("__top_dummy"),
            vec![RuleElem::new_nonterm(&ruleset.top)],
        );
        ruleset.update_top(top_dummy.clone());

        // First 集合作成
        let first_set = FirstSet::from(&ruleset);

        // LR(1) オートマトン作成
        let dfa = LR1DFA::from((&ruleset, &first_set));

        // LR(1) 構文解析表作成
        let mut builder = LRTableBuilder::from(&dfa);
        for node in &dfa.nodes {
            let node = node.read().unwrap();
            for (rule, la_token) in node.find_all_by(is_lr1_reduce_state) {
                // A -> α β . [la_token] を含む場合，la_token 列に対して Reduce をマーク
                match la_token {
                    RuleElem::Term(term) => {
                        builder.try_set(
                            node.id,
                            Some(term.clone()),
                            LRAction::Reduce(rule.clone())
                        )?;
                    }
                    RuleElem::EOF => {
                        builder.try_set(
                            node.id,
                            None,
                            LRAction::Reduce(rule.clone())
                        )?;
                    }
                    _ => {}
                }

                // S -> Top . を含む場合，EOF 列に対して Accept をマーク
                if rule == &top_dummy {
                    builder.set(node.id, None, LRAction::Accept);
                }
            }
        }
        let table = builder.build();

        Ok(table)
    }
}

fn is_lr1_reduce_state<T, R>(item: &&LR1Item<T, R>) -> bool
where
    T: TokenTag,
    R: RuleTag<T>,
{
    item.check_next_elem().is_none()
}
