use std::collections::VecDeque;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::CFL;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, Serialize, Deserialize, IR, IRBuilder)]
pub enum Tree<'input, Lang: CFL> {
    Leaf {
        tag: Lang::TokenTag,
        text: &'input str,
    },
    Node {
        tag: Lang::RuleTag,
        children: VecDeque<Tree<'input, Lang>>,
    },
}

impl<'input, Lang: CFL> From<RawIR<'input, Lang>> for Tree<'input, Lang> {
    fn from(raw: RawIR<'input, Lang>) -> Self {
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
