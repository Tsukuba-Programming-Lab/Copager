#![feature(gen_blocks)]

use regex::{Regex, RegexSet};

use copager_cfg::token::{TokenTag, Token};
use copager_lex::{LexSource, LexDriver};

#[derive(Debug)]
pub struct RegexLexer<S: LexSource> {
    regex_istr: Regex,
    regex_set: RegexSet,
    regex_map: Vec<(Regex, S::Tag)>,
}

impl<S: LexSource> LexDriver<S> for RegexLexer<S> {
    fn try_from(source: S) -> anyhow::Result<Self> {
        let regex_istr = Regex::new(source.ignore_token())?;
        let regex_set = source.iter()
            .map(|token| token.as_str())
            .collect::<Vec<_>>();
        let regex_set = RegexSet::new(regex_set)?;
        let regex_map = source.iter()
            .map(|token| Ok((Regex::new(token.as_str())?, token)))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(RegexLexer {
            regex_istr,
            regex_set,
            regex_map,
        })
    }

    gen fn run<'input>(&self, input: &'input str) -> Token<'input, S::Tag> {
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
            let matched = self
                .regex_set
                .matches(remain)
                .into_iter()
                .map(|idx| &self.regex_map[idx])
                .map(|(regex, token)| (*token, regex.find(remain).unwrap().as_str()))
                .next();

            // Update pos
            let (token, acc_s) = match matched {
                Some(a) => a,
                None => return,
            };
            let range = (pos, pos + acc_s.len());
            pos += acc_s.len();

            yield Token::new(token, &input, range);
        }
    }
}
