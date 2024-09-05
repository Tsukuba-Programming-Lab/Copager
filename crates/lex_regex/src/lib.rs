#![feature(gen_blocks)]

use std::rc::Rc;

use regex::{Regex, RegexSet};

use copager_cfg::token::{TokenTag, Token};
use copager_lex::{LexSource, LexDriver};

#[derive(Debug)]
pub struct RegexLexer<S: LexSource> {
    regex_istr: Rc<Regex>,
    regex_set: Rc<RegexSet>,
    regex_map: Rc<Vec<(Regex, S::Tag)>>,
}

impl<T, S> From<S> for RegexLexer<S>
where
    T: TokenTag,
    S: LexSource<Tag = T>,
{
    fn from(source: S) -> Self { // TODO: -> try_from
        let regex_istr = Regex::new(source.ignore_token()).unwrap();
        let regex_set = source.iter()
            .map(|token| token.as_str())
            .collect::<Vec<_>>();
        let regex_set = RegexSet::new(regex_set).unwrap();
        let regex_map = source.iter()
            .map(|token| Ok((Regex::new(token.as_str())?, token)))
            .collect::<anyhow::Result<Vec<_>>>().unwrap();

        RegexLexer {
            regex_istr: Rc::new(regex_istr),
            regex_set: Rc::new(regex_set),
            regex_map: Rc::new(regex_map),
        }
    }
}

impl<T, S> LexDriver<T> for RegexLexer<S>
where
    T: TokenTag,
    S: LexSource<Tag = T>,
{
    type From = S;

    gen fn run<'input>(&self, input: &'input str) -> Token<'input, T> {
        let mut pos = 0;
        loop {
            // Skip Spaces
            let remain = match self.regex_istr.find(&input[pos..]) {
                Some(acc_s) => {
                    pos += acc_s.len();
                    &input[pos..]
                }
                None => &input[pos..]
            };

            // Find the token
            let mut matches = self
                .regex_set
                .matches(remain)
                .into_iter()
                .map(|idx| &self.regex_map[idx])
                .map(|(regex, token)| (*token, regex.find(remain).unwrap().as_str()))
                .collect::<Vec<(S::Tag, &str)>>();
            matches.sort_by(|(_, a), (_, b)| a.len().cmp(&b.len()));

            // Update pos
            let (token, acc_s) = match matches.first() {
                Some(a) => a,
                None => return,
            };
            let range = (pos, pos + acc_s.len());
            pos += acc_s.len();

            yield Token::new(*token, &input, range);
        }
    }
}
