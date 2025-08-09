use copager::ir::r#ref::CSTreeWalker;

use crate::ast::Expr;
use crate::eval::{Env, Eval};
use crate::syntax::EasyArith;

#[derive(Debug)]
pub struct Assign<'input> {
    name: &'input str,
    expr: Expr<'input>,
}

impl<'input> From<CSTreeWalker<'input, EasyArith>> for Assign<'input> {
    fn from(mut walker: CSTreeWalker<'input, EasyArith>) -> Self {
        let name = walker.expect_leaf().1;
        let _ = walker.expect_leaf();  // '='
        let expr = walker.expect_node();
        Assign { name, expr }
    }
}

impl<'input> Eval<'input> for Assign<'input> {
    fn eval(&self, env: &mut Env<'input>) -> i32 {
        match env.vars.get(&self.name) {
            Some(_) => {
                let value = self.expr.eval(env);
                env.vars.insert(self.name, value);
            }
            None => {
                panic!("Variable '{}' not declared", self.name);
            }
        }
        0
    }
}
