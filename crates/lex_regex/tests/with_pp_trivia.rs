use copager_lang::token::{TokenSet, TokenTag};
use copager_lang::rule::{Rule, RuleElem, RuleSet, RuleTag};
use copager_lang::Lang;
use copager_lex::BaseLexer;
use copager_lex_regex::RegexLexer;

#[allow(dead_code)]
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
    #[token(r"^( |\t|\n|(//(.*)\n))*", pre_trivia)]
    #[token(r"^( |\t|)*(//(.*)\n)", post_trivia)]
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
fn with_pp_trivia_success() {
    const TEST_INPUT: &str = "
    // This is a comment
    // This is another comment
    1 + 2 * 3 // This is a comment
    ";

    let lexer = <MyLexer as BaseLexer<TestLang>>::init().unwrap();
    let lexed_tokens = lexer
        .run(TEST_INPUT)
        .collect::<Vec<_>>();
    let restored_input = lexed_tokens
        .into_iter()
        .map(|token| token.as_full_str())
        .collect::<String>();
    assert_eq!(restored_input, TEST_INPUT);
}
