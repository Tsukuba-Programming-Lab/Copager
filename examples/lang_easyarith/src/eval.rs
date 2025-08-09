use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Env<'input> {
    pub vars: HashMap<&'input str, i32>,
}

pub trait Eval<'input> {
    fn eval(&self, env: &mut Env<'input>) -> i32;
}

pub fn eval<'input>(ast: &impl Eval<'input>) -> i32 {
    let mut env = Env::default();
    ast.eval(&mut env)
}
