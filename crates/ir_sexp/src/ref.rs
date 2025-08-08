use std::collections::VecDeque;
use std::fmt::{Debug, Display};

use copager_lang::token::{Token, TokenTag};
use copager_lang::Lang;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, IR, IRBuilder)]
pub enum SExp<'input, L: Lang> {
    Atom(&'input str),
    List {
        rule: L::RuleTag,
        elems: VecDeque<SExp<'input, L>>,
    },
}

impl<L: Lang> Display for SExp<'_, L>
where
    L::TokenTag: Debug,
    L::RuleTag: Debug,
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

impl<'input, L: Lang> From<RawIR<'input, L>> for SExp<'input, L> {
    fn from(raw: RawIR<'input, L>) -> Self {
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
