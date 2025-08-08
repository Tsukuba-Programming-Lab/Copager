use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::token::TokenTag;
use copager_cfl::{CFL, CFLToken, CFLRule};

#[derive(CFL)]
struct MyLanguage (
    #[tokenset] MyToken,
    #[ruleset]  MyRule,
);

#[derive(Clone, Hash, PartialEq, Eq, CFLToken)]
enum MyToken {
    #[token(r"a")]
    A,
}

#[derive(Clone, Hash, PartialEq, Eq, CFLRule)]
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
