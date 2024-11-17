use copager::cfl::CFLRules;
use copager::prelude::*;

use crate::token::Pl0Token;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, CFLRules)]
pub enum Pl0Rule {
    // プログラム本体
    #[default]
    #[rule("<program> ::= <block> Period")]
    Program,

    // ブロック
    #[rule("<block> ::= <const_decl> <var_decl> <proc_decl_list> <stmt>")]
    Block,

    // 定数宣言
    #[rule("<const_decl> ::= Const <const_def_list> Semicolon")]
    #[rule("<const_decl> ::= ")]
    ConstDecl,

    #[rule("<const_def_list> ::= <const_def_list> Comma <const_def>")]
    #[rule("<const_def_list> ::= <const_def>")]
    ConstDefList,

    #[rule("<const_def> ::= Ident Eql Number")]
    ConstDef,

    // 変数宣言
    #[rule("<var_decl> ::= Var <ident_list> Semicolon")]
    #[rule("<var_decl> ::= ")]
    VarDecl,

    #[rule("<ident_list> ::= <ident_list> Comma Ident")]
    #[rule("<ident_list> ::= Ident")]
    IdentList,

    // 手続き宣言
    #[rule("<proc_decl_list> ::= <proc_decl_list> <proc_decl>")]
    #[rule("<proc_decl_list> ::= <proc_decl>")]
    #[rule("<proc_decl_list> ::= ")]
    ProcDeclList,

    #[rule("<proc_decl> ::= Procedure Ident Semicolon <stmt> Semicolon")]
    // #[rule("<proc_decl> ::= Procedure Ident Semicolon <block> Semicolon")]
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

    #[rule("<begin_stmt> ::= Begin <stmt_list> Semicolon End")]
    BeginStmt,

    #[rule("<stmt_list> ::= <stmt_list> Semicolon <stmt>")]
    #[rule("<stmt_list> ::= <stmt>")]
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
