pub mod error;

use std::marker::PhantomData;

use copager_lex::{LexSource, LexIterator};
use copager_parse::{ParseSource, ParseIterator};

pub struct Processor<'input, Sl, Il, Sp, Ip>
where
    Sl: LexSource,
    Il: LexIterator<'input, Sl::Tag>,
    Sp: ParseSource<Sl::Tag>,
    Ip: ParseIterator<'input, Sl::Tag, Sp::Tag, Il>,
{
    _phantom_sl: PhantomData<Sl>,
    _phantom_il: PhantomData<Il>,
    _phantom_sp: PhantomData<Sp>,
    _phantom_ip: PhantomData<Ip>,
    _phantom_input: PhantomData<&'input ()>,
}

impl<'input, 'cache, Sl, Il, Sp, Ip> Processor<'input, Sl, Il, Sp, Ip>
where
    Sl: LexSource,
    Il: LexIterator<'input, Sl::Tag, From = Sl>,
    Sp: ParseSource<Sl::Tag>,
    Ip: ParseIterator<'input, Sl::Tag, Sp::Tag, Il, From = Sp>,
{
    pub fn process(input: &'input str)
    where
        Sl: Default,
        Sp: Default,
    {
        let lexer = Il::from(Sl::default()).init(input);
        let mut parser = Ip::from(Sp::default()).init(lexer);
        loop {
            match parser.next() {
                Some(_) => {}
                None => break,
            }
        }
    }
}
