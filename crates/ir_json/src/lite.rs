use std::fmt::{Debug, Display};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::CFL;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, IR, IRBuilder)]
pub enum JsonLite<'input, Lang: CFL> {
    Token(&'input str),
    List {
        tag: Lang::RuleTag,
        elems: Vec<JsonLite<'input, Lang>>,
    }
}

impl<Lang: CFL> Display for JsonLite<'_, Lang>
where
    Lang::TokenTag: Debug,
    Lang::RuleTag: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonLite::Token(s) => {
                write!(f, r#""{}""#, s)?;
            }
            JsonLite::List { tag, elems } => {
                write!(f, r#"{{"tag": "{:?}", "elements": ["#, tag)?;
                if elems.len() > 0 {
                    write!(f, "{}", elems[0])?;
                }
                for elem in &elems[1..] {
                    write!(f, ", {}", elem)?;
                }
                write!(f, "]}}")?;
            }
        }
        write!(f, "")
    }
}

impl<'input, Lang: CFL> From<RawIR<'input, Lang>> for JsonLite<'input, Lang> {
    fn from(raw: RawIR<'input, Lang>) -> Self {
        match raw {
            RawIR::Atom(token) => JsonLite::Token(token.as_str()),
            RawIR::List { rule, elems } => JsonLite::List {
                tag: rule,
                elems: elems.into_iter().map(JsonLite::from).collect(),
            },
        }
    }
}
