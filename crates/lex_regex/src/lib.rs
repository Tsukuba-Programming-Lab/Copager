use regex::{Regex, RegexSet};

use copager_cfg::token::{TokenTag, Token};
use copager_lex::{LexSource, LexIterator};

#[derive(Debug)]
struct RegexLexer<'input, S: LexSource> {
    // regex
    regex_istr: Regex,
    regex_set: RegexSet,
    regex_map: Vec<(Regex, S::Tag)>,

    // state
    input: &'input str,
    pos: usize,
}

impl<'input, T, S> From<S> for RegexLexer<'input, S>
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
            regex_istr,
            regex_set,
            regex_map,
            input: "",
            pos: 0,
        }
    }
}

impl<'input, T, S> LexIterator<'input, T> for RegexLexer<'input, S>
where
    T: TokenTag,
    S: LexSource<Tag = T>,
{
    type From = S;

    fn init(&self, input: &'input str) -> Self {
        RegexLexer {
            regex_istr: self.regex_istr.clone(),
            regex_set: self.regex_set.clone(),
            regex_map: self.regex_map.clone(),
            input: input,
            pos: 0,
        }
    }

    fn next(&mut self) -> Option<Token<'input, T>> {
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
            .collect::<Vec<(S::Tag, &str)>>();
        matches.sort_by(|(_, a), (_, b)| a.len().cmp(&b.len()));

        // Update myself
        let (token, acc_s) = matches.first()?;
        let range = (self.pos, self.pos + acc_s.len());
        self.pos += acc_s.len();

        Some(Token::new(*token, &self.input, range))
    }
}
