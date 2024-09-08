use copager_cfg::token::Token;
use copager_lex::LexSource;
use copager_parse::ParseSource;

pub trait IR<'input, Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    type Builder: IRBuilder<'input, Sl, Sp, Output = Self>;
}

pub trait IRBuilder<'input, Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    type Output: IR<'input, Sl, Sp>;

    fn new() -> Self;
    fn on_read(&mut self, token: Token<'input, Sl::Tag>) -> anyhow::Result<()>;
    fn on_parse(&mut self, rule: Sp::Tag, len: usize) -> anyhow::Result<()>;
    fn build(self) -> anyhow::Result<Self::Output>;
}
