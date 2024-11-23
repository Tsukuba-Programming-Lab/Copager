use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let ident_builder = Ident::new(&format!("{}Builder", ident), ident.span());

    quote! {
        impl<'input, Ts, Rs> IR<'input, Ts, Rs> for #ident<'input, Ts, Rs>
        where
            Ts: CFLTokens,
            Rs: CFLRules<Ts::Tag>,
        {
            type Builder = #ident_builder<'input, Ts, Rs>;
        }
    }
}
