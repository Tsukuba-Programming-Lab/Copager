use serde::{Serialize, Deserialize};

use copager_cfg::token::TokenTag;
use copager_cfg::rule::{RuleTag, Rule, RuleElem};
use copager_lex::{LexSource, LexDriver};
use copager_lex_regex::RegexLexer;
use copager_parse::{ParseSource, ParseDriver};
use copager_parse_lr1::LR1;
use copager_utils::cache::Cacheable;

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    LexSource, Serialize, Deserialize
)]
enum ExprToken {
    #[default]
    #[token(text = r"\+")]
    Plus,
    #[token(text = r"-")]
    Minus,
    #[token(text = r"\*")]
    Mul,
    #[token(text = r"/")]
    Div,
    #[token(text = r"\(")]
    BracketL,
    #[token(text = r"\)")]
    BracketR,
    #[token(text = r"[1-9][0-9]*")]
    Num,
    #[token(text = r"[ \t\n]+", ignored)]
    _Whitespace,
}

#[derive(
    Debug, Default, Copy, Clone, Hash, PartialEq, Eq,
    ParseSource, Serialize, Deserialize
)]
enum ExprRule {
    #[default]
    #[rule("<expr> ::= <expr> Plus <term>")]
    #[rule("<expr> ::= <expr> Minus <term>")]
    #[rule("<expr> ::= <term>")]
    Expr,
    #[rule("<term> ::= <term> Mul <num>")]
    #[rule("<term> ::= <term> Div <num>")]
    #[rule("<term> ::= <num>")]
    Term,
    #[rule("<num> ::= BracketL <expr> BracketR")]
    #[rule("<num> ::= Num")]
    Num,
}

#[test]
fn simple_success() -> anyhow::Result<()> {
    let lexer = RegexLexer::from(ExprToken::default());
    let lexer = lexer.init("1 + 2 * 3");

    let parser_conf = LR1::new((ExprToken::default(), ExprRule::default()))?;
    let parser = LR1::from(&parser_conf);
    let parser = parser.init(lexer);

    assert_eq!(parser.count(), 0);

    Ok(())
}
