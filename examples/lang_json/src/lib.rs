use copager::cfl::{CFL, CFLRule, CFLToken};
use copager::template::LALR1;
use copager::prelude::*;

pub type Json = LALR1<JsonLang>;

#[derive(Debug, CFL)]
pub struct JsonLang (
    #[tokenset] JsonToken,
    #[ruleset]  JsonRule,
);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, CFLToken)]
pub enum JsonToken {
    // 記号
    #[token(r"\:", ir_omit)]
    Colon,
    #[token(r"\,", ir_omit)]
    Comma,

    // キーワード
    #[token(r"true")]
    True,
    #[token(r"false")]
    False,
    #[token(r"null")]
    Null,

    // 識別子 & 数値
    #[token(r#""[a-zA-Z_][a-zA-Z0-9_]*""#)]
    String,
    #[token(r"\d+")]
    Number,

    // オブジェクト用括弧
    #[token(r"\{", ir_omit)]
    CurlyBracketL,
    #[token(r"\}", ir_omit)]
    CurlyBracketR,

    // 配列用括弧
    #[token(r"\[", ir_omit)]
    SquareBracketL,
    #[token(r"\]", ir_omit)]
    SquareBracketR,

    // 空白文字
    #[token(r"[ \t\n]+", trivia)]
    _Whitespace,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, CFLRule)]
pub enum JsonRule {
    // 字句集合
    #[tokenset(JsonToken)]

    // JSON本体
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
