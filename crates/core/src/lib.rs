pub mod error;

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};
use serde_cbor::ser::to_vec_packed;
use serde_cbor::de::from_slice;

use copager_lex::{LexSource, BaseLexer};
use copager_parse::{ParseSource, BaseParser, ParseEvent};
use copager_ir::{IR, IRBuilder};
use copager_utils::cache::Cacheable;

pub trait LanguageDesign {
    type Lex: LexSource;
    type Parse: ParseSource<<Self::Lex as LexSource>::Tag>;
}

pub struct Language<Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    _phantom_sl: PhantomData<Sl>,
    _phantom_sp: PhantomData<Sp>,
}

impl<Sl, Sp> LanguageDesign for Language<Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    type Lex = Sl;
    type Parse = Sp;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Processor<G, Dl, Dp>
where
    G: LanguageDesign,
    Dl: BaseLexer<G::Lex>,
    Dp: BaseParser<G::Lex, G::Parse>,
{
    // Cache
    cache_lex: Option<Vec<u8>>,
    cache_parse: Option<Vec<u8>>,

    // Driver
    #[serde(skip, default="Option::default")]
    lexer: Option<Dl>,
    #[serde(skip, default="Option::default")]
    parser: Option<Dp>,

    // Phantom
    #[serde(skip)]
    _phantom_g: PhantomData<G>,
    #[serde(skip)]
    _phantom_dl: PhantomData<Dl>,
    #[serde(skip)]
    _phantom_dp: PhantomData<Dp>,
}

impl<G, Dl, Dp> Processor<G, Dl, Dp>
where
    G: LanguageDesign,
    Dl: BaseLexer<G::Lex>,
    Dp: BaseParser<G::Lex, G::Parse>,
{
    pub fn new() -> Self {
        Processor {
            cache_lex: None,
            cache_parse: None,
            lexer: None,
            parser: None,
            _phantom_g: PhantomData,
            _phantom_dl: PhantomData,
            _phantom_dp: PhantomData,
        }
    }

    pub fn build_lexer(self) -> anyhow::Result<Self>
    where
        G::Lex: Default,
    {
        self.build_lexer_by(G::Lex::default())
    }

    pub fn build_lexer_by(mut self, source: G::Lex) -> anyhow::Result<Self> {
        let lexer = Dl::try_from(source)?;
        self.lexer = Some(lexer);

        Ok(self)
    }

    pub fn build_parser(self) -> anyhow::Result<Self>
    where
        G::Lex: Default,
        G::Parse: Default,
    {
        self.build_parser_by((G::Lex::default(), G::Parse::default()))
    }

    pub fn build_parser_by(mut self, source: (G::Lex, G::Parse)) -> anyhow::Result<Self> {
        let parser = Dp::try_from(source)?;
        self.parser = Some(parser);

        Ok(self)
    }

    pub fn process<'input, I>(&self, input: &'input str) -> anyhow::Result<I>
    where
        I: IR<'input, G::Lex, G::Parse>,
    {
        let lexer = self.lexer.as_ref().unwrap();
        let parser = self.parser.as_ref().unwrap();

        let mut ir_builder = I::Builder::new();
        for result in parser.run(lexer.run(input)) {
            match result {
                ParseEvent::Read(token) => ir_builder.on_read(token)?,
                ParseEvent::Parse{ rule,len } => ir_builder.on_parse(rule, len)?,
                ParseEvent::Err(err) => return Err(err),
            }
        }

        ir_builder.build()
    }
}

impl<G, Dl, Dp> Processor<G, Dl, Dp>
where
    G: LanguageDesign,
    Dl: BaseLexer<G::Lex> + Cacheable<G::Lex>,
    Dp: BaseParser<G::Lex, G::Parse>,
{
    pub fn prebuild_lexer(self) -> anyhow::Result<Self>
    where
        G::Lex: Default,
    {
        self.prebuild_lexer_by(G::Lex::default())
    }

    pub fn prebuild_lexer_by(mut self, source: G::Lex) -> anyhow::Result<Self> {
        let cache_lex = Dl::new(source)?;
        self.cache_lex = Some(to_vec_packed(&cache_lex)?);

        Ok(self)
    }

    pub fn build_lexer_by_cache(mut self) -> Self {
        let cache_lex = self.cache_lex.as_ref().unwrap();
        let cache_lex = from_slice(cache_lex);
        let lexer = Dl::restore(cache_lex.unwrap());
        self.lexer = Some(lexer);

        self
    }
}

impl<G, Dl, Dp> Processor<G, Dl, Dp>
where
    G: LanguageDesign,
    Dl: BaseLexer<G::Lex>,
    Dp: BaseParser<G::Lex, G::Parse> + Cacheable<(G::Lex, G::Parse)>,
{
    pub fn prebuild_parser(self) -> anyhow::Result<Self>
    where
        G::Lex: Default,
        G::Parse: Default,
    {
        self.prebuild_parser_by((G::Lex::default(), G::Parse::default()))
    }

    pub fn prebuild_parser_by(mut self, source: (G::Lex, G::Parse)) -> anyhow::Result<Self> {
        let cache_parse = Dp::new(source)?;
        self.cache_parse = Some(to_vec_packed(&cache_parse)?);

        Ok(self)
    }

    pub fn build_parser_by_cache(mut self) -> Self {
        let cache_parse = self.cache_parse.as_ref().unwrap();
        let cache_parse = from_slice(cache_parse);
        let parser = Dp::restore(cache_parse.unwrap());
        self.parser = Some(parser);

        self
    }
}
