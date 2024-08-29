use copager_cfg::token::TokenTag;
use copager_cfg::rule::RuleTag;

pub trait IR<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    type Builder: IRBuilder<T, R>;
}

pub trait IRBuilder<T, R>
where
    T: TokenTag,
    R: RuleTag<T>,
{
    type Output: IR<T, R>;

    fn new() -> Self;
    fn build(self) -> anyhow::Result<Self::Output>;
}
