use std::cmp::{max, min};
use std::error::Error as StdError;
use std::fmt::Display;

use thiserror::Error;

use copager_lang::token::{TokenTag, Token};

#[derive(Debug, Error)]
pub struct DiagnosticError {
    pub err: Box<dyn StdError + Send + Sync>,
    pub diagnostics: Option<Diagnostics>,
}

impl Display for DiagnosticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.err)?;
        if let Some(d) = &self.diagnostics {
            writeln!(f, "{}", d)?;
        }
        Ok(())
    }
}

impl DiagnosticError {
    pub fn from<E>(err: E) -> DiagnosticError
    where
        E: StdError + Send + Sync + 'static,
    {
        DiagnosticError {
            err: Box::new(err),
            diagnostics: None,
        }
    }

    pub fn with<'input, T: TokenTag>(self, token: Token<'input, T>) -> DiagnosticError {
        DiagnosticError {
            err: self.err,
            diagnostics: Some(Diagnostics::from(token)),
        }
    }
}

#[derive(Debug, Error)]
pub struct Diagnostics {
    pub src: String,
    pub range: (usize, usize),
}

impl Display for Diagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range_to_pos = |range: (usize, usize)| -> (usize, usize) {
            let mut sum = 0;
            let (mut rows, mut cols) = (1, 1);
            for c in self.src.chars() {
                if range.0 <= sum {
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
            (rows, cols)
        };

        let mut pretty_print = |src: &str, pos: (usize, usize)| {
            writeln!(f, "-----")?;

            let (row, col) = (pos.0 as i32 - 1, pos.1 as i32 - 1);
            let lines = src.split('\n');
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
        };

        pretty_print(&self.src, range_to_pos(self.range))
    }
}

impl<T: TokenTag> From<Token<'_, T>> for Diagnostics {
    fn from(token: Token<'_, T>) -> Self {
        Diagnostics {
            src: token.src.to_string(),
            range: (token.body.0, token.body.1),
        }
    }
}
