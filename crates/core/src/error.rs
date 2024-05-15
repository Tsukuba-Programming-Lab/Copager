use std::error::Error as StdError;
use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub struct ParseError {
    err: Box<dyn StdError + Send + Sync>,
    pos: Option<(u32, u32)>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {:?}", self.err, self.pos)
    }
}

impl ParseError {
    pub fn from<E>(err: E) -> ParseError
    where
        E: StdError + Send + Sync + 'static,
    {
        ParseError {
            err: Box::new(err),
            pos: None,
        }
    }

    pub fn with(self, pos: Option<(u32, u32)>) -> ParseError {
        ParseError {
            err: self.err,
            pos,
        }
    }
}
