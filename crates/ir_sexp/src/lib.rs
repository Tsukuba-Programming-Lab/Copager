use std::fmt::{Debug, Display};

use copager_cfg::token::Token;
use copager_cfg::{RuleKind, TokenKind};
use copager_ir::{IR, IRBuilder};

#[derive(Debug)]
pub enum SExp<'a, 'b, T, S>
where
    T: TokenKind<'a> + 'a,
    S: RuleKind<'a, TokenKind = T>,
{
    List {
        tag: S,
        elems: Vec<SExp<'a, 'b, T, S>>,
    },
    Atom(Token<'a, 'b, T>),
}

impl<'a, T, S> Display for SExp<'a, '_, T, S>
where
    T: TokenKind<'a> + 'a,
    S: RuleKind<'a, TokenKind = T> + Debug,
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

impl<'a, 'b, T, R> IR<'a, T, R> for SExp<'a, 'b, T, R>
where
    T: TokenKind<'a> + 'a,
    R: RuleKind<'a, TokenKind = T>,
{
    type Builder = SExpBuilder<'a, 'b, T, R>;
}

#[derive(Debug)]
pub struct SExpBuilder<'a, 'b, T, R>
where
    T: TokenKind<'a> + 'a,
    R: RuleKind<'a, TokenKind = T>,
{
    stack: Vec<SExp<'a, 'b, T, R>>,
}

impl <'a, 'b, T, R> IRBuilder<'a> for SExpBuilder<'a, 'b, T, R>
where
    T: TokenKind<'a> + 'a,
    R: RuleKind<'a, TokenKind = T>,
{
    type TokenKind = T;
    type RuleKind = R;
    type Output = SExp<'a, 'b, T, R>;

    fn new() -> SExpBuilder<'a, 'b, T, R> {
        SExpBuilder { stack: vec![] }
    }

    fn build(mut self) -> anyhow::Result<SExp<'a, 'b, T, R>> {
        if self.stack.len() == 1 {
            Ok(self.stack.pop().unwrap())
        } else {
            Err(anyhow::anyhow!("Invalid S-Expression"))
        }
    }
}

impl<'a, 'b, T, R> SExpBuilder<'a, 'b, T, R>
where
    T: TokenKind<'a> + 'a,
    R: RuleKind<'a, TokenKind = T>,
{
    pub fn push(&mut self, token: Token<'a, 'b, T>) {
        self.stack.push(SExp::Atom(token));
    }

    pub fn wrap(&mut self, tag: R, cnt: usize) {
        let elems = self.stack.split_off(self.stack.len() - cnt);
        self.stack.push(SExp::List { tag, elems });
    }
}
