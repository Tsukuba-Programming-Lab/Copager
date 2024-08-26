use std::marker::PhantomData;

use crate::TokenKind;

#[derive(Debug, Copy, Clone)]
pub struct Token<'a, 'b, T: TokenKind<'a>> {
    pub kind: T,
    pub src: &'b str,
    pub range: (usize, usize),
    _phantom: PhantomData<&'a ()>,
}

impl<'a, 'b, T: TokenKind<'a>> Token<'a, 'b, T> {
    pub fn new(kind: T, src: &'b str, range: (usize, usize)) -> Self {
        Token {
            kind,
            src,
            range,
            _phantom: PhantomData,
        }
    }

    pub fn as_str(&self) -> &'b str {
        let (l, r) = self.range;
        &self.src[l..r]
    }
}
