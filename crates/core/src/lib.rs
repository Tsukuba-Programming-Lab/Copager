pub mod error;

use std::marker::PhantomData;

use copager_lex::{LexSource, LexDriver};
use copager_parse::{ParseSource, ParseDriver};

pub struct Processor<'input, Sl, Dl, Sp, Dp>
where
    Sl: LexSource,
    Dl: LexDriver<'input, Sl::Tag>,
    Sp: ParseSource<Sl::Tag>,
    Dp: ParseDriver<'input, Sl::Tag, Sp::Tag>,
{
    _phantom_sl: PhantomData<Sl>,
    _phantom_il: PhantomData<Dl>,
    _phantom_sp: PhantomData<Sp>,
    _phantom_ip: PhantomData<Dp>,
    _phantom_input: PhantomData<&'input ()>,
}

impl<'input, 'cache, Sl, Dl, Sp, Dp> Processor<'input, Sl, Dl, Sp, Dp>
where
    Sl: LexSource,
    Dl: LexDriver<'input, Sl::Tag, From = Sl>,
    Sp: ParseSource<Sl::Tag>,
    Dp: ParseDriver<'input, Sl::Tag, Sp::Tag, From = Sp>,
{
    pub fn process(input: &'input str)
    where
        Sl: Default,
        Sp: Default,
    {
        let lexer = Dl::from(Sl::default());
        let parser = Dp::from(Sp::default());
        loop {
            for _ in parser.init(lexer.init(input)) {
                println!("-----");
            }
        }
    }
}
