use copager_cfg::token::TokenTag;
use copager_cfg::rule::{RuleTag, RuleSet};
use copager_lex::LexIterator;

pub trait ParseSource<T: TokenTag> {
    type Tag: RuleTag<TokenTag = T>;

    fn iter(&self) -> impl Iterator<Item = Self::Tag>;

    fn into_ruleset(&self) -> RuleSet<T> {
        let set_id_for_all = |(id, tag): (usize, Self::Tag)| {
            tag.as_rules()
                .into_iter()
                .map(move |rule| {
                    let mut rule = rule.clone();
                    rule.id = id;
                    rule
                })
        };

        self.iter()
            .enumerate()
            .flat_map(set_id_for_all)
            .collect::<RuleSet<_>>()
    }
}

pub trait ParseIterator<'input, T, R, Il>
where
    Self: From<Self::From>,
    T: TokenTag,
    R: RuleTag,
    Il: LexIterator<'input, T>,
{
    type From;

    fn init(&self, lexer: Il) -> Self;
    fn next(&mut self) -> Option<()>;
}
