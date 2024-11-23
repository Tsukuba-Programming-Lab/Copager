use std::fmt::Debug;

use copager_cfl::token::Token;
use copager_cfl::CFL;
use copager_ir::{IR, IRBuilder};

#[derive(Debug)]
pub struct Void;

impl<'input, Lang: CFL> IR<'input, Lang> for Void {
    type Builder = Self;
}

impl <'input, Lang: CFL> IRBuilder<'input, Lang> for Void {
    type Output = Self;

    fn new() -> Void {
        Void
    }

    fn on_read(&mut self, _: Token<'input, Lang::TokenTag>) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_parse(&mut self, _: Lang::RuleTag, _: usize) -> anyhow::Result<()> {
        Ok(())
    }

    fn build(self) -> anyhow::Result<Void> {
        Ok(Void)
    }
}
