use std::fmt::Debug;

use copager_cfl::token::Token;
use copager_cfl::{CFLTokens, CFLRules};
use copager_ir::{IR, IRBuilder};

#[derive(Debug)]
pub struct Void;

impl<'input, Ts, Rs> IR<'input, Ts, Rs> for Void
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    type Builder = Self;
}

impl <'input, Ts, Rs> IRBuilder<'input, Ts, Rs> for Void
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    type Output = Self;

    fn new() -> Void {
        Void
    }

    fn on_read(&mut self, _: Token<'input, Ts::Tag>) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_parse(&mut self, _: Rs::Tag, _: usize) -> anyhow::Result<()> {
        Ok(())
    }

    fn build(self) -> anyhow::Result<Void> {
        Ok(Void)
    }
}
