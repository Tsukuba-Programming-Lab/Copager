use copager::lex::LexSource;
use copager::parse::ParseSource;
use copager::prelude::*;
use copager::Language;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, LexSource)]
pub enum JsonToken {
    // 記号
    #[token(text = r"\:")]
    Colon,
    #[token(text = r"\,")]
    Comma,

    // キーワード
    #[token(text = r"true")]
    True,
    #[token(text = r"false")]
    False,
    #[token(text = r"null")]
    Null,

    // 識別子 & 数値
    #[token(text = r#""[a-zA-Z_][a-zA-Z0-9_]*""#)]
    String,
    #[token(text = r"\d+")]
    Number,

    // オブジェクト用括弧
    #[default]
    #[token(text = r"\{")]
    CurlyBracketL,
    #[token(text = r"\}")]
    CurlyBracketR,

    // 配列用括弧
    #[token(text = r"\[")]
    SquareBracketL,
    #[token(text = r"\]")]
    SquareBracketR,

    // 空白文字
    #[token(text = r"[ \t\n]+", ignored)]
    _Whitespace,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, ParseSource)]
pub enum JsonRule {
    // JSON本体
    #[default]
    #[rule("<json> ::= <json> <item>")]
    #[rule("<json> ::= <item>")]
    Json,

    #[rule("<item> ::= <object>")]
    #[rule("<item> ::= <array>")]
    Item,

    // 配列
    #[rule("<array> ::= SquareBracketL <value_list> SquareBracketR")]
    Array,

    #[rule("<value_list> ::= <value_list> Comma <value>")]
    #[rule("<value_list> ::= <value>")]
    #[rule("<value_list> ::= ")]
    ValueList,

    // オブジェクト
    #[rule("<object> ::= CurlyBracketL <key_value_list> CurlyBracketR")]
    Object,

    #[rule("<key_value_list> ::= <key_value_list> Comma <key_value>")]
    #[rule("<key_value_list> ::= <key_value>")]
    #[rule("<key_value_list> ::= ")]
    KeyValueList,

    #[rule("<key_value> ::= <key> Colon <value>")]
    KeyValue,

    #[rule("<key> ::= String")]
    Key,

    #[rule("<value> ::= <object>")]
    #[rule("<value> ::= <array>")]
    #[rule("<value> ::= String")]
    #[rule("<value> ::= Number")]
    #[rule("<value> ::= True")]
    #[rule("<value> ::= False")]
    #[rule("<value> ::= Null")]
    Value,
}

pub type Json = Language<JsonToken, JsonRule>;
