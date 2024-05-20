use std::fmt::Debug;
use std::hash::Hash;

use regex::{Regex, RegexSet};

pub trait TokenSet<'a>
where
    Self: Debug + Copy + Clone + Hash + Eq,
{
    fn ignore_str() -> &'a str;
    fn into_iter() -> impl Iterator<Item = Self>;
    fn into_regex_str(&self) -> &'a str;

    fn into_regex(&self) -> anyhow::Result<Regex> {
        Ok(Regex::new(self.into_regex_str())?)
    }

    fn try_into_regexset() -> anyhow::Result<RegexSet> {
        let regex_set = Self::into_iter()
            .map(|token| Self::into_regex_str(&token))
            .collect::<Vec<_>>();

        Ok(RegexSet::new(regex_set)?)
    }
}
