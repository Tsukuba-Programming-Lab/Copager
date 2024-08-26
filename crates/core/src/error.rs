use std::cmp::{max, min};
use std::error::Error as StdError;
use std::fmt::Display;

use thiserror::Error;

use copager_cfg::token::Token;
use copager_cfg::TokenKind;

#[derive(Debug, Error)]
pub struct ParseError {
    err: Box<dyn StdError + Send + Sync>,
    src: Option<String>,
    pos: Option<(usize, usize)>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
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

    pub fn with<'a, T: TokenKind<'a>>(self, token: Token<'a, '_, T>) -> ParseError {
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

    pub fn pretty_print(&self) {
        let pretty_printer = |input: &str, pos: (usize, usize)| {
            eprintln!("-----");

            let (row, col) = (pos.0 as i32 - 1, pos.1 as i32 - 1);
            let lines = input.split('\n');
            let neighbor_lines = lines
                .skip(max(0, row - 2) as usize)
                .take(min(row + 1, 3) as usize);

            neighbor_lines.enumerate().for_each(|(idx, line)| {
                let row = max(1, row - 1) + (idx as i32);
                println!("{:2}: {}", row, line);
            });

            eprintln!("    {}^ here", " ".repeat(col as usize));
            eprintln!("Error at line {}, column {}.", row + 1, col + 1);
            eprintln!("-----\n");
        };

        match (&self.src, self.pos) {
            (Some(src), Some(pos)) => {
                pretty_printer(&src, pos);
            }
            _ => {},
        }
    }
}
