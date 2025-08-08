#![feature(gen_blocks)]

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::rule::{Rule, RuleElem, RuleTag};
use copager_cfl::{CFL, CFLRule};
use copager_parse::{BaseParser, ParseEvent};
use copager_parse_common::rule::FirstSet;
use copager_parse_lr_common::lr1::LR1DFA;
use copager_parse_lr_common::lalr1::item::LALR1Item;
use copager_parse_lr_common::lalr1::LALR1DFA;
use copager_parse_lr_common::{LRDriver, LRAction, LRTable, LRTableBuilder};
use copager_utils::cache::Cacheable;

pub struct LALR1<Lang: CFL> {
    table: LRTable<Lang::TokenTag, Lang::RuleTag>,
}

impl<Lang: CFL> BaseParser<Lang> for LALR1<Lang> {
    fn init() -> anyhow::Result<Self> {
        Ok(LALR1 {
            table: LALR1Table::<Lang>::init()?,
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

impl<Lang> Cacheable<()> for LALR1<Lang>
where
    Lang: CFL,
    Lang::TokenTag: Serialize + for<'de> Deserialize<'de>,
    Lang::RuleTag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LRTable<Lang::TokenTag, Lang::RuleTag>;

    fn cache(_: ()) -> anyhow::Result<Self::Cache> {
        Ok(LALR1Table::<Lang>::init()?)
    }

    fn restore(table: Self::Cache) -> Self {
        LALR1 { table }
    }
}

pub struct LALR1Table<Lang: CFL> {
    _phantom: PhantomData<Lang>,
}

impl<Lang: CFL> LALR1Table<Lang> {
    pub fn init() -> anyhow::Result<LRTable<Lang::TokenTag, Lang::RuleTag>> {
        // Rules 準備
        let ruleset = Lang::RuleSet::instantiate();

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

        // LALR(1) オートマトン作成
        let dfa = LR1DFA::from((&ruleset, &first_set));
        let dfa = LALR1DFA::from(dfa);

        // LALR(1) 構文解析表作成
        let mut builder = LRTableBuilder::from(&dfa);
        for node in &dfa.nodes {
            for (rule, la_tokens) in node.find_all_by(is_lalr1_reduce_state) {
                // A -> α β . [la_token] を含む場合，la_token 列に対して Reduce をマーク
                for la_token in la_tokens {
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

fn is_lalr1_reduce_state<T, R>(item: &&LALR1Item<T, R>) -> bool
where
    T: TokenTag,
    R: RuleTag<T>,
{
    item.check_next_elem().is_none()
}
