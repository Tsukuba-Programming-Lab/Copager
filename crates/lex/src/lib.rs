use std::marker::PhantomData;

use copager_cfg::token::{TokenTag, Token};
use copager_utils::cache::Cacheable;

pub struct Lexer<'cache, 'input, T, S, I>
where
    T: TokenTag,
    S: LexSource<T>,
    I: LexIterator<'cache, 'input, T, S>,
{
    cache: I::Cache,
    _phantom_s: PhantomData<S>,
    _phantom_t: PhantomData<&'input T>,
}

impl<'cache, 'input, T, S, I> Lexer<'cache, 'input, T, S, I>
where
    T: TokenTag,
    S: LexSource<T>,
    I: LexIterator<'cache, 'input, T, S>,
{
    pub fn new() -> anyhow::Result<Self>
    where
        S: Default,
    {
        Self::try_from(S::default())
    }

    pub fn try_from(source: S) -> anyhow::Result<Self> {
        Ok(Lexer {
            cache: I::new(source)?,
            _phantom_s: PhantomData,
            _phantom_t: PhantomData,
        })
    }

    pub fn iter(&'cache self, input: &'input str) -> I {
        I::restore(&self.cache).init(input)
    }
}

pub trait LexSource<T>
where
    T: TokenTag,
{
    fn ignore_token(&self) -> &str;
    fn iter(&self) -> impl Iterator<Item = T>;
}

pub trait LexIterator<'cache, 'input, T, S>
where
    Self: Sized + Cacheable<'cache, S>,
    T: TokenTag,
    S: LexSource<T>,
{
    fn init(&self, input: &'input str) -> Self;
    fn next(&mut self) -> Option<Token<'input, T>>;
}
