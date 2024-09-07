use std::fmt::{Debug, Display};

use copager_cfg::token::{TokenTag, Token};
use copager_cfg::rule::RuleTag;
use copager_ir::{IR, IRBuilder};

#[derive(Debug)]
pub enum SExp<'input, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    List {
        rule: R,
        elems: Vec<SExp<'input, T, R>>,
    },
    Atom(Token<'input, T>),
}

impl<T, R> Display for SExp<'_, T, R>
where
    T: TokenTag,
    R: RuleTag<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SExp::List { rule, elems } => {
                write!(f, "({:?}", rule)?;
                for elem in elems {
                    write!(f, " {}", elem)?;
                }
                write!(f, ")")
            }
            SExp::Atom(token) => write!(f, "{:?}", token.as_str()),
        }
    }
}

impl<'input, T, R> IR<'input, T, R> for SExp<'input, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    type Builder = SExpBuilder<'input, T, R>;
}

#[derive(Debug)]
pub struct SExpBuilder<'input, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    stack: Vec<SExp<'input, T, R>>,
}

impl <'input, T, R> IRBuilder<'input, T, R> for SExpBuilder<'input, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    type Output = SExp<'input, T, R>;

    fn new() -> SExpBuilder<'input, T, R> {
        SExpBuilder { stack: vec![] }
    }

    fn on_read(&mut self, token: Token<'input, T>) -> anyhow::Result<()> {
        self.stack.push(SExp::Atom(token));
        Ok(())
    }

    fn on_parse(&mut self, rule: R) -> anyhow::Result<()> {
        let elems = self.stack.split_off(self.stack.len() - rule.len());
        self.stack.push(SExp::List { rule, elems });
        Ok(())
    }

    fn build(mut self) -> anyhow::Result<SExp<'input, T, R>> {
        if self.stack.len() == 1 {
            Ok(self.stack.pop().unwrap())
        } else {
            Err(anyhow::anyhow!("Invalid S-Expression"))
        }
    }
}
