use copager::ir::r#ref::CSTreeWalker;

use crate::ast::{Assign, Decl, Print};
use crate::eval::{Env, Eval};
use crate::syntax::{EasyArith, EARule};

#[derive(Debug)]
pub enum Stmt<'input> {
    Decl(Decl<'input>),
    Assign(Assign<'input>),
    Print(Print<'input>),
}

impl<'input> From<CSTreeWalker<'input, EasyArith>> for Stmt<'input> {
    fn from(mut walker: CSTreeWalker<'input, EasyArith>) -> Self {
        match walker.peek().1.unwrap() {
            EARule::Decl => Stmt::Decl(walker.expect_node()),
            EARule::Assign => Stmt::Assign(walker.expect_node()),
            EARule::Print => Stmt::Print(walker.expect_node()),
            _ => unreachable!(),
        }
    }
}

impl<'input> Eval<'input> for Stmt<'input> {
    fn eval(&self, env: &mut Env<'input>) -> i32 {
        match self {
            Stmt::Decl(decl) => decl.eval(env),
            Stmt::Assign(assign) => assign.eval(env),
            Stmt::Print(print) => print.eval(env),
        }
    }
}
