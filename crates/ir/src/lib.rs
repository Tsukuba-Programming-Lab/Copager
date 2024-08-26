use copager_cfg::{RuleKind, TokenKind};

pub trait IR<'a, T, R>
where
    T: TokenKind<'a>,
    R: RuleKind<'a, TokenKind = T>,
{
    type Builder: IRBuilder<'a>;
}

pub trait IRBuilder<'a> {
    type TokenKind: TokenKind<'a>;
    type RuleKind: RuleKind<'a, TokenKind = Self::TokenKind>;
    type Output: IR<'a, Self::TokenKind, Self::RuleKind>;

    fn new() -> Self;
    fn build(self) -> anyhow::Result<Self::Output>;
}
