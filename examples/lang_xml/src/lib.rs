use copager::lex::LexSource;
use copager::parse::ParseSource;
use copager::prelude::*;
use copager::Language;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, LexSource)]
pub enum XmlToken {
    // 記号
    #[default]
    #[token(text = r"<")]
    TagL,
    #[token(text = r">")]
    TagR,
    #[token(text = r"/")]
    Slash,
    #[token(text = r"=")]
    Equal,

    // 文字列 & 識別子
    #[token(text = r"[a-zA-Z_][a-zA-Z0-9_]*")]
    String,
    #[token(text = r"'[a-zA-Z_][a-zA-Z0-9_]*'")]
    QuotedString,
    #[token(text = r#""[a-zA-Z_][a-zA-Z0-9_]*""#)]
    WQuotedString,

    // 空白文字
    #[token(text = r"[ \t\n]+", ignored)]
    _Whitespace,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, ParseSource)]
pub enum XmlRule {
    // XML本体
    #[default]
    #[rule("<xml> ::= <xml> <tag>")]
    #[rule("<xml> ::= <tag>")]
    Xml,

    // タグ
    #[rule("<tag> ::= <begin> <value> <end>")]
    #[rule("<tag> ::= <single>")]
    Tag,

    #[rule("<single> ::= TagL String <attr_list> Slash TagR")]
    Single,

    #[rule("<begin> ::= TagL String <attr_list> TagR")]
    Begin,

    #[rule("<end> ::= TagL Slash String TagR")]
    End,

    // 属性
    #[rule("<attr_list> ::= <attr_list> <attr>")]
    #[rule("<attr_list> ::= <attr>")]
    #[rule("<attr_list> ::= ")]
    AttrList,

    #[rule("<attr> ::= String Equal QuotedString")]
    #[rule("<attr> ::= String Equal WQuotedString")]
    Attr,

    // 値
    #[rule("<value> ::= <xml>")]
    #[rule("<value> ::= String")]
    Value,
}

pub type Xml = Language<XmlToken, XmlRule>;
