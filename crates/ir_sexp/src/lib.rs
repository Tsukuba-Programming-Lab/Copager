use std::fmt::{Debug, Display};

use copager_cfl::token::Token;
use copager_lex::LexSource;
use copager_parse::ParseSource;
use copager_ir::{IR, IRBuilder};

#[derive(Debug)]
pub enum SExp<'input, Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    List {
        rule: Sp::Tag,
        elems: Vec<SExp<'input, Sl, Sp>>,
    },
    Atom(Token<'input, Sl::Tag>),
}

impl<Sl, Sp> Display for SExp<'_, Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
    Sp::Tag: Debug,
    Sl::Tag: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SExp::List { rule, elems } => {
                write!(f, "({:?}", rule)?;
                for elem in elems {
                    write!(f, " {}", elem)?;
                }
                write!(f, ")")
            }
            SExp::Atom(token) => write!(f, "{:?}", token.as_str()),
        }
    }
}

impl<'input, Sl, Sp> IR<'input, Sl, Sp> for SExp<'input, Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    type Builder = SExpBuilder<'input, Sl, Sp>;
}

#[derive(Debug)]
pub struct SExpBuilder<'input, Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    stack: Vec<SExp<'input, Sl, Sp>>,
}


impl <'input, Sl, Sp> IRBuilder<'input, Sl, Sp> for SExpBuilder<'input, Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    type Output = SExp<'input, Sl, Sp>;

    fn new() -> SExpBuilder<'input, Sl, Sp> {
        SExpBuilder { stack: vec![] }
    }

    fn on_read(&mut self, token: Token<'input, Sl::Tag>) -> anyhow::Result<()> {
        self.stack.push(SExp::Atom(token));
        Ok(())
    }

    fn on_parse(&mut self, rule: Sp::Tag, len: usize) -> anyhow::Result<()> {
        let elems = self.stack.split_off(self.stack.len() - len);
        self.stack.push(SExp::List { rule, elems });
        Ok(())
    }

    fn build(mut self) -> anyhow::Result<Self::Output> {
        if self.stack.len() == 1 {
            Ok(self.stack.pop().unwrap())
        } else {
            Err(anyhow::anyhow!("Invalid S-Expression"))
        }
    }
}
