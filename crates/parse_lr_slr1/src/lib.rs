#![feature(gen_blocks)]

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::rule::{Rule, RuleElem, RuleTag};
use copager_cfl::{CFLTokens, CFLRules};
use copager_parse::{BaseParser, ParseEvent};
use copager_parse_common::rule::FollowSet;
use copager_parse_lr_common::lr0::item::LR0Item;
use copager_parse_lr_common::lr0::LR0DFA;
use copager_parse_lr_common::{LRDriver, LRAction, LRTable, LRTableBuilder};
use copager_utils::cache::Cacheable;

pub struct SLR1<T, R>
where
    T: TokenTag,
    R: RuleTag<T>
{
    table: LRTable<T, R>,
}

impl<Ts, Rs> BaseParser<Ts, Rs> for SLR1<Ts::Tag, Rs::Tag>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    fn try_from((_, rules): (Ts, Rs)) -> anyhow::Result<Self> {
        let table = SLR1Table::try_from(rules)?;
        Ok(SLR1 { table })
    }

    gen fn run<'input, Il>(&self, mut lexer: Il) -> ParseEvent<'input, Ts::Tag, Rs::Tag>
    where
        Il: Iterator<Item = Token<'input, Ts::Tag>>,
    {
        let mut driver = LRDriver::from(&self.table);
        while !driver.accepted() {
            for event in driver.consume(lexer.next()).collect::<Vec<_>>() {
                yield event;
            }
        }
    }
}

impl<Ts, Rs> Cacheable<(Ts, Rs)> for SLR1<Ts::Tag, Rs::Tag>
where
    Ts: CFLTokens,
    Ts::Tag: Serialize + for<'de> Deserialize<'de>,
    Rs: CFLRules<Ts::Tag>,
    Rs::Tag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LRTable<Ts::Tag, Rs::Tag>;

    fn new((_, rules): (Ts, Rs)) -> anyhow::Result<Self::Cache> {
        let table = SLR1Table::try_from(rules)?;
        Ok(table)
    }

    fn restore(table: Self::Cache) -> Self {
        SLR1 { table }
    }
}

pub struct SLR1Table<T, R>
where
    T: TokenTag,
    R: RuleTag<T>
{
    _phantom_t: PhantomData<T>,
    _phantom_r: PhantomData<R>,
}

impl<T, R> SLR1Table<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    fn try_from<Rs>(rules: Rs) -> anyhow::Result<LRTable<T, R>>
    where
        Rs: CFLRules<T, Tag = R>,
    {
        // 最上位規則を追加して RuleSet を更新
        let mut ruleset = rules.into_ruleset();
        let top_dummy = Rule::new(
            None,
            RuleElem::new_nonterm("__top_dummy"),
            vec![RuleElem::new_nonterm(&ruleset.top)],
        );
        ruleset.update_top(top_dummy.clone());

        // Follow 集合作成
        let follow_set = FollowSet::from(&ruleset);

        // LR(0) オートマトン作成
        let dfa = LR0DFA::from(&ruleset);

        // SLR(1) 構文解析表作成
        let mut builder = LRTableBuilder::from(&dfa);
        for node in dfa.nodes {
            let node = node.read().unwrap();

            // A -> α β . を含む場合，Follow(A) 列に対して Reduce をマーク
            for rule in node.find_all_by(is_slr1_reduce_state) {
                let lhs = lhs_as_str(&rule.lhs);
                for term in follow_set.get(lhs).unwrap() {
                    match term {
                        RuleElem::Term(term) => {
                            builder.try_set(node.id, Some(*term), LRAction::Reduce(rule.clone()))?;
                        }
                        RuleElem::EOF => {
                            builder.try_set(node.id, None, LRAction::Reduce(rule.clone()))?;
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

fn is_slr1_reduce_state<T, R>(item: &&LR0Item<T, R>) -> bool
where
    T: TokenTag,
    R: RuleTag<T>,
{
    item.check_next_elem().is_none()
}

fn lhs_as_str<T: TokenTag>(lhs: &RuleElem<T>) -> &str {
    if let RuleElem::NonTerm(nt) = lhs {
        nt.as_str()
    } else {
        unreachable!()
    }
}
