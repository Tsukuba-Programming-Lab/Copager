use std::marker::PhantomData;

use serde::{Serialize, Deserialize};
use serde_cbor::ser::to_vec_packed;
use serde_cbor::de::from_slice;

use copager_lex::BaseLexer;
use copager_parse::{BaseParser, ParseEvent};
use copager_ir::{IR, IRBuilder};
use copager_utils::cache::Cacheable;

use crate::generator::GeneratorDesign;

#[derive(Debug, Serialize, Deserialize)]
pub struct Processor<Gen: GeneratorDesign> {
    // Cache
    cache_lex: Option<Vec<u8>>,
    cache_parse: Option<Vec<u8>>,

    // Driver
    #[serde(skip, default="Option::default")]
    lexer: Option<Gen::Lexer>,
    #[serde(skip, default="Option::default")]
    parser: Option<Gen::Parser>,

    // Phantom
    #[serde(skip)]
    _phantom_lang: PhantomData<Gen::Lang>,
    #[serde(skip)]
    _phantom_gen: PhantomData<Gen>,
}

impl<Gen: GeneratorDesign> Processor<Gen> {
    pub fn new() -> Self {
        Processor {
            cache_lex: None,
            cache_parse: None,
            lexer: None,
            parser: None,
            _phantom_lang: PhantomData,
            _phantom_gen: PhantomData,
        }
    }

    pub fn build(self) -> anyhow::Result<Self> {
        self.build_lexer()?
            .build_parser()
    }

    pub fn build_lexer(mut self) -> anyhow::Result<Self> {
        let lexer = Gen::Lexer::init()?;
        self.lexer = Some(lexer);

        Ok(self)
    }

    pub fn build_parser(mut self) -> anyhow::Result<Self> {
        let parser = Gen::Parser::init()?;
        self.parser = Some(parser);

        Ok(self)
    }

    pub fn process<'input, I>(&self, input: &'input str) -> anyhow::Result<I>
    where
        I: IR<'input, Gen::Lang>,
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

impl<Gen> Processor<Gen>
where
    Gen: GeneratorDesign<Lexer: Cacheable<()>>,
{
    pub fn prebuild_lexer(mut self) -> anyhow::Result<Self> {
        let cache_lex = Gen::Lexer::cache(())?;
        self.cache_lex = Some(to_vec_packed(&cache_lex)?);

        Ok(self)
    }

    pub fn restore_lexer_by_cache(mut self) -> Self {
        let cache_lex = self.cache_lex.as_ref().unwrap();
        let cache_lex = from_slice(cache_lex).unwrap();
        let lexer = Gen::Lexer::restore(cache_lex);
        self.lexer = Some(lexer);

        self
    }
}

impl<Gen> Processor<Gen>
where
    Gen: GeneratorDesign< Parser: Cacheable<()>>,
{
    pub fn prebuild_parser(mut self) -> anyhow::Result<Self> {
        let cache_parse = Gen::Parser::cache(())?;
        self.cache_parse = Some(to_vec_packed(&cache_parse)?);

        Ok(self)
    }

    pub fn restore_parser_by_cache(mut self) -> Self {
        let cache_parse = self.cache_parse.as_ref().unwrap();
        let cache_parse = from_slice(cache_parse).unwrap();
        let parser = Gen::Parser::restore(cache_parse);
        self.parser = Some(parser);

        self
    }
}
