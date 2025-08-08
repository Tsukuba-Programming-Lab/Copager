mod r#impl;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Lang, attributes(tokenset, ruleset))]
pub fn derive_lang(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::lang::proc_macro_impl(ast).into()
}

#[proc_macro_derive(TokenSet, attributes(token))]
pub fn derive_tokenset(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::token::proc_macro_impl(ast).into()
}

#[proc_macro_derive(RuleSet, attributes(tokenset, rule))]
pub fn derive_ruleset(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::rule::proc_macro_impl(ast).into()
}
