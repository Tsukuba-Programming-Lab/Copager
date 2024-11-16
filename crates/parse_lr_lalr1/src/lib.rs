#![feature(gen_blocks)]

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::rule::{Rule, RuleElem, RuleTag};
use copager_cfl::{CFLTokens, CFLRules};
use copager_parse::{BaseParser, ParseEvent};
use copager_parse_common::rule::FirstSet;
use copager_parse_lr_common::lr1::LR1DFA;
use copager_parse_lr_common::lalr1::item::LALR1Item;
use copager_parse_lr_common::lalr1::LALR1DFA;
use copager_parse_lr_common::{LRDriver, LRAction, LRTable, LRTableBuilder};
use copager_utils::cache::Cacheable;

pub struct LALR1<T, R>
where
    T: TokenTag,
    R: RuleTag<T>
{
    table: LRTable<T, R>,
}

impl<Ts, Rs> BaseParser<Ts, Rs> for LALR1<Ts::Tag, Rs::Tag>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    fn try_from((_, rules): (Ts, Rs)) -> anyhow::Result<Self> {
        let table = LALR1Table::try_from(rules)?;
        Ok(LALR1 { table })
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

impl<Ts, Rs> Cacheable<(Ts, Rs)> for LALR1<Ts::Tag, Rs::Tag>
where
    Ts: CFLTokens,
    Ts::Tag: Serialize + for<'de> Deserialize<'de>,
    Rs: CFLRules<Ts::Tag>,
    Rs::Tag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LRTable<Ts::Tag, Rs::Tag>;

    fn new((_, rules): (Ts, Rs)) -> anyhow::Result<Self::Cache> {
        let table = LALR1Table::try_from(rules)?;
        Ok(table)
    }

    fn restore(table: Self::Cache) -> Self {
        LALR1 { table }
    }
}

pub struct LALR1Table<T, R>
where
    T: TokenTag,
    R: RuleTag<T>
{
    _phantom_t: PhantomData<T>,
    _phantom_r: PhantomData<R>,
}

impl<T, R> LALR1Table<T, R>
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

fn is_lalr1_reduce_state<T, R>(item: &&LALR1Item<T, R>) -> bool
where
    T: TokenTag,
    R: RuleTag<T>,
{
    item.check_next_elem().is_none()
}
