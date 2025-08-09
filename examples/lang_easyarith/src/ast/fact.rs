use copager::ir::r#ref::CSTreeWalker;

use crate::ast::Expr;
use crate::eval::{Env, Eval};
use crate::syntax::{EasyArith, EAToken};

#[derive(Debug)]
pub enum Fact<'input> {
    Num(i32),
    Var(&'input str),
    Expr(Box<Expr<'input>>),
}

impl<'input> From<CSTreeWalker<'input, EasyArith>> for Fact<'input> {
    fn from(mut walker: CSTreeWalker<'input, EasyArith>) -> Self {
        match walker.peek().0 {
            Some(EAToken::Num) => {
                let s = walker.expect_leaf().1;
                let num = match s {
                    "0" => Ok(0),
                    _ if s.starts_with("0b") => i32::from_str_radix(&s[2..], 2),
                    _ if s.starts_with("0x") => i32::from_str_radix(&s[2..], 16),
                    _ if s.starts_with("0") => i32::from_str_radix(&s[1..], 8),
                    _ => s.parse::<i32>(),
                }.unwrap();
                Fact::Num(num)
            }
            Some(EAToken::Id) => {
                let var = walker.expect_leaf().1;
                Fact::Var(var)
            }
            _ => {
                let expr = walker.expect_node();
                Fact::Expr(Box::new(expr))
            }
        }
    }
}

impl<'input> Eval<'input> for Fact<'input> {
    fn eval(&self, env: &mut Env<'input>) -> i32 {
        match self {
            Fact::Num(num) => *num,
            Fact::Var(var) => {
                match env.vars.get(var) {
                    Some(&value) => value,
                    None => panic!("Variable '{}' not found", var),
                }
            }
            Fact::Expr(expr) => expr.eval(env),
        }
    }
}
