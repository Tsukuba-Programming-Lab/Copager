use copager::ir::r#ref::CSTreeWalker;

use crate::eval::{Env, Eval};
use crate::syntax::EasyArith;

#[derive(Debug)]
pub struct Decl<'input> {
    name: &'input str,
}

impl<'input> From<CSTreeWalker<'input, EasyArith>> for Decl<'input> {
    fn from(mut walker: CSTreeWalker<'input, EasyArith>) -> Self {
        let name = walker.expect_leaf().1;
        Decl { name }
    }
}

impl<'input> Eval<'input> for Decl<'input> {
    fn eval(&self, env: &mut Env<'input>) -> i32 {
        env.vars.insert(self.name, 0);
        0
    }
}
