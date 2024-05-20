use std::fmt::{Display, Debug};

use crate::cfg::{TokenSet, Syntax};
use crate::lex::Token;

pub trait ParserImpl<'a>
where
    Self: Sized,
{
    type TokenSet: TokenSet<'a> + 'a;
    type Syntax: Syntax<'a, TokenSet = Self::TokenSet>;

    fn setup() -> anyhow::Result<Self>;
    fn parse<'b>(
        &self,
        lexer: impl Iterator<Item = Token<'a, 'b, Self::TokenSet>>,
    ) -> anyhow::Result<SExp<'a, 'b, Self::TokenSet, Self::Syntax>>;
}

#[derive(Debug)]
pub enum SExp<'a, 'b, T, S>
where
    T: TokenSet<'a> + 'a,
    S: Syntax<'a, TokenSet = T>,
{
    List {
        tag: S,
        elems: Vec<SExp<'a, 'b, T, S>>,
    },
    Atom(Token<'a, 'b, T>),
}

impl<'a, T, S> Display for SExp<'a, '_, T, S>
where
    T: TokenSet<'a> + 'a,
    S: Syntax<'a, TokenSet = T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SExp::List { tag, elems } => {
                write!(f, "({:?}", tag)?;
                for elem in elems {
                    write!(f, " {}", elem)?;
                }
                write!(f, ")")
            }
            SExp::Atom(token) => write!(f, "{:?}", token.as_str()),
        }
    }
}

#[derive(Debug)]
pub struct SExpBuilder<'a, 'b, T, S>
where
    T: TokenSet<'a> + 'a,
    S: Syntax<'a, TokenSet = T>,
{
    stack: Vec<SExp<'a, 'b, T, S>>,
}

impl<'a, 'b, T, S> SExpBuilder<'a, 'b, T, S>
where
    T: TokenSet<'a> + 'a,
    S: Syntax<'a, TokenSet = T>,
{
    pub fn new() -> SExpBuilder<'a, 'b, T, S> {
        SExpBuilder { stack: vec![] }
    }

    pub fn push(&mut self, token: Token<'a, 'b, T>) {
        self.stack.push(SExp::Atom(token));
    }

    pub fn wrap(&mut self, tag: S, cnt: usize) {
        let elems = self.stack.split_off(self.stack.len() - cnt);
        self.stack.push(SExp::List { tag, elems });
    }

    pub fn build(mut self) -> anyhow::Result<SExp<'a, 'b, T, S>> {
        if self.stack.len() == 1 {
            Ok(self.stack.pop().unwrap())
        } else {
            Err(anyhow::anyhow!("Invalid S-Expression"))
        }
    }
}
