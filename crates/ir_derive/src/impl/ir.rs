use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let ident_builder = Ident::new(&format!("{}Builder", ident), ident.span());

    quote! {
        impl<'input, Lang: CFL> IR<'input, Lang> for #ident<'input, Lang> {
            type Builder = #ident_builder<'input, Lang>;
        }
    }
}
