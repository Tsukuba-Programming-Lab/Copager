pub mod rule;
pub mod token;

use token::{TokenTag, TokenSet};
use rule::{RuleTag, RuleSet};

#[cfg(feature = "derive")]
pub use copager_lang_derive::Lang;

pub trait Lang {
    type TokenTag: TokenTag;
    type TokenSet: TokenSet<Tag = Self::TokenTag>;
    type RuleTag: RuleTag<Self::TokenTag>;
    type RuleSet: RuleSet<Self::TokenTag, Tag = Self::RuleTag>;
}
