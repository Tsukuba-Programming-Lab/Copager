use std::marker::PhantomData;

use regex::{Regex, RegexSet};

use crate::cfg::TokenSet;

#[derive(Debug, Copy, Clone)]
pub struct Token<'a, 'b, T: TokenSet<'a>> {
    pub kind: T,
    pub pos: (u32, u32),
    orig_txt: &'b str,
    tokenset: PhantomData<&'a T>,
}

impl<'a, 'b, T: TokenSet<'a>> Token<'a, 'b, T> {
    pub fn new(kind: T, orig_txt: &'b str, pos: (u32, u32)) -> Self {
        Token {
            kind,
            pos,
            orig_txt,
            tokenset: PhantomData,
        }
    }

    pub fn as_str(&self) -> &'b str {
        self.orig_txt
    }

    pub fn to_string(&self) -> String {
        self.orig_txt.to_string()
    }
}

pub(crate) struct Lexer;

impl Lexer {
    pub fn new<'a, 'b, T>(input: &'b str) -> anyhow::Result<impl Iterator<Item = Token<'a, 'b, T>>>
    where
        T: TokenSet<'a> + 'a,
    {
        LexDriver::<'a, 'b, T>::try_from(input)
    }
}

struct LexDriver<'a, 'b, T: TokenSet<'a>> {
    // Regex
    regex_set: RegexSet,
    regex_map: Vec<(Regex, T)>,
    regex_istr: Regex,

    // State
    input: &'b str,
    pos: (u32, u32),

    // PhantomData
    tokenset: PhantomData<&'a T>,
}

impl<'a, 'b, T: TokenSet<'a>> TryFrom<&'b str> for LexDriver<'a, 'b, T> {
    type Error = anyhow::Error;

    fn try_from(input: &'b str) -> anyhow::Result<Self> {
        let regex_map = T::try_into()?;
        let regex_set = regex_map
            .iter()
            .map(|(_, token)| T::to_regex(&token))
            .collect::<Vec<_>>();
        let regex_set = RegexSet::new(regex_set)?;
        let regex_istr = Regex::new(T::ignore_str())?;

        Ok(LexDriver {
            regex_set,
            regex_map,
            regex_istr,
            input,
            pos: (0, 0),
            tokenset: PhantomData,
        })
    }
}

impl<'a, 'b, T: TokenSet<'a> + 'a> Iterator for LexDriver<'a, 'b, T> {
    type Item = Token<'a, 'b, T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip spaces
        if let Some(acc_s) = self.regex_istr.find(self.input) {
            self.update_state(acc_s.as_str());
        }

        // Find the token
        let mut matches = self
            .regex_set
            .matches(self.input)
            .into_iter()
            .map(|idx| &self.regex_map[idx])
            .map(|(regex, token)| (*token, regex.find(self.input).unwrap().as_str()))
            .collect::<Vec<(T, &str)>>();
        matches.sort_by(|(_, a), (_, b)| a.len().cmp(&b.len()));

        // Update myself
        let (token, acc_s) = matches.first()?;
        let pos = self.pos;
        self.update_state(acc_s);

        Some(Token::new(*token, acc_s, pos))
    }
}

impl<'a, 'b, T: TokenSet<'a>> LexDriver<'a, 'b, T> {
    fn update_state(&mut self, acc_s: &str) {
        let (mut rows, mut cols) = self.pos;
        for c in acc_s.chars() {
            match c {
                '\n' => {
                    rows += 1;
                    cols = 0;
                }
                _ => {
                    cols += 1;
                }
            }
        }

        self.input = &self.input[acc_s.len()..];
        self.pos = (rows, cols);
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use crate::cfg::TokenSet;
    use super::Lexer;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
    enum TestToken {
        Num,
        Plus,
    }

    impl TokenSet<'_> for TestToken {
        fn ignore_str() -> &'static str {
            r"^[ \t\n]+"
        }

        fn enum_iter() -> Box<dyn Iterator<Item = Self>> {
            Box::new(vec![TestToken::Num, TestToken::Plus].into_iter())
        }

        fn to_regex(&self) -> &'static str {
            match self {
                TestToken::Num => r"^[1-9][0-9]*",
                TestToken::Plus => r"^\+",
            }
        }
    }

    fn check<'a, 'b>(
        expected: &Vec<(TestToken, &'b str, (u32, u32))>,
        input: &'b str,
    ) -> bool {
        Lexer::new::<TestToken>(input)
            .unwrap()
            .into_iter()
            .zip(expected.iter())
            .all(|(a, b)| a.kind == b.0 && a.pos == b.2 && a.orig_txt == b.1)
    }

    #[test]
    fn input_ok_1() {
        let expected = vec![
            (TestToken::Num, "10", (0, 0)),
            (TestToken::Plus, "+", (0, 2)),
            (TestToken::Num, "20", (0, 3)),
        ];
        let input = "10+20";
        assert!(check(&expected, input));
    }

    #[test]
    fn input_ok_2() {
        let expected = vec![
            (TestToken::Num, "10", (0, 12)),
            (TestToken::Plus, "+", (0, 15)),
            (TestToken::Num, "20", (1, 6)),
        ];
        let input = "            10 +\n      20     ";
        assert!(check(&expected, input));
    }

    #[test]
    fn input_ok_3() {
        let expected = vec![
            (TestToken::Num, "10", (0, 12)),
            (TestToken::Plus, "+", (0, 15)),
            (TestToken::Num, "20", (1, 6)),
        ];
        let input = "            10 +\n      20ffff30 - 40 * 50";
        assert!(check(&expected, input));
    }
}
