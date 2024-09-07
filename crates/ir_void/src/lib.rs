use std::fmt::Debug;

use copager_cfg::token::{TokenTag, Token};
use copager_cfg::rule::RuleTag;
use copager_ir::{IR, IRBuilder};

#[derive(Debug)]
pub struct Void;

impl<'input, T, R> IR<'input, T, R> for Void
where
    T: TokenTag,
    R: RuleTag<T>,
{
    type Builder = Self;
}

impl <'input, T, R> IRBuilder<'input, T, R> for Void
where
    T: TokenTag,
    R: RuleTag<T>,
{
    type Output = Self;

    fn new() -> Void {
        Void
    }

    fn on_read(&mut self, _: Token<'input, T>) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_parse(&mut self, _: R) -> anyhow::Result<()> {
        Ok(())
    }

    fn build(self) -> anyhow::Result<Void> {
        Ok(Void)
    }
}
