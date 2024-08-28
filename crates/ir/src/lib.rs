use copager_cfg::token::TokenTag;
use copager_cfg::RuleKind;

pub trait IR<T, R>
where
    T: TokenTag,
    R: RuleKind<T>,
{
    type Builder: IRBuilder<T, R>;
}

pub trait IRBuilder<T, R>
where
    T: TokenTag,
    R: RuleKind<T>,
{
    type Output: IR<T, R>;

    fn new() -> Self;
    fn build(self) -> anyhow::Result<Self::Output>;
}
