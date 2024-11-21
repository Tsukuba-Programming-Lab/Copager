use std::fmt::Debug;
use std::hash::Hash;

use serde::{Serialize, Deserialize};

pub trait TokenTag
where
    Self: Debug + Copy + Clone + Hash + Eq,
{
    fn as_str_list<'a, 'b>(&'a self) -> &'a[&'b str];
    fn as_option_list<'a, 'b>(&'a self) -> &'a[&'b str] { &[] }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Token<'input, T: TokenTag> {
    pub kind: T,
    pub src: &'input str,
    pub range: (usize, usize),
}

impl<'input, T: TokenTag> Token<'input, T> {
    pub fn new(kind: T, src: &'input str, range: (usize, usize)) -> Self {
        Token { kind, src, range }
    }

    pub fn as_str(&self) -> &'input str {
        let (l, r) = self.range;
        &self.src[l..r]
    }
}
