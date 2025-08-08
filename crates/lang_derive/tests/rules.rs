use copager_lang::rule::{Rule, RuleElem, RuleSet, RuleTag};
use copager_lang::token::{TokenSet, TokenTag};

#[derive(Clone, Hash, PartialEq, Eq, TokenSet)]
enum MyToken {
    #[token(r"\+")]
    Plus,
    #[token(r"\-")]
    Minus,
    #[token(r"[1-9]+")]
    Number,
}

#[derive(Clone, Hash, PartialEq, Eq, RuleSet)]
enum MyRule {
    #[tokenset(MyToken)]
    #[rule("<expr> ::= <expr> Plus Number")]
    #[rule("<expr> ::= <expr> Minus Number")]
    #[rule("<expr> ::= Number")]
    Expr,
}

#[test]
fn check_compile_rules() {
    // RuleSet
    let myrule = MyRule::instantiate();
    assert_eq!(myrule.iter().count(), 1);

    // RuleTag
    let rules = MyRule::Expr.as_rules();
    assert_eq!(rules.len(), 3);
    assert_eq!(rules[0].lhs, RuleElem::new_nonterm("expr"));
    assert_eq!(rules[0].rhs, vec![RuleElem::new_nonterm("expr"), RuleElem::new_term(MyToken::Plus), RuleElem::new_term(MyToken::Number)]);
    assert_eq!(rules[1].lhs, RuleElem::new_nonterm("expr"));
    assert_eq!(rules[1].rhs, vec![RuleElem::new_nonterm("expr"), RuleElem::new_term(MyToken::Minus), RuleElem::new_term(MyToken::Number)]);
    assert_eq!(rules[2].lhs, RuleElem::new_nonterm("expr"));
    assert_eq!(rules[2].rhs, vec![RuleElem::new_term(MyToken::Number)]);
}
