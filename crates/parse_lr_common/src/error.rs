use thiserror::Error;

use copager_core::error::ParseError;
use copager_cfg::token::{TokenTag, Token};

#[derive(Debug, Error)]
pub enum LRError {
    #[error("Unexpected token {actual:?} found")]
    UnexpectedToken {
        actual: String,
    },
    #[error("Unexpected EOF")]
    UnexpectedEOF,
}

impl LRError {
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
