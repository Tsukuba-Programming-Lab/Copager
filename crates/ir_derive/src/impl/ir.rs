use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Ident, Generics};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let ident_builder = Ident::new(&format!("{}Builder", ident), ident.span());
    let generics = to_generics_without_where(&ast.generics);

    quote! {
        impl<'input, Lang: CFL> IR<'input, Lang> for #ident #generics {
            type Builder = #ident_builder<'input, Lang>;
        }
    }
}

fn to_generics_without_where(generics: &Generics) -> TokenStream {
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
