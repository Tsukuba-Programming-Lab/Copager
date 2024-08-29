mod error;
mod builder;

use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use copager_lex::{LexSource, LexIterator};
use copager_parse::{ParseSource, ParseIterator};
use copager_utils::cache::Cacheable;

use builder::{LR1Configure, LRAction};
use error::ParseError;

#[derive(Debug)]
pub struct LR1<'cache, 'input, Sl, Il, Sp>
where
    Sl: LexSource,
    Il: LexIterator<'input, Sl::Tag>,
    Sp: ParseSource<Sl::Tag>,
{
    // LR-Table
    tables: &'cache LR1Configure<Sl, Sp>,

    // States
    lexer: Option<Il>,
    stack: Option<Vec<usize>>,

    // Phantom Data
    _phantom: PhantomData<&'input ()>,
}

impl<'cache, 'input, Sl, Il, Sp> Cacheable<'cache, (Sl, Sp)> for LR1<'cache, 'input, Sl, Il, Sp>
where
    Sl: LexSource,
    Sl::Tag: Serialize + for<'de> Deserialize<'de>,
    Il: LexIterator<'input, Sl::Tag>,
    Sp: ParseSource<Sl::Tag>,
    Sp::Tag: Serialize + for<'de> Deserialize<'de>,
{
    type Cache = LR1Configure<Sl, Sp>;

    fn new((source_l, source_p): (Sl, Sp)) -> anyhow::Result<Self::Cache> {
        Ok(LR1Configure::new(&source_l, &source_p)?)
    }

    fn restore(tables: &'cache Self::Cache) -> Self {
        Self::from(tables)
    }
}

impl<'cache, 'input, Sl, Il, Sp> From<&'cache LR1Configure<Sl, Sp>> for LR1<'cache, 'input, Sl, Il, Sp>
where
    Sl: LexSource,
    Il: LexIterator<'input, Sl::Tag>,
    Sp: ParseSource<Sl::Tag>,
{
    fn from(tables: &'cache LR1Configure<Sl, Sp>) -> Self {
        LR1 {
            tables,
            lexer: None,
            stack: None,
            _phantom: PhantomData,
        }
    }
}

impl<'cache, 'input, Sl, Il, Sp> ParseIterator<'input, Sl::Tag, Sp::Tag, Il> for LR1<'cache, 'input, Sl, Il, Sp>
where
    Sl: LexSource,
    Il: LexIterator<'input, Sl::Tag>,
    Sp: ParseSource<Sl::Tag>,
{
    type From = &'cache LR1Configure<Sl, Sp>;

    fn init(&self, lexer: Il) -> Self {
        LR1 {
            tables: &self.tables,
            lexer: Some(lexer),
            stack: Some(Vec::new()),
            _phantom: PhantomData,
        }
    }

    fn next(&mut self) -> Option<()> {
        let lexer = self.lexer.as_mut().unwrap();
        let stack = self.stack.as_mut().unwrap();
        loop {
            let input = lexer.next();
            loop {
                let top = stack[stack.len() - 1];
                let action = match input {
                    Some(token) => (
                        self.tables.action_table[top].get(&token.kind).unwrap(),
                        Some(token),
                    ),
                    None => (
                        &self.tables.eof_action_table[top],
                        None
                    ),
                };
                match action {
                    (LRAction::Shift(new_state), Some(token)) => {
                        stack.push(*new_state);
                        // builder.push(token);
                        println!("Shift: {:?}", token);
                        break;
                    }
                    (LRAction::Reduce(tag, goto, elems_cnt), _) => {
                        stack.truncate(stack.len() - elems_cnt);
                        stack.push(self.tables.goto_table[stack[stack.len() - 1]][*goto]);
                        // builder.wrap(*tag, *elems_cnt);
                        println!("Reduce: {:?}", tag);
                    }
                    (LRAction::Accept, _) => {
                        // return builder.build();
                        return Some(());
                    }
                    (LRAction::None, Some(token)) => {
                        // return Err(ParseError::new_unexpected_token(token).into());
                        println!("Done!");
                        return None;
                    }
                    (LRAction::None, None) => {
                        // return Err(ParseError::UnexpectedEOF.into());
                        return None;
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
