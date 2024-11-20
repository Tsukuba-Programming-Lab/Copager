use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Variant, Ident, LitStr};

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
    let enum_matcher_table = parsed_variantes
        .iter()
        .map(|variant| variant.gen_matcher());
    let enum_ignored = parsed_variantes
        .iter()
        .find(|variant| variant.ignored)
        .map(|variant| variant.gen_str_list())
        .unwrap_or(quote!{ &[] });
    let enum_variants = parsed_variantes
        .iter()
        .filter(|variant| !variant.ignored)
        .map(|variant| variant.gen_ident());

    quote! {
        impl TokenTag for #enum_name {
            fn as_str_list<'a, 'b>(&'a self) -> &'a[&'b str] {
                match self {
                    #( #enum_matcher_table, )*
                }
            }
        }

        impl CFLTokens for #enum_name {
            type Tag = Self;

            fn ignore_tokens(&self) -> &[&'static str] {
                #enum_ignored
            }

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
    texts: Vec<String>,
    ignored: bool,
}

impl<'a> VariantInfo<'a> {
    fn parse(parent_ident: &'a Ident, variant: &'a Variant) -> VariantInfo<'a> {
        let self_ident = &variant.ident;

        let mut texts = vec![];
        let mut ignored = false;
        for attr in &variant.attrs {
            let _ = attr.parse_nested_meta(|meta| {
                // #[...(text = "...")]
                if meta.path.is_ident("text") {
                    let raw_text = meta.value()?.parse::<LitStr>()?.value();
                    texts.push(raw_text);
                    return Ok(());
                }

                // #[...(ignord)]
                if meta.path.is_ident("ignored") {
                    ignored = true;
                    return Ok(());
                }

                Err(meta.error("Unknown attribute"))
            });
        }

        VariantInfo {
            parent_ident,
            self_ident,
            texts,
            ignored,
        }
    }

    fn gen_ident(&self) -> TokenStream {
        let parent_ident = self.parent_ident;
        let self_ident = self.self_ident;

        quote! { #parent_ident :: #self_ident }
    }

    fn gen_str_list(&self) -> TokenStream {
        let texts = self.texts.iter().map(|text| {
            let text = LitStr::new(text, proc_macro2::Span::call_site());
            quote! { #text }
        });
        quote! { &[ #( #texts, )* ] }
    }

    fn gen_matcher(&self) -> TokenStream {
        let ident = self.gen_ident();
        let str_list = self.gen_str_list();
        quote! { #ident => #str_list }
    }
}
