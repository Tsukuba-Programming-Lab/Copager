use std::hash::Hash;

use regex::Regex;

pub trait TokenSet<'a>
where
    Self: Copy + Clone + Hash + Eq,
{
    fn ignore_str() -> &'a str;
    fn enum_iter() -> impl Iterator<Item = Self>;
    fn to_regex(&self) -> &'a str;

    fn try_into() -> anyhow::Result<Vec<(Regex, Self)>> {
        Self::enum_iter()
            .map(|token| Ok((Regex::new(Self::to_regex(&token))?, token)))
            .collect::<anyhow::Result<Vec<_>>>()
    }
}
