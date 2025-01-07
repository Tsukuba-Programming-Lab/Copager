use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::token::TokenTag;
use copager_cfl::{CFL, CFLTokens, CFLRules};

#[derive(Default, CFL)]
struct MyLanguage (
    #[tokens] MyToken,
    #[rules]  MyRule,
);

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLTokens)]
enum MyToken {
    #[default]
    #[token(r"a")]
    A,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLRules)]
enum MyRule {
    #[default]
    #[rule("<a> ::= A")]
    A,
}

#[test]
fn check_compile_cfl() {
    let _ = MyLanguage::default();
}
