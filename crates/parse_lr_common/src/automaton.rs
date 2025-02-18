use copager_cfl::token::TokenTag;
use copager_cfl::rule::RuleElem;

pub mod lr0;
pub mod lr1;
pub mod lalr1;

pub trait Automaton<'a: 'b, 'b, T: TokenTag + 'a> {
    fn len(&self) -> usize;
    fn edges(&'b self) -> impl Iterator<Item = &'b (usize, usize, &'a RuleElem<T>)>;
}
