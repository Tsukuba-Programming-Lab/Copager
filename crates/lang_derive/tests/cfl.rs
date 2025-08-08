use copager_lang::rule::{Rule, RuleElem, RuleSet, RuleTag};
use copager_lang::token::{TokenSet, TokenTag};
use copager_lang::Lang;

#[derive(Lang)]
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
fn check_compile_lang() {
    let _ = MyLanguage (
        MyToken::A,
        MyRule::A,
    );
}
