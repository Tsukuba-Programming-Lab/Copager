use copager_cfl::rule::{RuleTag, Rule, RuleElem};
use copager_cfl::token::TokenTag;
use copager_cfl::{CFL, CFLTokens, CFLRules};

#[allow(dead_code)]
#[derive(CFL)]
struct MyLanguage (
    #[tokens] MyToken,
    #[rules]  MyRule,
);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, CFLTokens)]
enum MyToken {
    #[token(text = r"a")]
    A,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, CFLRules)]
enum MyRule {
    #[rule("<a> ::= A")]
    A,
}

#[test]
fn check_compile_cfl() {}
