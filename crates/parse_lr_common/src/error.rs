use thiserror::Error;

use copager_cfl::token::{TokenTag, Token};
use copager_cfl::rule::RuleTag;
use copager_utils::error::PrettyError;

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
    pub fn new_conflict<T, R>(action: &LRAction<T, R>) -> PrettyError
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
        PrettyError::from(LRError::Conflilct{ action })
    }

    pub fn new_unexpected_token<T: TokenTag>(expected: Token<T>) -> PrettyError {
        let err = LRError::UnexpectedToken {
            actual: format!("{:?}", expected.kind),
        };
        PrettyError::from(err).with(expected)
    }

    pub fn new_unexpected_eof() -> PrettyError {
        PrettyError::from(LRError::UnexpectedEOF)
    }
}
