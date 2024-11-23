use std::fmt::{Debug, Display};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::CFL;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, IR, IRBuilder)]
pub enum Json<'input, Lang: CFL> {
    Token {
        tag: Lang::TokenTag,
        string: &'input str,
    },
    List {
        tag: Lang::RuleTag,
        elems: Vec<Json<'input, Lang>>,
    }
}

impl<Lang: CFL> Display for Json<'_, Lang>
where
    Lang::TokenTag: Debug,
    Lang::RuleTag: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        match self {
            Json::Token { tag, string } => {
                write!(f, r#""kind": "token", "tag": "{:?}", "text": "{}""#, tag, string)?;
            }
            Json::List { tag, elems } => {
                write!(f, r#""kind": "list", "tag": "{:?}", "elements": ["#, tag)?;
                if elems.len() > 0 {
                    write!(f, "{}", elems[0])?;
                }
                for elem in &elems[1..] {
                    write!(f, ", {}", elem)?;
                }
                write!(f, "]")?;
            }
        }
        write!(f, "}}")
    }
}

impl<'input, Lang: CFL> From<RawIR<'input, Lang>> for Json<'input, Lang> {
    fn from(raw: RawIR<'input, Lang>) -> Self {
        match raw {
            RawIR::Atom(token) => Json::Token {
                tag: token.kind,
                string: token.as_str(),
            },
            RawIR::List { rule, elems } => Json::List {
                tag: rule,
                elems: elems.into_iter().map(Json::from).collect(),
            },
        }
    }
}
