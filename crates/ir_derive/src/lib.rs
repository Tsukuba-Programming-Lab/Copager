mod r#impl;
mod utils;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IR, attributes(tokens, rules))]
pub fn derive_ir(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::ir::proc_macro_impl(ast).into()
}

#[proc_macro_derive(IRBuilder, attributes(tokens, rules))]
pub fn derive_ir_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::builder::proc_macro_impl(ast).into()
}
