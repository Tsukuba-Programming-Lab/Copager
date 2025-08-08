use std::collections::VecDeque;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use copager_lang::token::{Token, TokenTag};
use copager_lang::Lang;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, Serialize, Deserialize, IR, IRBuilder)]
pub enum TreeOwned<L: Lang> {
    Leaf {
        tag: L::TokenTag,
        text: String,
    },
    Node {
        tag: L::RuleTag,
        children: VecDeque<TreeOwned<L>>,
    },
}

impl<'input, L: Lang> From<RawIR<'input, L>> for TreeOwned<L> {
    fn from(raw: RawIR<'input, L>) -> Self {
        match raw {
            RawIR::Atom(token) => {
                let text = token.as_str().to_string();
                let tag = token.kind;
                TreeOwned::Leaf { tag, text }
            },
            RawIR::List { rule: tag, elems } => {
                let children = elems.into_iter().map(TreeOwned::from).collect();
                TreeOwned::Node { tag, children }
            }
        }
    }
}
