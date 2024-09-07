use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn prebuild(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn load(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
