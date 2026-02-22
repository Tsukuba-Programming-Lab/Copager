use std::fmt::Debug;

use copager_lang::token::Token;
use copager_lang::Lang;
use copager_ir::{IR, IRBuilder};
use copager_utils::result::Result as CoResult;

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

    fn on_read(&mut self, _: Token<'input, L::TokenTag>) -> CoResult<()> {
        Ok(())
    }

    fn on_parse(&mut self, _: L::RuleTag, _: usize) -> CoResult<()> {
        Ok(())
    }

    fn build(self) -> CoResult<Void> {
        Ok(Void)
    }
}
