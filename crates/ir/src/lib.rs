use copager_lang::token::Token;
use copager_lang::Lang;
#[cfg(feature = "derive")]
pub use copager_ir_derive::{IR, IRBuilder};

pub trait IR<'input, L: Lang> {
    type Builder: IRBuilder<'input, L, Output = Self>;
}

pub trait IRBuilder<'input, L: Lang> {
    type Output: IR<'input, L>;

    fn new() -> Self;
    fn on_read(&mut self, token: Token<'input, L::TokenTag>) -> anyhow::Result<()>;
    fn on_parse(&mut self, rule: L::RuleTag, len: usize) -> anyhow::Result<()>;
    fn build(self) -> anyhow::Result<Self::Output>;
}

#[cfg(feature = "derive")]
#[derive(Debug)]
pub enum RawIR<'input, L: Lang> {
    Atom(Token<'input, L::TokenTag>),
    List {
        rule: L::RuleTag,
        elems: Vec<RawIR<'input, L>>
    },
}
