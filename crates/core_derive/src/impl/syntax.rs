use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Variant, Ident, LitStr};

pub fn syntax_proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_enum = if let Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!("\"Syntax\" proc-macro is only implemented for enum.")
    };

    let parsed_variantes = data_enum
        .variants
        .iter()
        .map(|variant| VariantInfo::parse(&ast.ident, variant))
        .collect::<Vec<_>>();

    let enum_name = &ast.ident;
    let enum_assoc_type = format!("{}", enum_name)
        .replace("Syntax", "TokenSet")
        .parse::<TokenStream>()
        .unwrap();
    let enum_variants = parsed_variantes
        .iter()
        .map(|variant| variant.gen_ident());
    let enum_rule_table = parsed_variantes
        .iter()
        .map(|variant| variant.gen_ident_with_rule());

    quote! {
        impl<'a> Syntax<'a> for #enum_name {
            type TokenSet = #enum_assoc_type;

            fn into_iter() -> impl Iterator<Item = Self> {
                vec![
                    #( #enum_variants, )*
                ].into_iter()
            }

            fn into_rule(&self) -> Rule<'a, Self::TokenSet> {
                match self {
                    #( #enum_rule_table, )*
                    _ => unimplemented!(),
                }
            }
        }
    }
}

struct VariantInfo<'a> {
    parent_ident: &'a Ident,
    self_ident: &'a Ident,
    rule: Option<TokenStream>,
}

impl<'a> VariantInfo<'a> {
    fn parse(parent_ident: &'a Ident, variant: &'a Variant) -> VariantInfo<'a> {
        let self_ident = &variant.ident;

        let mut rule = None;
        for attr in &variant.attrs {
            let attr = attr.parse_args::<LitStr>().unwrap().value();
            rule = Some(Self::parse_rule(&attr));
        }

        VariantInfo {
            parent_ident,
            self_ident,
            rule,
        }
    }

    fn parse_rule(s: &str) -> TokenStream {
        let mut splitted = s.split("::=");

        let lhs = splitted.next().unwrap().trim();
        let lhs = &lhs[1..lhs.len() - 1];
        let lhs = quote! { RuleElem::new_nonterm(#lhs) };

        let rhs = splitted.collect::<String>()
            .split_whitespace()
            .map(|s| {
                if s.starts_with('<') {
                    let s = &s[1..s.len() - 1];
                    quote! { RuleElem::new_nonterm(#s) }
                } else {
                    let ident = s.parse::<TokenStream>().unwrap();
                    quote! { RuleElem::new_term(Self::TokenSet::#ident) }
                }
            })
            .collect::<Vec<_>>();

        quote! { Rule::from((#lhs, vec![ #( #rhs, )* ])) }
    }

    fn gen_ident(&self) -> TokenStream {
        let parent_ident = self.parent_ident;
        let self_ident = self.self_ident;

        quote! { #parent_ident :: #self_ident }
    }

    fn gen_ident_with_rule(&self) -> TokenStream {
        let ident = self.gen_ident();
        match &self.rule {
            Some(rule) => quote! { #ident => #rule },
            None => quote! { unimplemented!() },
        }
    }
}
