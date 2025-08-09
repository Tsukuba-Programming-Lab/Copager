use copager::ir::r#ref::CSTreeWalker;

use crate::ast::Term;
use crate::eval::{Env, Eval};
use crate::syntax::{EasyArith, EARule};

#[derive(Debug)]
pub enum Expr<'input> {
    Plus {
        lhs: Box<Expr<'input>>,
        rhs: Term<'input>,
    },
    Term(Term<'input>),
}

impl<'input> From<CSTreeWalker<'input, EasyArith>> for Expr<'input> {
    fn from(mut walker: CSTreeWalker<'input, EasyArith>) -> Self {
        match walker.peek().1.unwrap() {
            EARule::Expr => {
                let lhs = Box::new(walker.expect_node());
                let rhs = walker.expect_node();
                Expr::Plus { lhs, rhs }
            }
            EARule::Term => {
                let term = walker.expect_node();
                Expr::Term(term)
            }
            _ => unreachable!(),
        }
    }
}

impl<'input> Eval<'input> for Expr<'input> {
    fn eval(&self, env: &mut Env<'input>) -> i32 {
        match self {
            Expr::Plus { lhs, rhs } => {
                let lhs = lhs.eval(env);
                let rhs = rhs.eval(env);
                lhs + rhs
            }
            Expr::Term(term) => term.eval(env),
        }
    }
}
