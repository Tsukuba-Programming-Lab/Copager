use std::marker::PhantomData;

use copager_cfl::CFL;
use copager_lex::BaseLexer;
use copager_parse::BaseParser;

#[derive(Debug)]
pub struct Generator<Lang, Lexer, Parser>
where
    Lang: CFL,
    Lexer: BaseLexer<Lang>,
    Parser: BaseParser<Lang>
{
    _phantom_lang: PhantomData<Lang>,
    _phantom_lexer: PhantomData<Lexer>,
    _phantom_parser: PhantomData<Parser>,
}

impl<Lang, Lexer, Parser> GeneratorDesign for Generator<Lang, Lexer, Parser>
where
    Lang: CFL,
    Lexer: BaseLexer<Lang>,
    Parser: BaseParser<Lang>
{
    type Lang = Lang;
    type Lexer = Lexer;
    type Parser = Parser;
}

pub trait GeneratorDesign {
    type Lang: CFL;
    type Lexer: BaseLexer<Self::Lang>;
    type Parser: BaseParser<Self::Lang>;
}
