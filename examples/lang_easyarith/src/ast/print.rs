use copager::ir::r#ref::CSTreeWalker;

use crate::ast::Expr;
use crate::eval::{Env, Eval};
use crate::syntax::EasyArith;

#[derive(Debug)]
pub struct Print<'input> {
    pub expr: Expr<'input>,
}

impl<'input> From<CSTreeWalker<'input, EasyArith>> for Print<'input> {
    fn from(mut walker: CSTreeWalker<'input, EasyArith>) -> Self {
        let _ = walker.expect_leaf();  // 'print'
        let expr = walker.expect_node();
        Print { expr }
    }
}

impl<'input> Eval<'input> for Print<'input> {
    fn eval(&self, env: &mut Env<'input>) -> i32 {
        let value = self.expr.eval(env);
        println!("{}", value);
        0
    }
}
