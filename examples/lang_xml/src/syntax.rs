use copager::lang::{RuleSet, TokenSet, Lang};
use copager::prelude::*;

#[derive(Lang)]
pub struct Xml (
    #[tokenset] XmlToken,
    #[ruleset]  XmlRule,
);

#[derive(Debug, Clone, Hash, PartialEq, Eq, TokenSet)]
pub enum XmlToken {
    // 記号
    #[token(r"<", ir_omit)]
    TagL,
    #[token(r">", ir_omit)]
    TagR,
    #[token(r"/", ir_omit)]
    Slash,
    #[token(r"=", ir_omit)]
    Equal,

    // 文字列 & 識別子
    #[token(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    String,
    #[token(r"'[a-zA-Z_][a-zA-Z0-9_]*'")]
    QuotedString,
    #[token(r#""[a-zA-Z_][a-zA-Z0-9_]*""#)]
    WQuotedString,

    // 空白文字
    #[token(r"[ \t\n]+", trivia)]
    _Whitespace,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, RuleSet)]
pub enum XmlRule {
    // 字句集合
    #[tokenset(XmlToken)]

    // XML本体
    #[rule("<xml> ::= <tag_list>")]
    Xml,

    // タグ
    #[rule("<tag_list> ::= <tag_list> <tag>")]
    #[rule("<tag_list> ::= <tag>")]
    TagList,

    #[rule("<tag> ::= <begin> <value_list> <end>")]
    #[rule("<tag> ::= <begin> <end>")]
    #[rule("<tag> ::= <single>")]
    Tag,

    #[rule("<single> ::= TagL String <attr_list> Slash TagR")]
    #[rule("<single> ::= TagL String Slash TagR")]
    Single,

    #[rule("<begin> ::= TagL String <attr_list> TagR")]
    #[rule("<begin> ::= TagL String TagR")]
    Begin,

    #[rule("<end> ::= TagL Slash String TagR")]
    End,

    // 属性
    #[rule("<attr_list> ::= <attr_list> <attr>")]
    #[rule("<attr_list> ::= <attr>")]
    AttrList,

    #[rule("<attr> ::= String Equal QuotedString")]
    #[rule("<attr> ::= String Equal WQuotedString")]
    Attr,

    // 値
    #[rule("<value_list> ::= <value_list> <value>")]
    #[rule("<value_list> ::= <value>")]
    ValueList,

    #[rule("<value> ::= <tag>")]
    #[rule("<value> ::= String")]
    Value,
}
