pub mod error;

use std::marker::PhantomData;

use copager_lex::{LexSource, LexDriver};
use copager_parse::{ParseSource, ParseDriver};

pub struct Processor<Sl, Dl, Sp, Dp>
where
    Sl: LexSource,
    Dl: LexDriver<Sl::Tag>,
    Sp: ParseSource<Sl::Tag>,
    Dp: ParseDriver<Sl::Tag, Sp::Tag>,
{
    _phantom_sl: PhantomData<Sl>,
    _phantom_il: PhantomData<Dl>,
    _phantom_sp: PhantomData<Sp>,
    _phantom_ip: PhantomData<Dp>,
}

impl<'cache, Sl, Dl, Sp, Dp> Processor<Sl, Dl, Sp, Dp>
where
    Sl: LexSource,
    Dl: LexDriver<Sl::Tag, From = Sl>,
    Sp: ParseSource<Sl::Tag>,
    Dp: ParseDriver<Sl::Tag, Sp::Tag, From = Sp>,
{
    pub fn process<'input>(input: &'input str)
    where
        Sl: Default,
        Sp: Default,
    {
        let lexer = Dl::from(Sl::default());
        let parser = Dp::from(Sp::default());
        loop {
            for _ in parser.run(lexer.run(input)) {
                println!("-----");
            }
        }
    }
}
