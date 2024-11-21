use copager::cfl::{CFL, CFLTokens, CFLRules};
use copager::template::LALR1;
use copager::prelude::*;

pub type Pl0 = LALR1<Pl0Lang>;

#[derive(Debug, Default, CFL)]
pub struct Pl0Lang (
    #[tokens] Pl0Token,
    #[rules]  Pl0Rule,
);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, CFLTokens)]
pub enum Pl0Token {
    // キーワード
    #[default]
    #[token(r"const", r"CONST")]
    Const,
    #[token(r"var", r"VAR")]
    Var,
    #[token(r"procedure", r"PROCEDURE")]
    Procedure,
    #[token(r"call", r"CALL")]
    Call,
    #[token(r"begin", r"BEGIN")]
    Begin,
    #[token(r"end", r"END")]
    End,
    #[token(r"if", r"IF")]
    If,
    #[token(r"then", r"THEN")]
    Then,
    #[token(r"while", r"WHILE")]
    While,
    #[token(r"do", r"DO")]
    Do,
    #[token(r"odd", r"ODD")]
    Odd,
    #[token(r"write", r"WRITE")]
    Write,
    #[token(r"read", r"READ")]
    Read,

    // 識別子と数値
    #[token(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[token(r"\d+")]
    Number,

    // 演算子と記号
    #[token(r"\+")]
    Plus,
    #[token(r"-")]
    Minus,
    #[token(r"\*")]
    Times,
    #[token(r"/")]
    Slash,
    #[token(r"=")]
    Eql,
    #[token(r"#")]
    Neq,
    #[token(r"<=")]
    Leq,
    #[token(r"<")]
    Lss,
    #[token(r">=")]
    Geq,
    #[token(r">")]
    Gtr,
    #[token(r"\(")]
    ParenL,
    #[token(r"\)")]
    ParenR,
    #[token(r",")]
    Comma,
    #[token(r"\.")]
    Period,
    #[token(r";")]
    Semicolon,
    #[token(r":=")]
    Becomes,

    // 空白
    #[token(r"[ \t\n\r]+", ignored)]
    _Whitespace,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, CFLRules)]
pub enum Pl0Rule {
    // プログラム本体
    #[default]
    #[rule("<program> ::= <block> Period")]
    Program,

    // ブロック
    #[rule("<block> ::= <decl_list> <stmt>")]
    #[rule("<block> ::= <stmt>")]
    Block,

    // 宣言
    #[rule("<decl_list> ::= <decl_list> <decl>")]
    #[rule("<decl_list> ::= <decl>")]
    DeclList,

    #[rule("<decl> ::= <const_decl>")]
    #[rule("<decl> ::= <var_decl>")]
    #[rule("<decl> ::= <proc_decl>")]
    Decl,

    // 定数宣言
    #[rule("<const_decl> ::= Const <const_def_list> Semicolon")]
    ConstDecl,

    #[rule("<const_def_list> ::= <const_def_list> Comma <const_def>")]
    #[rule("<const_def_list> ::= <const_def>")]
    ConstDefList,

    #[rule("<const_def> ::= Ident Eql Number")]
    ConstDef,

    // 変数宣言
    #[rule("<var_decl> ::= Var <ident_list> Semicolon")]
    VarDecl,

    #[rule("<ident_list> ::= <ident_list> Comma Ident")]
    #[rule("<ident_list> ::= Ident")]
    IdentList,

    // 手続き宣言
    #[rule("<proc_decl> ::= Procedure Ident Semicolon <block> Semicolon")]
    ProcDecl,

    // 文
    #[rule("<stmt> ::= <assign_stmt>")]
    #[rule("<stmt> ::= <call_stmt>")]
    #[rule("<stmt> ::= <begin_stmt>")]
    #[rule("<stmt> ::= <if_stmt>")]
    #[rule("<stmt> ::= <while_stmt>")]
    #[rule("<stmt> ::= <read_stmt>")]
    #[rule("<stmt> ::= <write_stmt>")]
    Stmt,

    #[rule("<assign_stmt> ::= Ident Becomes <expr>")]
    AssignStmt,

    #[rule("<call_stmt> ::= Call Ident")]
    CallStmt,

    #[rule("<begin_stmt> ::= Begin <stmt> <stmt_list> End")]
    #[rule("<begin_stmt> ::= Begin <stmt> End")]
    BeginStmt,

    #[rule("<stmt_list> ::= <stmt_list> Semicolon <stmt>")]
    #[rule("<stmt_list> ::= Semicolon <stmt>")]
    StmtList,

    #[rule("<if_stmt> ::= If <condition> Then <stmt>")]
    IfStmt,

    #[rule("<while_stmt> ::= While <condition> Do <stmt>")]
    WhileStmt,

    #[rule("<read_stmt> ::= Read ParenL Ident ParenR")]
    ReadStmt,

    #[rule("<write_stmt> ::= Write ParenL <expr> ParenR")]
    WriteStmt,

    // 式
    #[rule("<condition> ::= Odd <expr>")]
    #[rule("<condition> ::= <expr> <relop> <expr>")]
    Condition,

    #[rule("<relop> ::= Eql")]
    #[rule("<relop> ::= Neq")]
    #[rule("<relop> ::= Lss")]
    #[rule("<relop> ::= Leq")]
    #[rule("<relop> ::= Gtr")]
    #[rule("<relop> ::= Geq")]
    RelOp,

    #[rule("<expr> ::= <expr> Plus <term>")]
    #[rule("<expr> ::= <expr> Minus <term>")]
    #[rule("<expr> ::= <term>")]
    Expr,

    #[rule("<term> ::= <term> Times <factor>")]
    #[rule("<term> ::= <term> Slash <factor>")]
    #[rule("<term> ::= <factor>")]
    Term,

    #[rule("<factor> ::= Ident")]
    #[rule("<factor> ::= Number")]
    #[rule("<factor> ::= ParenL <expr> ParenR")]
    Factor,
}
