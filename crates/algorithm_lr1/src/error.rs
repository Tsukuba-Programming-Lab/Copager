use thiserror::Error;

use pgen_core::error::ParseError as SuperParseError;
use pgen_core::cfg::TokenSet;
use pgen_core::lex::Token;

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
    pub fn new_unexpected_token<'a, T>(expected: Token<'a, '_, T>) -> SuperParseError
    where
        T: TokenSet<'a>,
    {
        let err = ParseError::UnexpectedToken {
            actual: format!("{:?}", expected.kind),
        };
        SuperParseError::from(err).with(expected)
    }
}
