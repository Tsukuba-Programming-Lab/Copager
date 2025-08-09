use copager::ir::r#ref::CSTreeWalker;

use crate::ast::Fact;
use crate::eval::{Env, Eval};
use crate::syntax::{EasyArith, EARule};

#[derive(Debug)]
pub enum Term<'input> {
    Mul {
        lhs: Box<Term<'input>>,
        rhs: Fact<'input>,
    },
    Fact(Fact<'input>),
}

impl<'input> From<CSTreeWalker<'input, EasyArith>> for Term<'input> {
    fn from(mut walker: CSTreeWalker<'input, EasyArith>) -> Self {
        match walker.peek().1.unwrap() {
            EARule::Term => {
                let lhs = Box::new(walker.expect_node());
                let rhs = walker.expect_node();
                Term::Mul { lhs, rhs }
            }
            EARule::Fact => {
                let fact = walker.expect_node();
                Term::Fact(fact)
            }
            _ => unreachable!(),
        }
    }
}

impl<'input> Eval<'input> for Term<'input> {
    fn eval(&self, env: &mut Env<'input>) -> i32 {
        match self {
            Term::Mul { lhs, rhs } => {
                let lhs = lhs.eval(env);
                let rhs = rhs.eval(env);
                lhs * rhs
            }
            Term::Fact(fact) => fact.eval(env),
        }
    }
}
