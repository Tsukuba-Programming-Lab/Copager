use std::fmt::{Debug, Display};

use copager_cfl::token::{Token, TokenTag};
use copager_cfl::CFL;
use copager_ir::{IR, IRBuilder, RawIR};

#[derive(Debug, IR, IRBuilder)]
pub enum Xml<'input, Lang: CFL> {
    Token {
        tag: Lang::TokenTag,
        string: &'input str,
    },
    List {
        tag: Lang::RuleTag,
        elems: Vec<Xml<'input, Lang>>,
    }
}

impl<Lang: CFL> Display for Xml<'_, Lang>
where
    Lang::TokenTag: Debug,
    Lang::RuleTag: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Xml::Token { tag, string } => {
                write!(f, r#"<token><tag>{:?}</tag><string>{:?}</string></token>"#, tag, string)?;
            }
            Xml::List { tag, elems } => {
                write!(f, r#"<list><tag>{:?}</tag><elements>"#, tag)?;
                if elems.len() > 0 {
                    write!(f, "{}", elems[0])?;
                }
                for elem in &elems[1..] {
                    write!(f, "{}", elem)?;
                }
                write!(f, "</elements></list>")?;
            }
        }
        write!(f, "")
    }
}

impl<'input, Lang: CFL> From<RawIR<'input, Lang>> for Xml<'input, Lang> {
    fn from(raw: RawIR<'input, Lang>) -> Self {
        match raw {
            RawIR::Atom(token) => Xml::Token {
                tag: token.kind,
                string: token.as_str(),
            },
            RawIR::List { rule, elems } => Xml::List {
                tag: rule,
                elems: elems.into_iter().map(Xml::from).collect(),
            },
        }
    }
}
