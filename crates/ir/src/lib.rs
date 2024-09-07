use copager_cfg::token::{TokenTag, Token};
use copager_cfg::rule::RuleTag;

pub trait IR<'input, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    type Builder: IRBuilder<'input, T, R, Output = Self>;
}

pub trait IRBuilder<'input, T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    type Output: IR<'input, T, R>;

    fn new() -> Self;
    fn on_read(&mut self, token: Token<'input, T>) -> anyhow::Result<()>;
    fn on_parse(&mut self, rule: R) -> anyhow::Result<()>;
    fn build(self) -> anyhow::Result<Self::Output>;
}
