use copager_cfg::token::{TokenTag, Token};
use copager_lex::{LexSource, BaseLexer};
use copager_lex_regex::RegexLexer;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, LexSource)]
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

type MyLexer = RegexLexer<ExprToken>;

#[test]
fn simple_success() {
    let source = ExprToken::default();
    let lexer = <MyLexer as BaseLexer<ExprToken>>::try_from(source).unwrap();
    let mut lexer = lexer.run("1 + 2 * 3");
    assert_eq_token(lexer.next(), "1");
    assert_eq_token(lexer.next(), "+");
    assert_eq_token(lexer.next(), "2");
    assert_eq_token(lexer.next(), "*");
    assert_eq_token(lexer.next(), "3");
    assert!(lexer.next().is_none());
}

#[test]
#[should_panic]
fn simple_failed() {
    let source = ExprToken::default();
    let lexer = <MyLexer as BaseLexer<ExprToken>>::try_from(source).unwrap();
    let mut lexer = lexer.run("1 + 2 * stop 3");
    assert_eq_token(lexer.next(), "1");
    assert_eq_token(lexer.next(), "+");
    assert_eq_token(lexer.next(), "2");
    assert_eq_token(lexer.next(), "*");
    assert_eq_token(lexer.next(), "3");
    assert!(lexer.next().is_none());
}

fn assert_eq_token(token: Option<Token<ExprToken>>, s: &str) {
    match token {
        Some(token) => assert_eq!(token.as_str(), s),
        None => panic!("unexpected eof"),
    }
}
