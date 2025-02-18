use core::panic;

use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Ident, Variant};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_enum = if let Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!("\"CFLTokens\" proc-macro is only implemented for enum.")
    };

    let parsed_variantes = data_enum
        .variants
        .iter()
        .map(|variant| VariantInfo::parse(&ast.ident, variant))
        .collect::<Vec<_>>();

    let enum_name = &ast.ident;
    let enum_str_matcher_table = parsed_variantes
        .iter()
        .map(|variant| variant.gen_str_matcher());
    let enum_opts_matcher_table = parsed_variantes
        .iter()
        .map(|variant| variant.gen_option_matcher());
    let enum_variants = parsed_variantes
        .iter()
        .map(|variant| variant.gen_ident());

    quote! {
        impl TokenTag for #enum_name {
            fn as_str_list<'a, 'b>(&'a self) -> &'a[&'b str] {
                match self {
                    #( #enum_str_matcher_table, )*
                }
            }

            fn as_option_list<'a, 'b>(&'a self) -> &'a[&'b str] {
                match self {
                    #( #enum_opts_matcher_table, )*
                }
            }
        }

        impl CFLTokens for #enum_name {
            type Tag = Self;

            fn iter(&self) -> impl Iterator<Item = Self::Tag> {
                vec![ #( #enum_variants, )* ].into_iter()
            }
        }
    }
}

#[derive(Debug)]
struct VariantInfo<'a> {
    parent_ident: &'a Ident,
    self_ident: &'a Ident,
    texts: Vec<TokenStream>,
    options: Vec<TokenStream>,
}

impl<'a> VariantInfo<'a> {
    fn parse(parent_ident: &'a Ident, variant: &'a Variant) -> VariantInfo<'a> {
        let self_ident = &variant.ident;

        let mut texts = vec![];
        let mut options = vec![];
        for attr in variant.attrs.iter().filter(|attr| attr.path().is_ident("token")) {
            let meta_list = attr.meta.require_list().unwrap().tokens.clone();
            for meta in meta_list.into_iter() {
                match meta {
                    TokenTree::Literal(lit) => texts.push(lit.to_token_stream()),
                    TokenTree::Ident(ident) => options.push(ident.to_token_stream()),
                    _ => {},
                }
            }
        }

        VariantInfo {
            parent_ident,
            self_ident,
            texts,
            options,
        }
    }

    fn gen_ident(&self) -> TokenStream {
        let parent_ident = self.parent_ident;
        let self_ident = self.self_ident;
        quote! { #parent_ident :: #self_ident }
    }

    fn gen_str_matcher(&self) -> TokenStream {
        let ident = self.gen_ident();
        let str_list = &self.texts;
        quote! { #ident => &[#(#str_list,)*] }
    }

    fn gen_option_matcher(&self) -> TokenStream {
        let ident = self.gen_ident();
        let opt_list = &self.options;
        quote! { #ident => &[#(stringify!(#opt_list),)*] }
    }
}
