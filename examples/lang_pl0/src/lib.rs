mod token;
mod rule;

use copager::Language;

pub use token::Pl0Token;
pub use rule::Pl0Rule;

pub type Pl0 = Language<Pl0Token, Pl0Rule>;
