use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Variant, Ident, LitStr};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_enum = if let Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!("\"Tokenset\" proc-macro is only implemented for enum.")
    };

    let parsed_variantes = data_enum
        .variants
        .iter()
        .map(|variant| VariantInfo::parse(&ast.ident, variant))
        .collect::<Vec<_>>();

    let enum_name = &ast.ident;
    let enum_ignored = parsed_variantes
        .iter()
        .find(|variant| variant.ignored)
        .map(|variant| variant.regex.as_ref().unwrap().as_str())
        .unwrap_or("");
    let enum_variants = parsed_variantes
        .iter()
        .filter(|variant| !variant.ignored)
        .map(|variant| variant.gen_ident());
    let enum_regex_table = parsed_variantes
        .iter()
        .filter(|variant| !variant.ignored)
        .map(|variant| variant.gen_ident_with_regex());

    quote! {
        impl TokenSet<'_> for #enum_name {
            fn ignore_str() -> &'static str {
                #enum_ignored
            }

            fn enum_iter() -> impl Iterator<Item = Self> {
                vec![
                    #( #enum_variants, )*
                ].into_iter()
            }

            fn to_regex(&self) -> &'static str {
                match self {
                    #( #enum_regex_table, )*
                    _ => unimplemented!(),
                }
            }
        }
    }
}

#[derive(Debug)]
struct VariantInfo<'a> {
    parent_ident: &'a Ident,
    self_ident: &'a Ident,
    regex: Option<String>,
    ignored: bool,
}

impl<'a> VariantInfo<'a> {
    fn parse(parent_ident: &'a Ident, variant: &'a Variant) -> VariantInfo<'a> {
        let self_ident = &variant.ident;

        let mut regex = None;
        let mut ignored = false;
        for attr in &variant.attrs {
            let _ = attr.parse_nested_meta(|meta| {
                // #[...(regex = "...")]
                if meta.path.is_ident("regex") {
                    let raw_regex = meta.value()?.parse::<LitStr>()?.value();
                    regex = Some(format!("^{}", raw_regex));
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
            regex,
            ignored,
        }
    }

    fn gen_ident(&self) -> TokenStream {
        let parent_ident = self.parent_ident;
        let self_ident = self.self_ident;

        quote! { #parent_ident :: #self_ident }
    }

    fn gen_ident_with_regex(&self) -> TokenStream {
        let ident = self.gen_ident();
        match &self.regex {
            Some(regex) => quote! { #ident => #regex },
            None => quote! { unimplemented!() },
        }
    }
}
