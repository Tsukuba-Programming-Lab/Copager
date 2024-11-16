use std::fmt::Debug;

use copager_cfl::token::Token;
use copager_cfl::{CFLTokens, CFLRules};
use copager_ir::{IR, IRBuilder};

#[derive(Debug)]
pub struct Void;

impl<'input, Sl, Sp> IR<'input, Sl, Sp> for Void
where
    Sl: CFLTokens,
    Sp: CFLRules<Sl::Tag>,
{
    type Builder = Self;
}

impl <'input, Sl, Sp> IRBuilder<'input, Sl, Sp> for Void
where
    Sl: CFLTokens,
    Sp: CFLRules<Sl::Tag>,
{
    type Output = Self;

    fn new() -> Void {
        Void
    }

    fn on_read(&mut self, _: Token<'input, Sl::Tag>) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_parse(&mut self, _: Sp::Tag, _: usize) -> anyhow::Result<()> {
        Ok(())
    }

    fn build(self) -> anyhow::Result<Void> {
        Ok(Void)
    }
}
