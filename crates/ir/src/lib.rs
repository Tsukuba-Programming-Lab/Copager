use copager_cfl::token::Token;
use copager_cfl::{CFLTokens, CFLRules};

pub trait IR<'input, Sl, Sp>
where
    Sl: CFLTokens,
    Sp: CFLRules<Sl::Tag>,
{
    type Builder: IRBuilder<'input, Sl, Sp, Output = Self>;
}

pub trait IRBuilder<'input, Sl, Sp>
where
    Sl: CFLTokens,
    Sp: CFLRules<Sl::Tag>,
{
    type Output: IR<'input, Sl, Sp>;

    fn new() -> Self;
    fn on_read(&mut self, token: Token<'input, Sl::Tag>) -> anyhow::Result<()>;
    fn on_parse(&mut self, rule: Sp::Tag, len: usize) -> anyhow::Result<()>;
    fn build(self) -> anyhow::Result<Self::Output>;
}
