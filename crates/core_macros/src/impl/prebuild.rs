use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn proc_macro_impl_prebuild(_args: TokenStream, ast: ItemFn) -> TokenStream {
    let fn_visibility = ast.vis;
    let fn_ident = ast.sig.ident;
    let fn_args = ast.sig.inputs;
    let fn_ret_type = ast.sig.output;
    let fn_body = ast.block;

    quote! {
        fn #fn_ident () {
            #fn_visibility fn __inner (#fn_args) #fn_ret_type {
                #fn_body
            }

            let serialized = copager::prebuild::__serialize(&__inner()).unwrap();
            let out_dir = std::env::var_os("OUT_DIR").unwrap();
            let cache_path = std::path::Path::new(&out_dir).join("MyProcessor.cache");
            std::fs::write(cache_path, serialized).unwrap();
        }
    }
}
