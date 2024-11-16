use copager_cfl::token::TokenTag;
use copager_cfl::CFLTokens;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLTokens)]
enum MyToken {
    #[default]
    #[token(text = r"\+")]
    Abc,
    #[token(text = r"\-")]
    Def,
    #[token(text = r"[1-9]+")]
    Number,
}


#[test]
fn check_compile_tokens() {
    // CFLTokens
    let mytoken = MyToken::default();
    assert!(mytoken.ignore_token().is_empty());
    assert_eq!(mytoken.iter().count(), 3);

    // TokenTag
    assert_eq!(MyToken::Abc.as_str(), r"^\+");
    assert_eq!(MyToken::Def.as_str(), r"^\-");
    assert_eq!(MyToken::Number.as_str(), r"^[1-9]+");
}
