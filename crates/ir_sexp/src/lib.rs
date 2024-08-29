use std::fmt::{Debug, Display};

use copager_cfg::token::{TokenTag, Token};
use copager_cfg::rule::RuleTag;
use copager_ir::{IR, IRBuilder};

#[derive(Debug)]
pub enum SExp<'input, T, S>
where
    T: TokenTag,
    S: RuleTag<TokenTag = T>,
{
    List {
        tag: S,
        elems: Vec<SExp<'input, T, S>>,
    },
    Atom(Token<'input, T>),
}

impl<T, S> Display for SExp<'_, T, S>
where
    T: TokenTag,
    S: RuleTag<TokenTag = T> + Debug,
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

impl<'input, T, R> IR<T, R> for SExp<'input, T, R>
where
    T: TokenTag,
    R: RuleTag<TokenTag = T>,
{
    type Builder = SExpBuilder<'input, T, R>;
}

#[derive(Debug)]
pub struct SExpBuilder<'input, T, R>
where
    T: TokenTag,
    R: RuleTag<TokenTag = T>,
{
    stack: Vec<SExp<'input, T, R>>,
}

impl <'input, T, R> IRBuilder<T, R> for SExpBuilder<'input, T, R>
where
    T: TokenTag,
    R: RuleTag<TokenTag = T>,
{
    type Output = SExp<'input, T, R>;

    fn new() -> SExpBuilder<'input, T, R> {
        SExpBuilder { stack: vec![] }
    }

    fn build(mut self) -> anyhow::Result<SExp<'input, T, R>> {
        if self.stack.len() == 1 {
            Ok(self.stack.pop().unwrap())
        } else {
            Err(anyhow::anyhow!("Invalid S-Expression"))
        }
    }
}

impl<'input, T, R> SExpBuilder<'input, T, R>
where
    T: TokenTag,
    R: RuleTag<TokenTag = T>,
{
    pub fn push(&mut self, token: Token<'input, T>) {
        self.stack.push(SExp::Atom(token));
    }

    pub fn wrap(&mut self, tag: R, cnt: usize) {
        let elems = self.stack.split_off(self.stack.len() - cnt);
        self.stack.push(SExp::List { tag, elems });
    }
}
