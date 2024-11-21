use copager_cfl::token::TokenTag;
use copager_cfl::CFLTokens;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLTokens)]
enum MyToken {
    #[default]
    #[token(r"\+", r"plus")]
    Plus,
    #[token(r"\-", r"minus")]
    Minus,
    #[token(r"[1-9]+")]
    Number,
}

#[test]
fn check_compile_tokens() {
    // CFLTokens
    let mytoken = MyToken::default();
    assert_eq!(mytoken.iter().count(), 3);

    // TokenTag
    assert_eq!(MyToken::Plus.as_str_list(), &[r"\+", r"plus"]);
    assert_eq!(MyToken::Plus.as_option_list().len(), 0);
    assert_eq!(MyToken::Minus.as_str_list(), &[r"\-", r"minus"]);
    assert_eq!(MyToken::Minus.as_option_list().len(), 0);
    assert_eq!(MyToken::Number.as_str_list(), &[r"[1-9]+"]);
    assert_eq!(MyToken::Number.as_option_list().len(), 0);
}
