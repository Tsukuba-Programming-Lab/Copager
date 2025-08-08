#![feature(gen_blocks)]

use regex::{Regex, RegexSet};

use copager_lang::token::{Token, TokenSet, TokenTag};
use copager_lang::Lang;
use copager_lex::BaseLexer;

#[derive(Debug)]
pub struct RegexLexer<L: Lang> {
    regex_pre_trivia: Option<Regex>,
    regex_post_trivia: Option<Regex>,
    regex_set: RegexSet,
    regex_map: Vec<(Regex, L::TokenTag)>,
}

impl<L: Lang> BaseLexer<L> for RegexLexer<L> {
    fn init() -> anyhow::Result<Self> {
        let tokens = L::TokenSet::instantiate();

        // Trivia 用正規表現の準備
        let regex_pre_trivia = get_regex_by_opts(&tokens, "pre_trivia")?
            .or(get_regex_by_opts(&tokens, "trivia")?);
        let regex_post_trivia = get_regex_by_opts(&tokens, "post_trivia")?;

        // トークンに対応する正規表現集合の準備
        let regex_set = tokens.iter()
            .filter(|token| {
                let opts = token.as_option_list();
                !opts.contains(&"pre_trivia") && !opts.contains(&"trivia") && !opts.contains(&"post_trivia")
            })
            .map(|token| to_or_regex(token.as_str_list()))
            .collect::<Vec<_>>();
        let regex_set = RegexSet::new(regex_set)?;

        // regex_set の結果からの逆引きで使用するためのマップの用意
        let regex_map = tokens.iter()
            .map(|token| Ok((Regex::new(&to_or_regex(token.as_str_list()))?, token)))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(RegexLexer {
            regex_pre_trivia,
            regex_post_trivia,
            regex_set,
            regex_map,
        })
    }

    gen fn run<'input>(&self, input: &'input str) -> Token<'input, L::TokenTag> {
        let mut pos = 0;
        loop {
            match self.extract_token(input, pos) {
                Some(token) => {
                    pos = token.full.1;
                    yield token;
                }
                None => return,
            }
        }
    }
}

impl<'input, L: Lang> RegexLexer<L> {
    fn extract_token(&self, src: &'input str, begin: usize) -> Option<Token<'input, L::TokenTag>> {
        let full_begin = begin;
        let pre_trivia_end = full_begin + self.pre_trivia_len(&src[full_begin..]);

        let body_begin = pre_trivia_end;
        let (kind, accepted) = self
            .regex_set
            .matches(&src[body_begin..])
            .into_iter()
            .map(|idx| &self.regex_map[idx])
            .map(|(regex, token)| {
                let accepted = regex.find(&src[body_begin..]).unwrap().as_str();
                (token.clone(), accepted)
            })
            .next()?;
        let body_end = body_begin + accepted.len();

        let post_trivia_begin = body_end;
        let full_end = body_end + self.post_trivia_len(&src[post_trivia_begin..]);

        Some(Token {
            kind,
            src,
            body: (body_begin, body_end),
            full: (full_begin, full_end),
        })
    }

    fn pre_trivia_len(&self, s: &str) -> usize {
        if self.regex_pre_trivia.is_none() {
            return 0;
        }

        self.regex_pre_trivia
            .as_ref()
            .unwrap()
            .find(s)
            .and_then(|acc_s| Some(acc_s.as_str()))
            .unwrap_or("")
            .len()
    }

    fn post_trivia_len(&self, s: &str) -> usize {
        if self.regex_post_trivia.is_none() {
            return 0;
        }

        let found = self.regex_post_trivia
            .as_ref()
            .unwrap()
            .find(s)
            .and_then(|acc_s| Some(acc_s.as_str()))
            .unwrap_or("");
        match found {
            "" => 0,
            s if &s[s.len()-1..] == "\n" => found.len() - 1,
            _ => found.len(),
        }
    }
}

fn to_or_regex<T: AsRef<str>>(str_list: &[T]) -> String {
    let str_list = str_list.iter()
        .map(|s| s.as_ref())
        .collect::<Vec<_>>()
        .join("|");
    format!("^({})", str_list)
}

fn get_regex_by_opts<Ts: TokenSet>(tokens: &Ts, opt: &str) -> anyhow::Result<Option<Regex>> {
    let tokens = tokens.iter()
        .filter(|token| token.as_option_list().contains(&opt))
        .map(|token| token.as_str_list().join("|"))
        .collect::<Vec<_>>();
    if tokens.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Regex::new(&to_or_regex(&tokens))?))
    }
}
