use std::collections::VecDeque;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use copager_lang::token::{Token, TokenTag};
use copager_lang::Lang;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, Serialize, Deserialize, IR, IRBuilder)]
pub enum Tree<'input, L: Lang> {
    Leaf {
        tag: L::TokenTag,
        text: &'input str,
    },
    Node {
        tag: L::RuleTag,
        children: VecDeque<Tree<'input, L>>,
    },
}

impl<'input, L: Lang> From<RawIR<'input, L>> for Tree<'input, L> {
    fn from(raw: RawIR<'input, L>) -> Self {
        match raw {
            RawIR::Atom(token) => {
                let text = token.as_str();
                let tag = token.kind;
                Tree::Leaf { tag, text }
            },
            RawIR::List { rule: tag, elems } => {
                let children = elems.into_iter().map(Tree::from).collect();
                Tree::Node { tag, children }
            }
        }
    }
}
