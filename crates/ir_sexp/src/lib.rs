use std::fmt::{Debug, Display};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::{CFLTokens, CFLRules};
use copager_ir::{IR, IRBuilder};

#[derive(Debug)]
pub enum SExp<'input, Ts, Rs>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    List {
        rule: Rs::Tag,
        elems: Vec<SExp<'input, Ts, Rs>>,
    },
    Atom(Token<'input, Ts::Tag>),
}

impl<Ts, Rs> Display for SExp<'_, Ts, Rs>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
    Rs::Tag: Debug,
    Ts::Tag: Debug,
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

impl<'input, Ts, Rs> IR<'input, Ts, Rs> for SExp<'input, Ts, Rs>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    type Builder = SExpBuilder<'input, Ts, Rs>;
}

#[derive(Debug)]
pub struct SExpBuilder<'input, Ts, Rs>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    stack: Vec<SExp<'input, Ts, Rs>>,
}


impl <'input, Ts, Rs> IRBuilder<'input, Ts, Rs> for SExpBuilder<'input, Ts, Rs>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    type Output = SExp<'input, Ts, Rs>;

    fn new() -> SExpBuilder<'input, Ts, Rs> {
        SExpBuilder { stack: vec![] }
    }

    fn on_read(&mut self, token: Token<'input, Ts::Tag>) -> anyhow::Result<()> {
        self.stack.push(SExp::Atom(token));
        Ok(())
    }

    fn on_parse(&mut self, rule: Rs::Tag, len: usize) -> anyhow::Result<()> {
        let elems = self.stack.split_off(self.stack.len() - len);
        let elems = elems
            .into_iter()
            .filter(|elem| match elem {
                SExp::Atom(token) => !token.kind.as_option_list().contains(&"ir_omit"),
                _ => true,
            })
            .collect();
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
