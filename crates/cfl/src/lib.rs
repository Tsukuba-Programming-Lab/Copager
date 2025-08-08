pub mod rule;
pub mod token;

use token::{TokenTag, TokenSet};
use rule::{RuleTag, RuleSet};

#[cfg(feature = "derive")]
pub use copager_cfl_derive::CFL;

pub trait CFL {
    type TokenTag: TokenTag;
    type TokenSet: TokenSet<Tag = Self::TokenTag>;
    type RuleTag: RuleTag<Self::TokenTag>;
    type RuleSet: RuleSet<Self::TokenTag, Tag = Self::RuleTag>;
}
