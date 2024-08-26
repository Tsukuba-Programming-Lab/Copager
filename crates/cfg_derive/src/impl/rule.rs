use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Variant, Ident, LitStr};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_enum = if let Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!("\"RuleKind\" proc-macro is only implemented for enum.")
    };

    let parsed_variantes = data_enum
        .variants
        .iter()
        .map(|variant| VariantInfo::parse(&ast.ident, variant))
        .collect::<Vec<_>>();

    let enum_name = &ast.ident;
    let enum_assoc_type = format!("{}", enum_name)
        .replace("Rule", "Token")
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
            type TokenKind = #enum_assoc_type;

            fn into_rules(&self) -> Vec<Rule<'a, Self::TokenKind>> {
                match self {
                    #( #enum_rule_table, )*
                    _ => unimplemented!(),
                }
            }

            fn into_iter() -> impl Iterator<Item = Self> {
                vec![
                    #( #enum_variants, )*
                ].into_iter()
            }
        }
    }
}

struct VariantInfo<'a> {
    parent_ident: &'a Ident,
    self_ident: &'a Ident,
    rules: Vec<TokenStream>,
}

impl<'a> VariantInfo<'a> {
    fn parse(parent_ident: &'a Ident, variant: &'a Variant) -> VariantInfo<'a> {
        let self_ident = &variant.ident;

        let mut rules = vec![];
        for attr in &variant.attrs {
            let attr = attr.parse_args::<LitStr>().unwrap().value();
            rules.push(Self::parse_rule(&attr));
        }

        VariantInfo {
            parent_ident,
            self_ident,
            rules,
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
        if self.rules.is_empty() {
            quote! { #ident => unimplemented!() }
        } else {
            let rules = &self.rules;
            quote! { #ident => vec![#(#rules),*] }
        }
    }
}
