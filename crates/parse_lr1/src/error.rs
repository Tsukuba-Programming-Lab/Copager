use thiserror::Error;

use copager_core::error::ParseError as SuperParseError;
use copager_cfg::token::{TokenTag, Token};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token {actual:?} found")]
    UnexpectedToken {
        actual: String,
    },
    #[error("Unexpected EOF")]
    UnexpectedEOF,
}

impl ParseError {
    pub fn new_unexpected_token<T: TokenTag>(expected: Token<T>) -> SuperParseError {
        let err = ParseError::UnexpectedToken {
            actual: format!("{:?}", expected.kind),
        };
        SuperParseError::from(err).with(expected)
    }
}
