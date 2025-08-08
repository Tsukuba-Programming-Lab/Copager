mod r#impl;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CFL, attributes(tokenset, ruleset))]
pub fn derive_cfl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::cfl::proc_macro_impl(ast).into()
}

#[proc_macro_derive(CFLToken, attributes(token))]
pub fn derive_cfl_token(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::token::proc_macro_impl(ast).into()
}

#[proc_macro_derive(CFLRule, attributes(tokenset, rule))]
pub fn derive_cfl_rule(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::rule::proc_macro_impl(ast).into()
}
