#![feature(gen_blocks)]

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use copager_cfg::token::{Token, TokenTag};
use copager_cfg::rule::{Rule, RuleElem, RuleTag};
use copager_lex::LexSource;
use copager_parse::{ParseDriver, ParseSource, ParseEvent};
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

impl<Sl, Sp> ParseDriver<Sl, Sp> for SLR1<Sl::Tag, Sp::Tag>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    fn try_from((_, source_p): (Sl, Sp)) -> anyhow::Result<Self> {
        let table = SLR1Table::try_from(source_p)?;
        Ok(SLR1 { table })
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

impl<Sl, Sp> Cacheable<(Sl, Sp)> for SLR1<Sl::Tag, Sp::Tag>
where
    Sl: LexSource,
    Sl::Tag: Serialize + for<'de> Deserialize<'de>,
    Sp: ParseSource<Sl::Tag>,
    Sp::Tag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LRTable<Sl::Tag, Sp::Tag>;

    fn new((_, source_p): (Sl, Sp)) -> anyhow::Result<Self::Cache> {
        let table = SLR1Table::try_from(source_p)?;
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
    fn try_from<Sp>(source_p: Sp) -> anyhow::Result<LRTable<T, R>>
    where
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

        // Follow 集合作成
        let follow_set = FollowSet::from(&ruleset);

        // LR(0) オートマトン作成
        let dfa = LR0DFA::from(&ruleset);

        // SLR(1) 構文解析表作成
        let mut builder = LRTableBuilder::from(&dfa);
        for node in dfa.nodes {
            let node = node.read().unwrap();
            if let Some(rule) = node.find_all_by(is_slr1_reduce_state).next() {
                // S -> Top . を含むノードに対して Accept をマーク
                if let Some(_) = node.find_all(&top_dummy).next() {
                    builder.set(node.id, None, LRAction::Accept);
                    continue;
                }

                // A -> α β . を含むノードに対して Reduce をマーク
                let lhs = lhs_as_str(&rule.lhs);
                for term in follow_set.get(lhs).unwrap() {
                    match term {
                        RuleElem::Term(term) => {
                            builder.set(node.id, Some(*term), LRAction::Reduce(rule.clone()));
                        }
                        RuleElem::EOF => {
                            builder.set(node.id, None, LRAction::Reduce(rule.clone()));
                        }
                        _ => {}
                    }
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
