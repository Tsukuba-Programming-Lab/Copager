use copager_lang::token::{Token, TokenSet, TokenTag};
use copager_lang::rule::{Rule, RuleElem, RuleSet, RuleTag};
use copager_lang::Lang;
use copager_lex::BaseLexer;
use copager_lex_regex::RegexLexer;

#[derive(Lang)]
struct TestLang (
    #[tokenset] TestToken,
    #[ruleset]  TestRule,
);

#[derive(Clone, Hash, PartialEq, Eq, TokenSet)]
enum TestToken {
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
    #[token(r"[ \t\n]+", trivia)]
    _Trivia,
}

#[derive(Clone, Hash, PartialEq, Eq, RuleSet)]
enum TestRule {
    #[tokenset(TestToken)]
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
fn with_trivia_success() {
    let lexer = MyLexer::init().unwrap();
    let lexer = lexer.run("1 + 2 * 3");
    assert_eq_tokens(lexer, &["1", "+", "2", "*", "3"]);
}

#[test]
fn with_trivia_failed() {
    let lexer = MyLexer::init().unwrap();
    let lexer = lexer.run("1 + 2 * stop 3");
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
