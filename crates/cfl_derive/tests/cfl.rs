use copager_cfl::rule::{Rule, RuleElem, RuleSet, RuleTag};
use copager_cfl::token::{TokenSet, TokenTag};
use copager_cfl::CFL;

#[derive(CFL)]
struct MyLanguage (
    #[tokenset] MyToken,
    #[ruleset]  MyRule,
);

#[derive(Clone, Hash, PartialEq, Eq, TokenSet)]
enum MyToken {
    #[token(r"a")]
    A,
}

#[derive(Clone, Hash, PartialEq, Eq, RuleSet)]
enum MyRule {
    #[tokenset(MyToken)]
    #[rule("<a> ::= A")]
    A,
}

#[test]
fn check_compile_cfl() {
    let _ = MyLanguage (
        MyToken::A,
        MyRule::A,
    );
}
