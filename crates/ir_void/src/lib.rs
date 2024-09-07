use std::fmt::Debug;

use copager_cfg::token::Token;
use copager_lex::LexSource;
use copager_parse::ParseSource;
use copager_ir::{IR, IRBuilder};

#[derive(Debug)]
pub struct Void;

impl<'input, Sl, Sp> IR<'input, Sl, Sp> for Void
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    type Builder = Self;
}

impl <'input, Sl, Sp> IRBuilder<'input, Sl, Sp> for Void
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    type Output = Self;

    fn new() -> Void {
        Void
    }

    fn on_read(&mut self, _: Token<'input, Sl::Tag>) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_parse(&mut self, _: Sp::Tag) -> anyhow::Result<()> {
        Ok(())
    }

    fn build(self) -> anyhow::Result<Void> {
        Ok(Void)
    }
}
