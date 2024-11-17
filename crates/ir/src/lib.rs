use copager_cfl::token::Token;
use copager_cfl::{CFLTokens, CFLRules};

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
