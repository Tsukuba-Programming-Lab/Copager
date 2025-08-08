use std::collections::VecDeque;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::CFL;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, Serialize, Deserialize, IR, IRBuilder)]
pub enum TreeOwned<Lang: CFL> {
    Leaf {
        tag: Lang::TokenTag,
        text: String,
    },
    Node {
        tag: Lang::RuleTag,
        children: VecDeque<TreeOwned<Lang>>,
    },
}

impl<'input, Lang: CFL> From<RawIR<'input, Lang>> for TreeOwned<Lang> {
    fn from(raw: RawIR<'input, Lang>) -> Self {
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
