use copager_cfl::token::TokenTag;
use copager_lex::LexSource;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, LexSource)]
enum MyToken {
    #[default]
    #[token(text = r"\+")]
    Abc,
    #[token(text = r"\-")]
    Def,
    #[token(text = r"[1-9]+")]
    Number,
    #[token(text = r"[ \t\n]+", ignored)]
    _WhiteSpace,
}


#[test]
fn check_compile_with_ignored() {
    // LexSource
    let mytoken = MyToken::default();
    assert_eq!(mytoken.ignore_token(), r"^[ \t\n]+");
    assert_eq!(mytoken.iter().count(), 3);

    // TokenTag
    assert_eq!(MyToken::Abc.as_str(), r"^\+");
    assert_eq!(MyToken::Def.as_str(), r"^\-");
    assert_eq!(MyToken::Number.as_str(), r"^[1-9]+");
    assert_eq!(MyToken::_WhiteSpace.as_str(), r"^[ \t\n]+");
}
