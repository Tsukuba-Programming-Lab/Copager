use std::fmt::Debug;

use copager_lang::token::Token;
use copager_lang::Lang;
use copager_ir::{IR, IRBuilder};

#[derive(Debug)]
pub struct Void;

impl<'input, L: Lang> IR<'input, L> for Void {
    type Builder = Self;
}

impl <'input, L: Lang> IRBuilder<'input, L> for Void {
    type Output = Self;

    fn new() -> Void {
        Void
    }

    fn on_read(&mut self, _: Token<'input, L::TokenTag>) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_parse(&mut self, _: L::RuleTag, _: usize) -> anyhow::Result<()> {
        Ok(())
    }

    fn build(self) -> anyhow::Result<Void> {
        Ok(Void)
    }
}
