use std::cmp::{max, min};
use std::error::Error as StdError;
use std::fmt::Display;

use thiserror::Error;

use copager_cfg::token::{TokenTag, Token};

#[derive(Debug, Error)]
pub struct ParseError {
    err: Box<dyn StdError + Send + Sync>,
    src: Option<String>,
    pos: Option<(usize, usize)>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn pretty_print(
            f: &mut std::fmt::Formatter<'_>,
            input: &str,
            pos: (usize, usize)
        ) -> std::fmt::Result {
            writeln!(f, "-----")?;

            let (row, col) = (pos.0 as i32 - 1, pos.1 as i32 - 1);
            let lines = input.split('\n');
            let neighbor_lines = lines
                .skip(max(0, row - 2) as usize)
                .take(min(row + 1, 3) as usize);

            for (idx, line) in neighbor_lines.enumerate() {
                let row = max(1, row - 1) + (idx as i32);
                writeln!(f, "{:2}: {}", row, line)?;
            }

            writeln!(f, "    {}^ here", " ".repeat(col as usize))?;
            writeln!(f, "Found at line {}, column {}.", row + 1, col + 1)?;
            writeln!(f, "-----")
        }

        writeln!(f, "{}", self.err)?;
        match (&self.src, self.pos) {
            (Some(src), Some(pos)) => pretty_print(f, &src, pos)?,
            _ => {},
        }

        Ok(())
    }
}

impl ParseError {
    pub fn from<E>(err: E) -> ParseError
    where
        E: StdError + Send + Sync + 'static,
    {
        ParseError {
            err: Box::new(err),
            src: None,
            pos: None,
        }
    }

    pub fn with<'input, T: TokenTag>(self, token: Token<'input, T>) -> ParseError {
        let mut sum = 0;
        let (mut rows, mut cols) = (1, 1);
        for c in token.src.chars() {
            if token.range.0 <= sum {
                break;
            }
            sum += c.len_utf8();

            match c {
                '\n' => {
                    rows += 1;
                    cols = 1;
                }
                _ => {
                    cols += 1;
                }
            }
        }

        ParseError {
            err: self.err,
            src: Some(token.src.to_string()),
            pos: Some((rows, cols)),
        }
    }
}
