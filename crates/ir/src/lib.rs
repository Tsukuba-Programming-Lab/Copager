use copager_cfl::token::Token;
use copager_cfl::CFL;
#[cfg(feature = "derive")]
pub use copager_ir_derive::{IR, IRBuilder};

pub trait IR<'input, Lang: CFL> {
    type Builder: IRBuilder<'input, Lang, Output = Self>;
}

pub trait IRBuilder<'input, Lang: CFL> {
    type Output: IR<'input, Lang>;

    fn new() -> Self;
    fn on_read(&mut self, token: Token<'input, Lang::TokenTag>) -> anyhow::Result<()>;
    fn on_parse(&mut self, rule: Lang::RuleTag, len: usize) -> anyhow::Result<()>;
    fn build(self) -> anyhow::Result<Self::Output>;
}

#[cfg(feature = "derive")]
#[derive(Debug)]
pub enum RawIR<'input, Lang: CFL> {
    Atom(Token<'input, Lang::TokenTag>),
    List {
        rule: Lang::RuleTag,
        elems: Vec<RawIR<'input, Lang>>
    },
}
