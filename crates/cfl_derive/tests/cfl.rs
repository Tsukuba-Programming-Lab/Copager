use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::token::TokenTag;
use copager_cfl::{CFL, CFLToken, CFLRule};

#[derive(Default, CFL)]
struct MyLanguage (
    #[tokenset] MyToken,
    #[ruleset]  MyRule,
);

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLToken)]
enum MyToken {
    #[default]
    #[token(r"a")]
    A,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLRule)]
enum MyRule {
    #[tokenset(MyToken)]
    #[default]
    #[rule("<a> ::= A")]
    A,
}

#[test]
fn check_compile_cfl() {
    let _ = MyLanguage::default();
}
