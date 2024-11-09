use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Variant, Ident, LitStr};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_enum = if let Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!("\"ParseResource\" proc-macro is only implemented for enum.")
    };

    let parsed_variantes = data_enum
        .variants
        .iter()
        .map(|variant| VariantInfo::parse(&ast.ident, variant))
        .collect::<Vec<_>>();

    let enum_name = &ast.ident;
    let enum_matcher_table_i2r = parsed_variantes
        .iter()
        .map(|variant| variant.gen_matcher_ident_to_rule());
    let enum_assoc_type = format!("{}", enum_name)
        .replace("Rule", "Token")
        .parse::<TokenStream>()
        .unwrap();
    let enum_variants = parsed_variantes
        .iter()
        .map(|variant| variant.gen_ident());

    quote! {
        impl RuleTag<#enum_assoc_type> for #enum_name {
            fn as_rules(&self) -> Vec<Rule<#enum_assoc_type>> {
                match self {
                    #( #enum_matcher_table_i2r, )*
                }
            }
        }

        impl ParseSource<#enum_assoc_type> for #enum_name {
            type Tag = Self;

            fn iter(&self) -> impl Iterator<Item = Self> {
                vec![ #( #enum_variants, )* ].into_iter()
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
        let token_ident = format!("{}", parent_ident)
            .replace("Rule", "Token")
            .parse::<TokenStream>()
            .unwrap();

        let mut rules = vec![];
        for attr in &variant.attrs {
            if attr.path().is_ident("rule") {
                let attr = attr.parse_args::<LitStr>().unwrap().value();
                rules.push(parse_rule(&token_ident, &attr));
            }
        }

        VariantInfo {
            parent_ident,
            self_ident,
            rules,
        }
    }

    fn gen_ident(&self) -> TokenStream {
        let parent_ident = self.parent_ident;
        let self_ident = self.self_ident;

        quote! { #parent_ident :: #self_ident }
    }

    fn gen_matcher_ident_to_rule(&self) -> TokenStream {
        let ident = self.gen_ident();
        if self.rules.is_empty() {
            quote! { #ident => unimplemented!() }
        } else {
            let rules = &self.rules;
            quote! { #ident => vec![#(#rules),*] }
        }
    }
}

fn parse_rule(token: &TokenStream, input: &str) -> TokenStream {
    let mut splitted = input.split("::=");

    let lhs = splitted.next().unwrap().trim();
    let lhs = &lhs[1..lhs.len() - 1];
    let lhs = quote! { RuleElem::new_nonterm(#lhs) };

    let rhs = splitted.collect::<String>()
        .split_whitespace()
        .map(|elem| {
            if elem.starts_with('<') {
                let elem = &elem[1..elem.len() - 1];
                quote! { RuleElem::new_nonterm(#elem) }
            } else {
                let ident = elem.parse::<TokenStream>().unwrap();
                quote! { RuleElem::new_term(#token::#ident) }
            }
        })
        .collect::<Vec<_>>();
    let rhs = if rhs.len() == 0 {
        vec![quote! { RuleElem::Epsilon }]
    } else {
        rhs
    };

    quote! { Rule::from((#lhs, vec![ #( #rhs, )* ])) }
}
