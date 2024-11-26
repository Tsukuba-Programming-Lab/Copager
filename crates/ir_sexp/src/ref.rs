use std::collections::VecDeque;
use std::fmt::{Debug, Display};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::CFL;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, IR, IRBuilder)]
pub enum SExp<'input, Lang: CFL> {
    Atom(&'input str),
    List {
        rule: Lang::RuleTag,
        elems: VecDeque<SExp<'input, Lang>>,
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
            SExp::Atom(s) => write!(f, "{:?}", s),
        }
    }
}

impl<'input, Lang: CFL> From<RawIR<'input, Lang>> for SExp<'input, Lang> {
    fn from(raw: RawIR<'input, Lang>) -> Self {
        match raw {
            RawIR::Atom(token) => {
                let s = token.as_str();
                SExp::Atom(s)
            }
            RawIR::List { rule, elems } => {
                let elems = elems.into_iter().map(SExp::from).collect();
                SExp::List { rule, elems }
            }
        }
    }
}
