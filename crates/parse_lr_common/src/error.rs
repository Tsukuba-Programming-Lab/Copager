use std::fmt::Debug;

use thiserror::Error;

use copager_lang::token::{TokenTag, Token};
use copager_lang::rule::RuleTag;
use copager_utils::error::DiagnosticError;

use crate::table::LRAction;

#[derive(Debug, Error)]
pub enum LRError {
    #[error("Conflict occured at {action_a} and {action_b}")]
    Conflilct {
        action_a: String,
        action_b: String,
    },
    #[error("Unexpected token {actual:?} found")]
    UnexpectedToken {
        actual: String,
    },
    #[error("Unexpected EOF")]
    UnexpectedEOF,
}

impl LRError {
    pub fn new_conflict<T, R>(action_a: &LRAction<T, R>, action_b: &LRAction<T, R>) -> DiagnosticError
    where
        T: TokenTag,
        R: RuleTag<T>,
    {
        let action_a = format!("{}", action_a);
        let action_b = format!("{}", action_b);
        DiagnosticError::from(LRError::Conflilct{ action_a, action_b })
    }

    pub fn new_unexpected_token<T>(expected: Token<T>) -> DiagnosticError
    where
        T: TokenTag,
    {
        let err = LRError::UnexpectedToken {
            actual: format!("{:?}", "TODO") // TODO: expected.kind),
        };
        DiagnosticError::from(err).with(expected)
    }

    pub fn new_unexpected_eof() -> DiagnosticError {
        DiagnosticError::from(LRError::UnexpectedEOF)
    }
}
