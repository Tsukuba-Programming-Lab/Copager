use std::fmt::Debug;
use std::hash::Hash;

use serde::{Serialize, Deserialize};

pub trait TokenTag
where
    Self: Clone + Hash + Eq,
{
    fn as_str_list<'a, 'b>(&'a self) -> &'a[&'b str];
    fn as_option_list<'a, 'b>(&'a self) -> &'a[&'b str] { &[] }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token<'input, T: TokenTag> {
    pub kind: T,
    pub src: &'input str,
    pub body: (usize, usize),  // Trivia を含まない
    pub full: (usize, usize),  // Trivia を含む
}

impl<'input, T: TokenTag> Token<'input, T> {
    pub fn new(
        kind: T,
        src: &'input str,
        body: (usize, usize),
        full: (usize, usize),
    ) -> Self {
        Token { kind, src, body, full }
    }

    pub fn as_str(&self) -> &'input str {
        let (l, r) = self.body;
        &self.src[l..r]
    }

    pub fn as_full_str(&self) -> &'input str {
        let (l, r) = self.full;
        &self.src[l..r]
    }
}

#[cfg(feature = "derive")]
pub use copager_cfl_derive::TokenSet;

pub trait TokenSet {
    type Tag: TokenTag;

    fn instantiate() -> Self;
    fn iter(&self) -> impl Iterator<Item = Self::Tag>;
}
