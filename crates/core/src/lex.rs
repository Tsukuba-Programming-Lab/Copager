use std::marker::PhantomData;

use regex::{Regex, RegexSet};

use crate::cfg::TokenSet;

#[derive(Debug, Copy, Clone)]
pub struct Token<'a, 'b, T: TokenSet<'a>> {
    pub kind: T,
    pub src: &'b str,
    pub range: (usize, usize),
    tokenset: PhantomData<&'a T>,
}

impl<'a, 'b, T: TokenSet<'a>> Token<'a, 'b, T> {
    pub fn new(kind: T, src: &'b str, range: (usize, usize)) -> Self {
        Token {
            kind,
            src,
            range,
            tokenset: PhantomData,
        }
    }

    pub fn as_str(&self) -> &'b str {
        let (l, r) = self.range;
        &self.src[l..r]
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
    pos: usize,

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
            pos: 0,
            tokenset: PhantomData,
        })
    }
}

impl<'a, 'b, T: TokenSet<'a> + 'a> Iterator for LexDriver<'a, 'b, T> {
    type Item = Token<'a, 'b, T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip Spaces
        let remain = match self.regex_istr.find(&self.input[self.pos..]) {
            Some(acc_s) => {
                self.pos += acc_s.len();
                &self.input[self.pos..]
            }
            None => &self.input[self.pos..]
        };

        // Find the token
        let mut matches = self
            .regex_set
            .matches(remain)
            .into_iter()
            .map(|idx| &self.regex_map[idx])
            .map(|(regex, token)| (*token, regex.find(remain).unwrap().as_str()))
            .collect::<Vec<(T, &str)>>();
        matches.sort_by(|(_, a), (_, b)| a.len().cmp(&b.len()));

        // Update myself
        let (token, acc_s) = matches.first()?;
        let range = (self.pos, self.pos + acc_s.len());
        self.pos += acc_s.len();

        Some(Token::new(*token, &self.input, range))
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
        expected: &Vec<(TestToken, &'b str, (usize, usize))>,
        input: &'b str,
    ) -> bool {
        Lexer::new::<TestToken>(input)
            .unwrap()
            .into_iter()
            .zip(expected.iter())
            .all(|(a, b)| {
                a.kind == b.0 && a.range == b.2 && a.as_str() == b.1
            })
    }

    #[test]
    fn input_ok_1() {
        let expected = vec![
            (TestToken::Num, "10", (0, 2)),
            (TestToken::Plus, "+", (2, 3)),
            (TestToken::Num, "20", (3, 5)),
        ];
        let input = "10+20";
        assert!(check(&expected, input));
    }

    #[test]
    fn input_ok_2() {
        let expected = vec![
            (TestToken::Num, "10", (12, 14)),
            (TestToken::Plus, "+", (15, 16)),
            (TestToken::Num, "20", (23, 25)),
        ];
        let input = "            10 +\n      20     ";
        assert!(check(&expected, input));
    }

    #[test]
    fn input_ok_3() {
        let expected = vec![
            (TestToken::Num, "10", (12, 14)),
            (TestToken::Plus, "+", (15, 16)),
            (TestToken::Num, "20", (23, 25)),
        ];
        let input = "            10 +\n      20ffff30 - 40 * 50";
        assert!(check(&expected, input));
    }
}
