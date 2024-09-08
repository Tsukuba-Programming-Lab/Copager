mod r#impl;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ParseSource, attributes(rule))]
pub fn derive_parse_source(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::rule::proc_macro_impl(ast).into()
}
