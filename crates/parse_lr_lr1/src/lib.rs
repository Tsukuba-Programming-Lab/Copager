#![feature(gen_blocks)]

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use copager_cfg::token::{Token, TokenTag};
use copager_cfg::rule::{Rule, RuleElem, RuleTag};
use copager_lex::LexSource;
use copager_parse::{BaseParser, ParseSource, ParseEvent};
use copager_parse_common::rule::FirstSet;
use copager_parse_lr_common::lr1::item::LR1Item;
use copager_parse_lr_common::lr1::LR1DFA;
use copager_parse_lr_common::{LRDriver, LRAction, LRTable, LRTableBuilder};
use copager_utils::cache::Cacheable;

pub struct LR1<T, R>
where
    T: TokenTag,
    R: RuleTag<T>
{
    table: LRTable<T, R>,
}

impl<Sl, Sp> BaseParser<Sl, Sp> for LR1<Sl::Tag, Sp::Tag>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    fn try_from((_, source_p): (Sl, Sp)) -> anyhow::Result<Self> {
        let table = LR1Table::try_from(source_p)?;
        Ok(LR1 { table })
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

impl<Sl, Sp> Cacheable<(Sl, Sp)> for LR1<Sl::Tag, Sp::Tag>
where
    Sl: LexSource,
    Sl::Tag: Serialize + for<'de> Deserialize<'de>,
    Sp: ParseSource<Sl::Tag>,
    Sp::Tag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LRTable<Sl::Tag, Sp::Tag>;

    fn new((_, source_p): (Sl, Sp)) -> anyhow::Result<Self::Cache> {
        let table = LR1Table::try_from(source_p)?;
        Ok(table)
    }

    fn restore(table: Self::Cache) -> Self {
        LR1 { table }
    }
}

pub struct LR1Table<T, R>
where
    T: TokenTag,
    R: RuleTag<T>
{
    _phantom_t: PhantomData<T>,
    _phantom_r: PhantomData<R>,
}

impl<T, R> LR1Table<T, R>
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
                        builder.try_set(node.id, Some(*term), LRAction::Reduce(rule.clone()))?;
                    }
                    RuleElem::EOF => {
                        builder.try_set(node.id, None, LRAction::Reduce(rule.clone()))?;
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
