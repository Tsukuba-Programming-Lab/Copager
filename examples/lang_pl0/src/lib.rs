mod token;
mod rule;

use copager::cfl::CFL;
use copager::template::LALR1;

pub use token::Pl0Token;
pub use rule::Pl0Rule;

pub type Pl0 = LALR1<Pl0Lang>;

#[derive(Debug, Default, CFL)]
pub struct Pl0Lang (
    #[tokens] Pl0Token,
    #[rules]  Pl0Rule,
);
