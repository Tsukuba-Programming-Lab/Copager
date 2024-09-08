pub use copager_core::*;
pub use copager_core_macros::*;
pub use copager_cfg as cfg;

#[cfg(any(feature = "all", feature = "prebuild"))]
pub mod prebuild {
    pub use serde_json::to_string as serialize;
    pub use serde_json::from_str as deserialize;
}

pub mod lex {
    pub use copager_lex::*;
    #[cfg(any(feature = "all", feature = "regexlex"))]
    pub use copager_lex_regex::*;
}

pub mod parse {
    pub use copager_parse::*;
    #[cfg(any(feature = "all", feature = "lr1"))]
    pub use copager_parse_lr1::*;
}

pub mod ir {
    pub use copager_ir::*;
    #[cfg(any(feature = "all", feature = "void"))]
    pub use copager_ir_void::*;
    #[cfg(any(feature = "all", feature = "sexp"))]
    pub use copager_ir_sexp::*;
}

pub mod prelude {
    pub use copager_cfg::rule::{RuleTag, Rule, RuleElem};
    pub use copager_cfg::token::TokenTag;
}
