use std::marker::PhantomData;

use copager_cfg::token::Token;
use copager_cfg::TokenKind;

pub struct Lexer<'a, 'b, T, I>
where
    T: TokenKind<'a>,
    I: LexIterator<'a, 'b>,
{
    _phantom_t: PhantomData<&'a T>,
    _phantom_b: PhantomData<&'b str>,
    _phantom_itr: PhantomData<I>,
}

impl<'a, 'b, T, I> Lexer<'a, 'b, T, I>
where
    T: TokenKind<'a>,
    I: LexIterator<'a, 'b>,
{
    pub fn new(input: &'b str) -> anyhow::Result<impl LexIterator<'a, 'b>>
    where
        T: TokenKind<'a> + 'a,
    {
        I::try_from(input)
    }
}

pub trait LexIterator<'a, 'b>
where
    Self: Sized + TryFrom<&'b str, Error = anyhow::Error>,
{
    type TokenKind: TokenKind<'a>;

    fn next(&mut self) -> Option<Token<'a, 'b, Self::TokenKind>>;
}
