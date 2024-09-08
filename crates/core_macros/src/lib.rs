mod r#impl;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn prebuild(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args: TokenStream = attr.into();
    let ast = parse_macro_input!(item as ItemFn);
    r#impl::prebuild::proc_macro_impl_prebuild(args, ast).into()
}

#[proc_macro_attribute]
pub fn load(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args: TokenStream = attr.into();
    let ast = parse_macro_input!(item as ItemFn);
    r#impl::load::proc_macro_impl_load(args, ast).into()
}
