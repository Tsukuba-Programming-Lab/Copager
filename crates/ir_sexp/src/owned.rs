use std::fmt::{Debug, Display};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::CFL;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, IR, IRBuilder)]
pub enum SExpOwned<Lang: CFL> {
    Atom(String),
    List {
        rule: Lang::RuleTag,
        elems: Vec<SExpOwned<Lang>>,
    },
}

impl<Lang: CFL> Display for SExpOwned<Lang>
where
    Lang::TokenTag: Debug,
    Lang::RuleTag: Debug,
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

impl<'input, Lang: CFL> From<RawIR<'input, Lang>> for SExpOwned<Lang> {
    fn from(raw: RawIR<'input, Lang>) -> Self {
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
