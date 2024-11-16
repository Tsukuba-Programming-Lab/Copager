mod r#impl;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CFLTokens, attributes(token))]
pub fn derive_cfl_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::token::proc_macro_impl(ast).into()
}

#[proc_macro_derive(CFLRules, attributes(rule))]
pub fn derive_cfl_rules(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::rule::proc_macro_impl(ast).into()
}
