use std::fmt::{Debug, Display};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::CFL;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, IR, IRBuilder)]
pub enum SExp<'input, Lang: CFL> {
    Atom(Token<'input, Lang::TokenTag>),
    List {
        rule: Lang::RuleTag,
        elems: Vec<SExp<'input, Lang>>,
    },
}

impl<Lang: CFL> Display for SExp<'_, Lang>
where
    Lang::TokenTag: Debug,
    Lang::RuleTag: Debug,
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

impl<'input, Lang: CFL> From<RawIR<'input, Lang>> for SExp<'input, Lang> {
    fn from(raw: RawIR<'input, Lang>) -> Self {
        match raw {
            RawIR::Atom(token) => SExp::Atom(token),
            RawIR::List { rule, elems } => SExp::List {
                rule,
                elems: elems.into_iter().map(SExp::from).collect(),
            },
        }
    }
}
