use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Variant, Ident, LitStr};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_enum = if let Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!("\"CFLRule\" proc-macro is only implemented for enum.")
    };

    // 字句集合の型を取得
    let tokenset_ty = data_enum
        .variants
        .iter()
        .filter_map(|v| v.attrs.iter().find(|a| a.path().is_ident("tokenset")))
        .next()
        .map(|a| a.parse_args::<Ident>().unwrap());
    let tokenset_ty = match tokenset_ty {
        Some(ty) => ty,
        None => panic!("\"CFLRule\" proc-macro requires a \"tokenset\" attribute on the enum."),
    };

    // 各列挙子と紐づく文法規則を解析
    let parsed_variantes = data_enum
        .variants
        .iter()
        .map(|variant| VariantInfo::parse(&tokenset_ty, variant))
        .collect::<Vec<_>>();

    // 列挙型に関する情報を用意
    let enum_name = &ast.ident;
    let enum_rule_matchers = parsed_variantes
        .iter()
        .map(|variant| variant.gen_matcher_ident_to_rule());
    let enum_variants = parsed_variantes
        .iter()
        .map(|variant| variant.gen_ident());

    quote! {
        impl RuleTag<#tokenset_ty> for #enum_name {
            fn as_rules(&self) -> Vec<Rule<#tokenset_ty, Self>> {
                match self {
                    #( #enum_rule_matchers, )*
                }
            }
        }

        impl CFLRule<#tokenset_ty> for #enum_name {
            type Tag = Self;

            fn iter(&self) -> impl Iterator<Item = Self> {
                vec![ #( #enum_variants, )* ].into_iter()
            }
        }
    }
}

struct VariantInfo<'a> {
    ident: &'a Ident,
    rule_lhs_rhs_tuples: Vec<TokenStream>,
}

impl<'a> VariantInfo<'a> {
    fn parse(tokenset_ty: &Ident, variant: &'a Variant) -> VariantInfo<'a> {
        // 列挙子名
        let ident = &variant.ident;

        // 文法規則を収集
        let mut rule_lhs_rhs_tuples = vec![];
        for attr in &variant.attrs {
            if attr.path().is_ident("rule") {
                let attr = attr.parse_args::<LitStr>().unwrap().value();
                rule_lhs_rhs_tuples.push(parse_rule(tokenset_ty, &attr));
            }
        }

        VariantInfo { ident, rule_lhs_rhs_tuples }
    }

    fn gen_ident(&self) -> TokenStream {
        let ident = self.ident;
        quote! { Self :: #ident }
    }

    fn gen_matcher_ident_to_rule(&self) -> TokenStream {
        let ident = self.gen_ident();
        if self.rule_lhs_rhs_tuples.is_empty() {
            quote! { #ident => unimplemented!() }
        } else {
            let lhs_rhs_tuple = &self.rule_lhs_rhs_tuples;
            quote! { #ident => vec![#(Rule::new(Some(#ident), #lhs_rhs_tuple)),*] }
        }
    }
}

fn parse_rule(tokenset_ty: &Ident, input: &str) -> TokenStream {
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
                quote! { RuleElem::new_term(#tokenset_ty :: #ident) }
            }
        })
        .collect::<Vec<_>>();
    let rhs = if rhs.len() == 0 {
        vec![quote! { RuleElem::Epsilon }]
    } else {
        rhs
    };

    quote! { #lhs, vec![ #( #rhs, )* ], }
}
