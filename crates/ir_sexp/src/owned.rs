use std::collections::VecDeque;
use std::fmt::{Debug, Display};

use copager_lang::token::{Token, TokenTag};
use copager_lang::Lang;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, IR, IRBuilder)]
pub enum SExpOwned<L: Lang> {
    Atom(String),
    List {
        rule: L::RuleTag,
        elems: VecDeque<SExpOwned<L>>,
    },
}

impl<L: Lang> Display for SExpOwned<L>
where
    L::TokenTag: Debug,
    L::RuleTag: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SExpOwned::List { rule, elems, .. } => {
                write!(f, "({:?}", rule)?;
                for elem in elems {
                    write!(f, " {}", elem)?;
                }
                write!(f, ")")
            }
            SExpOwned::Atom(s) => write!(f, "{:?}", s),
        }
    }
}

impl<'input, L: Lang> From<RawIR<'input, L>> for SExpOwned<L> {
    fn from(raw: RawIR<'input, L>) -> Self {
        match raw {
            RawIR::Atom(token) => {
                let s = token.as_str().to_string();
                SExpOwned::Atom(s)
            },
            RawIR::List { rule, elems } => {
                let elems = elems.into_iter().map(SExpOwned::from).collect();
                SExpOwned::List { rule, elems }
            }
        }
    }
}
