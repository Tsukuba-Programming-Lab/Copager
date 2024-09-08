use std::fmt::Debug;
use std::hash::Hash;

pub trait TokenTag
where
    Self: Debug + Copy + Clone + Hash + Eq,
{
    fn as_str<'a, 'b>(&'a self) -> &'b str;
}

#[derive(Debug, Copy, Clone)]
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
