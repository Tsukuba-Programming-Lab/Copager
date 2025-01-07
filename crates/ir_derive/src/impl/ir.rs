use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

use crate::utils::to_generics_without_where;

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
