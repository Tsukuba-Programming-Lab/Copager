pub use copager_core::*;
pub use copager_cfl as cfl;

#[cfg(feature = "prebuild")]
pub use copager_core_macros::*;

#[cfg(feature = "prebuild")]
pub mod prebuild {
    pub use serde_json::to_string as serialize;
    pub use serde_json::from_str as deserialize;
}

pub mod lex {
    pub use copager_lex::*;
    #[cfg(feature = "regexlex")]
    pub use copager_lex_regex::*;
}

pub mod parse {
    pub use copager_parse::*;
    #[cfg(feature = "lr0")]
    pub use copager_parse_lr_lr0::*;
    #[cfg(feature = "lr1")]
    pub use copager_parse_lr_lr1::*;
    #[cfg(feature = "slr1")]
    pub use copager_parse_lr_slr1::*;
    #[cfg(feature = "lalr1")]
    pub use copager_parse_lr_lalr1::*;
}

pub mod ir {
    pub use copager_ir::*;
    #[cfg(feature = "void")]
    pub use copager_ir_void::*;
    #[cfg(feature = "sexp")]
    pub use copager_ir_sexp::*;
}

pub mod prelude {
    pub use copager_cfl::rule::{Rule, RuleElem, RuleTag};
    pub use copager_cfl::token::TokenTag;
}

#[cfg(feature = "dev")]
pub mod dev {
    pub use copager_parse_common::*;
    pub use copager_parse_lr_common as lr;
}
