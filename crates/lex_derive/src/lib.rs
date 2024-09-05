mod r#impl;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(LexSource, attributes(token))]
pub fn derive_tokenset(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::lex::proc_macro_impl(ast).into()
}
