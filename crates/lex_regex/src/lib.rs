use regex::{Regex, RegexSet};

use copager_cfg::token::{TokenTag, Token};
use copager_lex::{LexSource, LexIterator};
use copager_utils::cache::Cacheable;

struct RegexLexer<'cache, 'input, T: TokenTag> {
    // regex
    regex_istr: &'cache Regex,
    regex_set: &'cache RegexSet,
    regex_map: &'cache Vec<(Regex, T)>,

    // state
    input: &'input str,
    pos: usize,
}

struct RegexLexerCache<T: TokenTag> {
    regex_istr: Regex,
    regex_set: RegexSet,
    regex_map: Vec<(Regex, T)>,
}

impl<'cache, 'input, T, S> Cacheable<'cache, S> for RegexLexer<'cache, 'input, T>
where
    T: TokenTag,
    S: LexSource<T>,
{
    type Cache = RegexLexerCache<T>;

    fn new(source: S) -> anyhow::Result<Self::Cache> {
        let regex_istr = Regex::new(source.ignore_token())?;
        let regex_set = source.iter()
            .map(|token| token.as_str())
            .collect::<Vec<_>>();
        let regex_set = RegexSet::new(regex_set)?;
        let regex_map = source.iter()
            .map(|token| Ok((Regex::new(token.as_str())?, token)))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(RegexLexerCache {
            regex_istr,
            regex_set,
            regex_map,
        })
    }

    fn restore(cache: &'cache Self::Cache) -> Self {
        RegexLexer {
            regex_istr: &cache.regex_istr,
            regex_set: &cache.regex_set,
            regex_map: &cache.regex_map,
            input: "",
            pos: 0,
        }
    }
}

impl<'cache, 'input, T, S> LexIterator<'cache, 'input, T, S> for RegexLexer<'cache, 'input, T>
where
    T: TokenTag,
    S: LexSource<T>,
{
    fn init(&self, input: &'input str) -> Self {
        RegexLexer {
            regex_istr: self.regex_istr,
            regex_set: self.regex_set,
            regex_map: self.regex_map,
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
            .collect::<Vec<(T, &str)>>();
        matches.sort_by(|(_, a), (_, b)| a.len().cmp(&b.len()));

        // Update myself
        let (token, acc_s) = matches.first()?;
        let range = (self.pos, self.pos + acc_s.len());
        self.pos += acc_s.len();

        Some(Token::new(*token, &self.input, range))
    }
}
