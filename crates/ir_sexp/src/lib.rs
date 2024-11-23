use std::fmt::{Debug, Display};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::{CFLTokens, CFLRules};
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, IR, IRBuilder)]
pub enum SExp<'input, Ts, Rs>
where
    Ts: CFLTokens + 'input,
    Rs: CFLRules<Ts::Tag>,
{
    Atom(Token<'input, Ts::Tag>),
    List {
        rule: Rs::Tag,
        elems: Vec<SExp<'input, Ts, Rs>>,
    },
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

impl<'input, Ts, Rs> From<RawIR<'input, Ts, Rs>> for SExp<'input, Ts, Rs>
where
    Ts: CFLTokens,
    Rs: CFLRules<Ts::Tag>,
{
    fn from(raw: RawIR<'input, Ts, Rs>) -> Self {
        match raw {
            RawIR::Atom(token) => SExp::Atom(token),
            RawIR::List { rule, elems } => SExp::List {
                rule,
                elems: elems.into_iter().map(SExp::from).collect(),
            },
        }
    }
}
