use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{ItemFn, FnArg};

pub fn proc_macro_impl_load(_args: TokenStream, ast: ItemFn) -> TokenStream {
    let fn_visibility = ast.vis;
    let fn_ident = ast.sig.ident;
    let fn_ret_type = ast.sig.output;
    let fn_body = ast.block;

    let fn_args = ast.sig.inputs;
    let fn_arg_processor = &fn_args[0];
    let fn_args_orig_def = fn_args
        .iter()
        .skip(1)
        .map(|fn_arg| {
            let fn_arg = fn_arg.to_token_stream();
            quote! { #fn_arg , }
        })
        .collect::<TokenStream>();
    let fn_args_orig_uses = fn_args
        .iter()
        .skip(1)
        .map(|fn_arg| {
            if let FnArg::Typed(pat_type) = fn_arg {
                let pat = pat_type.pat.to_token_stream();
                quote! { #pat , }
            } else {
                panic!("Unexpected argument type");
            }
        })
        .collect::<TokenStream>();

    quote! {
        fn #fn_ident (#fn_args_orig_def) #fn_ret_type {
            #fn_visibility fn __inner (#fn_arg_processor, #fn_args_orig_def) #fn_ret_type {
                #fn_body
            }

            let cache_body = include_str!(concat!(env!("OUT_DIR"), "/MyProcessor.cache"));
            let deserialized = copager::prebuild::deserialize(&cache_body).unwrap();
            __inner(deserialized, #fn_args_orig_uses)
        }
    }
}
