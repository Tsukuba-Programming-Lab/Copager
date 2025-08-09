use std::collections::VecDeque;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use copager_lang::token::{Token, TokenTag};
use copager_lang::Lang;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, Serialize, Deserialize, IR, IRBuilder)]
pub enum CSTreeOwned<L: Lang> {
    Leaf {
        tag: L::TokenTag,
        text: String,
    },
    Node {
        tag: L::RuleTag,
        children: VecDeque<CSTreeOwned<L>>,
    },
}

impl<'input, L: Lang> From<RawIR<'input, L>> for CSTreeOwned<L> {
    fn from(raw: RawIR<'input, L>) -> Self {
        match raw {
            RawIR::Atom(token) => {
                let text = token.as_str().to_owned();
                let tag = token.kind;
                CSTreeOwned::Leaf { tag, text }
            },
            RawIR::List { rule: tag, elems } => {
                let children = elems.into_iter().map(CSTreeOwned::from).collect();
                CSTreeOwned::Node { tag, children }
            }
        }
    }
}
