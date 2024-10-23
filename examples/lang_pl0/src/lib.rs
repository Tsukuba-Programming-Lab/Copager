mod token;
mod rule;

use copager::Grammar;

pub use token::Pl0Token;
pub use rule::Pl0Rule;

pub type Pl0 = Grammar<Pl0Token, Pl0Rule>;
