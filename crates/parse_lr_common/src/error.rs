use thiserror::Error;

use copager_core::error::ParseError;
use copager_cfg::token::{TokenTag, Token};
use copager_cfg::rule::RuleTag;

use crate::table::LRAction;

#[derive(Debug, Error)]
pub enum LRError {
    #[error("Conflict occured at [{action}]")]
    Conflilct {
        action: String,
    },
    #[error("Unexpected token {actual:?} found")]
    UnexpectedToken {
        actual: String,
    },
    #[error("Unexpected EOF")]
    UnexpectedEOF,
}

impl LRError {
    pub fn new_conflict<T, R>(action: &LRAction<T, R>) -> ParseError
    where
        T: TokenTag,
        R: RuleTag<T>,
    {
        let action = match action {
            LRAction::Shift(state) => format!("Shift({})", state),
            LRAction::Reduce(rule) => format!("Reduce({})", rule),
            LRAction::Accept => format!("Accept"),
            _ => unimplemented!(),
        };
        ParseError::from(LRError::Conflilct{ action })
    }

    pub fn new_unexpected_token<T: TokenTag>(expected: Token<T>) -> ParseError {
        let err = LRError::UnexpectedToken {
            actual: format!("{:?}", expected.kind),
        };
        ParseError::from(err).with(expected)
    }

    pub fn new_unexpected_eof() -> ParseError {
        ParseError::from(LRError::UnexpectedEOF)
    }
}
