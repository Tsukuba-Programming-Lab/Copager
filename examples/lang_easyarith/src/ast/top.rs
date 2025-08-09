use copager::ir::r#ref::CSTreeWalker;

use crate::ast::Stmt;
use crate::eval::{Env, Eval};
use crate::syntax::EasyArith;

#[derive(Debug)]
pub struct Top<'input> {
    pub stmts: Vec<Stmt<'input>>,
}

impl<'input> From<CSTreeWalker<'input, EasyArith>> for Top<'input> {
    fn from(mut walker: CSTreeWalker<'input, EasyArith>) -> Self {
        Top {
            stmts: walker.expect_nodes()
        }
    }
}

impl<'input> Eval<'input> for Top<'input> {
    fn eval(&self, env: &mut Env<'input>) -> i32 {
        for stmt in &self.stmts {
            stmt.eval(env);
        }
        0
    }
}
