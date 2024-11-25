use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Generics;

pub(crate) fn to_generics_without_where(generics: &Generics) -> TokenStream {
    let lifetimes = generics
        .lifetimes()
        .map(|lifetime| lifetime.lifetime.to_token_stream())
        .collect::<TokenStream>();

    let type_params = generics
        .type_params()
        .map(|param| param.ident.to_token_stream())
        .collect::<TokenStream>();

    if lifetimes.is_empty() {
        quote! { <#type_params> }
    } else {
        quote! { <#lifetimes, #type_params> }
    }
}
