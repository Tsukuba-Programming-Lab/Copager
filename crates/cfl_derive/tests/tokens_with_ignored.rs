use copager_cfl::token::TokenTag;
use copager_cfl::CFLTokens;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLTokens)]
enum MyToken {
    #[default]
    #[token(text = r"\+")]
    #[token(text = r"plus")]
    Plus,
    #[token(text = r"\-")]
    #[token(text = r"minus")]
    Minus,
    #[token(text = r"[1-9]+")]
    Number,
    #[token(ignored)]
    #[token(text = r"[ \t\n]+")]
    _WhiteSpace,
}


#[test]
fn check_compile_tokens_with_ignored() {
    // CFLTokens
    let mytoken = MyToken::default();
    assert_eq!(mytoken.ignore_tokens(), &[r"[ \t\n]+"]);
    assert_eq!(mytoken.iter().count(), 3);

    // TokenTag
    assert_eq!(MyToken::Plus.as_str_list(), &[r"\+", r"plus"]);
    assert_eq!(MyToken::Minus.as_str_list(), &[r"\-", r"minus"]);
    assert_eq!(MyToken::Number.as_str_list(), &[r"[1-9]+"]);
    assert_eq!(MyToken::_WhiteSpace.as_str_list(), &[r"[ \t\n]+"]);
}
