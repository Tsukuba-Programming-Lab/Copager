use copager::cfl::{CFLRules, CFLTokens, CFL};
use copager::template::LALR1;
use copager::prelude::*;

pub type Xml = LALR1<XmlLang>;

#[derive(Debug, Default, CFL)]
pub struct XmlLang (
    #[tokens] XmlToken,
    #[rules]  XmlRule,
);

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLTokens)]
pub enum XmlToken {
    // 記号
    #[default]
    #[token(r"<", ignore)]
    TagL,
    #[token(r">", ignore)]
    TagR,
    #[token(r"/", ignore)]
    Slash,
    #[token(r"=", ignore)]
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

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, CFLRules)]
pub enum XmlRule {
    // XML本体
    #[default]
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
