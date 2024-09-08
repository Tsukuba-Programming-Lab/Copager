use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn proc_macro_impl_load(_args: TokenStream, ast: ItemFn) -> TokenStream {
    let fn_visibility = ast.vis;
    let fn_ident = ast.sig.ident;
    let fn_args = ast.sig.inputs;
    let fn_ret_type = ast.sig.output;
    let fn_body = ast.block;

    quote! {
        fn #fn_ident () #fn_ret_type {
            #fn_visibility fn __inner (#fn_args) #fn_ret_type {
                #fn_body
            }

            let cache_body = include_str!(concat!(env!("OUT_DIR"), "/MyProcessor.cache"));
            let deserialized = copager::prebuild::deserialize(&cache_body).unwrap();
            __inner(deserialized)
        }
    }
}
