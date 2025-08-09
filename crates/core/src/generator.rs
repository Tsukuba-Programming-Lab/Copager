use std::marker::PhantomData;

use copager_lang::Lang as LangTrait;
use copager_lex::BaseLexer;
use copager_parse::BaseParser;

#[derive(Debug)]
pub struct Generator<Lang, Lexer, Parser>
where
    Lang: LangTrait,
    Lexer: BaseLexer<Lang>,
    Parser: BaseParser<Lang>
{
    _phantom_lang: PhantomData<Lang>,
    _phantom_lexer: PhantomData<Lexer>,
    _phantom_parser: PhantomData<Parser>,
}

impl<Lang, Lexer, Parser> GeneratorDesign for Generator<Lang, Lexer, Parser>
where
    Lang: LangTrait,
    Lexer: BaseLexer<Lang>,
    Parser: BaseParser<Lang>
{
    type Lang = Lang;
    type Lexer = Lexer;
    type Parser = Parser;
}

pub trait GeneratorDesign {
    type Lang: LangTrait;
    type Lexer: BaseLexer<Self::Lang>;
    type Parser: BaseParser<Self::Lang>;
}
