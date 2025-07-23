use copager_cfl::token::{TokenTag, Token};
use copager_cfl::rule::{Rule, RuleTag, RuleElem};
use copager_cfl::{CFL, CFLToken, CFLRule};
use copager_lex::BaseLexer;
use copager_lex_regex::RegexLexer;

#[derive(Default, CFL)]
struct TestLang (
    #[tokens] TestToken,
    #[rules]  TestRule,
);

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLToken)]
enum TestToken {
    #[default]
    #[token(r"\+")]
    Plus,
    #[token(r"-")]
    Minus,
    #[token(r"\*")]
    Mul,
    #[token(r"/")]
    Div,
    #[token(r"\(")]
    BracketL,
    #[token(r"\)")]
    BracketR,
    #[token(r"[1-9][0-9]*")]
    Num,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLRule)]
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
    let lexer = lexer.run("1+2*3");
    assert_eq_tokens(lexer, &["1", "+", "2", "*", "3"]);
}

#[test]
fn simple_failed() {
    let cfl = TestLang::default();
    let lexer = <MyLexer as BaseLexer<TestLang>>::try_from(&cfl).unwrap();
    let lexer = lexer.run("1+2*stop3");
    assert_eq_tokens(lexer, &["1", "+", "2", "*"]);
}

fn assert_eq_tokens<'a, T, Il>(mut lexer: Il, expected: &[&str])
where
    T: TokenTag,
    Il: Iterator<Item = Token<'a, T>>,
{
    for expected_elem in expected {
        let token = lexer.next();
        match token {
            Some(token) => assert_eq!(&token.as_str(), expected_elem),
            None => panic!("unexpected eof"),
        }
    }
    assert!(lexer.next().is_none());
}
