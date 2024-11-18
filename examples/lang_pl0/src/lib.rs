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
    #[token(text = r"(const|CONST)")]
    Const,
    #[token(text = r"(var|VAR)")]
    Var,
    #[token(text = r"(procedure|PROCEDURE)")]
    Procedure,
    #[token(text = r"(call|CALL)")]
    Call,
    #[token(text = r"(begin|BEGIN)")]
    Begin,
    #[token(text = r"(end|END)")]
    End,
    #[token(text = r"(if|IF)")]
    If,
    #[token(text = r"(then|THEN)")]
    Then,
    #[token(text = r"(while|WHILE)")]
    While,
    #[token(text = r"(do|DO)")]
    Do,
    #[token(text = r"(odd|ODD)")]
    Odd,
    #[token(text = r"(write|WRITE)")]
    Write,
    #[token(text = r"(read|READ)")]
    Read,

    // 識別子と数値
    #[token(text = r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[token(text = r"\d+")]
    Number,

    // 演算子と記号
    #[token(text = r"\+")]
    Plus,
    #[token(text = r"-")]
    Minus,
    #[token(text = r"\*")]
    Times,
    #[token(text = r"/")]
    Slash,
    #[token(text = r"=")]
    Eql,
    #[token(text = r"#")]
    Neq,
    #[token(text = r"<=")]
    Leq,
    #[token(text = r"<")]
    Lss,
    #[token(text = r">=")]
    Geq,
    #[token(text = r">")]
    Gtr,
    #[token(text = r"\(")]
    ParenL,
    #[token(text = r"\)")]
    ParenR,
    #[token(text = r",")]
    Comma,
    #[token(text = r"\.")]
    Period,
    #[token(text = r";")]
    Semicolon,
    #[token(text = r":=")]
    Becomes,

    // 空白
    #[token(text = r"[ \t\n\r]+", ignored)]
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
