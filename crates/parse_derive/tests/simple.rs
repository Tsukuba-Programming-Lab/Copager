use copager_cfg::rule::{RuleTag, Rule, RuleElem};
use copager_cfg::token::TokenTag;
use copager_lex::LexSource;
use copager_parse::ParseSource;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, LexSource)]
enum MyToken {
    #[token(text = r"\+")]
    Plus,
    #[token(text = r"\-")]
    Minus,
    #[token(text = r"[1-9]+")]
    Number,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, ParseSource)]
enum MyRule {
    #[default]
    #[rule("<expr> ::= <expr> Plus Number")]
    #[rule("<expr> ::= <expr> Minus Number")]
    #[rule("<expr> ::= Number")]
    Expr,
}

#[test]
fn check_compile_simple() {
    // ParseSource
    let myrule = MyRule::default();
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
