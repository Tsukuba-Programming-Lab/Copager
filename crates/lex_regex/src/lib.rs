use std::marker::PhantomData;

use regex::{Regex, RegexSet};

use copager_cfg::token::Token;
use copager_cfg::TokenKind;
use copager_lex::LexIterator;

struct RegexLexer<'a, 'b, T: TokenKind<'a>> {
    // Regex
    regex_istr: Regex,
    regex_set: RegexSet,
    regex_map: Vec<(Regex, T)>,

    // State
    input: &'b str,
    pos: usize,

    // PhantomData
    _phantom: PhantomData<&'a T>,
}

impl<'a, 'b, T: TokenKind<'a>> TryFrom<&'b str> for RegexLexer<'a, 'b, T> {
    type Error = anyhow::Error;

    fn try_from(input: &'b str) -> anyhow::Result<Self> {
        let regex_istr = Regex::new(T::ignore_str())?;
        let regex_set = T::into_iter()
            .map(|token| T::as_str(&token))
            .collect::<Vec<_>>();
        let regex_set = RegexSet::new(regex_set)?;
        let regex_map = T::into_iter()
            .map(|token| Ok((Regex::new(token.as_str())?, token)))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(RegexLexer {
            regex_istr,
            regex_set,
            regex_map,
            input,
            pos: 0,
            _phantom: PhantomData,
        })
    }
}

impl<'a, 'b, T: TokenKind<'a> + 'a> LexIterator<'a, 'b> for RegexLexer<'a, 'b, T> {
    type TokenKind = T;

    fn next(&mut self) -> Option<Token<'a, 'b, T>> {
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
