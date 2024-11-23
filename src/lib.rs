pub use copager_core::*;

pub mod cfl {
    pub use copager_cfl::*;
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
    #[cfg(feature = "json")]
    pub use copager_ir_json::*;
    #[cfg(feature = "sexp")]
    pub use copager_ir_sexp::*;
    #[cfg(feature = "xml")]
    pub use copager_ir_xml::*;
}

pub mod prelude {
    pub use copager_cfl::rule::{Rule, RuleElem, RuleTag};
    pub use copager_cfl::token::TokenTag;
}

#[cfg(feature = "prebuild")]
pub use copager_core_macros::*;

#[cfg(feature = "prebuild")]
pub mod prebuild {
    pub use serde_json::to_string as serialize;
    pub use serde_json::from_str as deserialize;
}

#[cfg(feature = "template")]
pub mod template {
    use copager_core::Generator;
    use copager_lex_regex::RegexLexer;

    #[cfg(feature = "lr0")]
    pub type LR0<T> = Generator<T, RegexLexer<T>, copager_parse_lr_lr0::LR0<T>>;
    #[cfg(feature = "lr1")]
    pub type LR1<T> = Generator<T, RegexLexer<T>, copager_parse_lr_lr1::LR1<T>>;
    #[cfg(feature = "slr1")]
    pub type SLR1<T> = Generator<T, RegexLexer<T>, copager_parse_lr_slr1::SLR1<T>>;
    #[cfg(feature = "lalr1")]
    pub type LALR1<T> = Generator<T, RegexLexer<T>, copager_parse_lr_lalr1::LALR1<T>>;
}

#[cfg(feature = "dev")]
pub mod dev {
    pub use copager_parse_common::*;
    pub use copager_parse_lr_common as lr;
}
