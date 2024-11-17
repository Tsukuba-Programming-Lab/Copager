use copager_cfl::token::{TokenTag, Token};
use copager_cfl::rule::{Rule, RuleTag, RuleElem};
use copager_cfl::{CFL, CFLTokens, CFLRules};
use copager_lex::BaseLexer;
use copager_lex_regex::RegexLexer;

#[derive(Default, CFL)]
struct TestLang (
    #[tokens] TestToken,
    #[rules]  TestRule,
);

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLTokens)]
enum TestToken {
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

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLRules)]
enum TestRule {
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

type MyLexer = RegexLexer<TestLang>;

#[test]
fn simple_success() {
    let cfl = TestLang::default();
    let lexer = <MyLexer as BaseLexer<TestLang>>::try_from(&cfl).unwrap();
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
    let cfl = TestLang::default();
    let lexer = <MyLexer as BaseLexer<TestLang>>::try_from(&cfl).unwrap();
    let mut lexer = lexer.run("1 + 2 * stop 3");
    assert_eq_token(lexer.next(), "1");
    assert_eq_token(lexer.next(), "+");
    assert_eq_token(lexer.next(), "2");
    assert_eq_token(lexer.next(), "*");
    assert_eq_token(lexer.next(), "3");
    assert!(lexer.next().is_none());
}

fn assert_eq_token(token: Option<Token<TestToken>>, s: &str) {
    match token {
        Some(token) => assert_eq!(token.as_str(), s),
        None => panic!("unexpected eof"),
    }
}
