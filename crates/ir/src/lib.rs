use copager_cfl::token::Token;
use copager_cfl::{CFLTokens, CFLRules};
#[cfg(feature = "derive")]
pub use copager_ir_derive::{IR, IRBuilder};

pub trait IR<'input, Ts, Rs>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    type Builder: IRBuilder<'input, Ts, Rs, Output = Self>;
}

pub trait IRBuilder<'input, Ts, Rs>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    type Output: IR<'input, Ts, Rs>;

    fn new() -> Self;
    fn on_read(&mut self, token: Token<'input, Ts::Tag>) -> anyhow::Result<()>;
    fn on_parse(&mut self, rule: Rs::Tag, len: usize) -> anyhow::Result<()>;
    fn build(self) -> anyhow::Result<Self::Output>;
}

#[cfg(feature = "derive")]
#[derive(Debug)]
pub enum RawIR<'input, Ts, Rs>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    Atom(Token<'input, Ts::Tag>),
    List {
        rule: Rs::Tag,
        elems: Vec<RawIR<'input, Ts, Rs>>
    },
}
