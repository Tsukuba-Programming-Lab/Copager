#![feature(gen_blocks)]

mod error;
mod builder;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use copager_cfg::token::Token;
use copager_lex::LexSource;
use copager_parse::{ParseSource, ParseDriver, ParseState};
use copager_utils::cache::Cacheable;

use builder::{LR1Configure, LRAction};
use error::ParseError;

#[derive(Debug)]
pub struct LR1<Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    tables: LR1Configure<Sl, Sp>,
}

impl<Sl, Sp> Cacheable<(Sl, Sp)> for LR1<Sl, Sp>
where
    Sl: LexSource,
    Sl::Tag: Serialize + for<'de> Deserialize<'de>,
    Sp: ParseSource<Sl::Tag>,
    Sp::Tag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LR1Configure<Sl, Sp>;

    fn new((source_l, source_p): (Sl, Sp)) -> anyhow::Result<Self::Cache> {
        Ok(LR1Configure::new(&source_l, &source_p)?)
    }

    fn restore(tables: Self::Cache) -> Self {
        LR1 { tables }
    }
}

impl<Sl, Sp> ParseDriver<Sl, Sp> for LR1<Sl, Sp>
where
    Sl: LexSource,
    Sp: ParseSource<Sl::Tag>,
{
    fn try_from((source_l, source_p): (Sl, Sp)) -> anyhow::Result<Self> {
        let tables = LR1Configure::new(&source_l, &source_p)?;
        Ok(LR1 { tables })
    }

    gen fn run<'input, Il>(&self, mut lexer: Il) -> ParseState<'input, Sl::Tag, Sp::Tag>
    where
        Il: Iterator<Item = Token<'input, Sl::Tag>>,
    {
        let mut stack = vec![0];
        loop {
            let token = lexer.next();
            loop {
                let top = stack[stack.len() - 1];
                let action = match token {
                    Some(token) => {
                        let local_action_table: &HashMap<_, _> = &self.tables.action_table[top];
                        (local_action_table.get(&token.kind).unwrap(), Some(token))
                    },
                    None => (&self.tables.eof_action_table[top], None),
                };
                match action {
                    (LRAction::Shift(new_state), Some(token)) => {
                        stack.push(*new_state);
                        yield ParseState::Consume(token);
                        break;
                    }
                    (LRAction::Reduce(tag, goto, elems_cnt), _) => {
                        stack.truncate(stack.len() - elems_cnt);
                        stack.push(self.tables.goto_table[stack[stack.len() - 1]][*goto]);
                        yield ParseState::Reduce(*tag);
                    }
                    (LRAction::Accept, _) => {
                        return;
                    }
                    (LRAction::None, Some(token)) => {
                        yield ParseState::Err(ParseError::new_unexpected_token(token).into());
                        return;
                    }
                    (LRAction::None, None) => {
                        yield ParseState::Err(ParseError::UnexpectedEOF.into());
                        return;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

// #[cfg(test)]
// mod test {
//     use copager_core::cfg::{TokenSet, Syntax, Rule, RuleElem};
//     use copager_core::Parser;

//     use super::LR1;

//     #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, TokenSet)]
//     enum TestTokenSet {
//         #[token(regex = r"\+")]
//         Plus,
//         #[token(regex = r"-")]
//         Minus,
//         #[token(regex = r"\*")]
//         Mul,
//         #[token(regex = r"/")]
//         Div,
//         #[token(regex = r"\(")]
//         BracketL,
//         #[token(regex = r"\)")]
//         BracketR,
//         #[token(regex = r"[1-9][0-9]*")]
//         Num,
//         #[token(regex = r"[ \t\n]+", ignored)]
//         _Whitespace,
//     }

//     #[derive(Debug, Clone, Copy, Syntax)]
//     enum TestSyntax {
//         #[rule("<expr> ::= <expr> Plus <term>")]
//         #[rule("<expr> ::= <expr> Minus <term>")]
//         #[rule("<expr> ::= <term>")]
//         Expr,
//         #[rule("<term> ::= <term> Mul <num>")]
//         #[rule("<term> ::= <term> Div <num>")]
//         #[rule("<term> ::= <num>")]
//         Term,
//         #[rule("<num> ::= BracketL <expr> BracketR")]
//         #[rule("<num> ::= Num")]
//         Num,
//     }

//     #[test]
//     fn input_ok() {
//         let inputs = vec![
//             "10",
//             "10 + 20",
//             "10 - 20",
//             "10 * 20",
//             "10 / 20",
//             "10 + 20 * 30 - 40",
//             "(10)",
//             "((((10))))",
//             "10 * (20 - 30)",
//             "((10 + 20) * (30 / 40)) - 50",
//         ];

//         let parser = Parser::<LR1<TestTokenSet, TestSyntax>>::new().unwrap();
//         for input in inputs {
//             assert!(parser.parse(input).is_ok(), "{}", input);
//         }
//     }

//     #[test]
//     fn input_err() {
//         let inputs = vec![
//             "()",
//             "(10 -",
//             "10 +",
//             "*",
//             "10 20 + 30",
//             "10 + 20 * 30 / 40 (",
//             "(((10))",
//         ];

//         let parser = Parser::<LR1<TestTokenSet, TestSyntax>>::new().unwrap();
//         for input in inputs {
//             assert!(parser.parse(input).is_err(), "{}", input);
//         }
//     }
// }
