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
                let attr = attr.parse_args::<LitStr>().unwrap();
                let bnf = match BNF::parse(tokenset_ty, &attr.value().as_str()) {
                    Ok(bnf) => bnf,
                    Err(e) => syn::Error::new(attr.span(), e).to_compile_error(),
                };
                rule_lhs_rhs_tuples.push(bnf);
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

struct BNF<'a> {
    tokenset_ty: &'a Ident,
    src: &'a str,
    cursor: usize,
    row: usize,
    col: usize,
}

impl<'a> BNF<'a> {
    // <bnf> ::= <rule>
    fn parse(tokenset_ty: &'a Ident, src: &'a str) -> Result<TokenStream, String> {
        BNF { tokenset_ty, src, cursor: 0, row: 1, col: 1 }.parse_rule()
    }

    // <rule> ::= <nonterm> '::=' <rhs>
    fn parse_rule(&mut self) -> Result<TokenStream, String> {
        let lhs = self.parse_nonterm()?;
        self.consume("::=")?;
        let rhs = self.parse_rhs()?;
        Ok(quote! { #lhs, vec![ #( #rhs, )* ], })
    }

    // <rhs> ::= ((<nonterm> | <ident>)*)?
    fn parse_rhs(&mut self) -> Result<Vec<TokenStream>, String> {
        let mut rhs = vec![];
        loop {
            self.skip_spaces();
            if self.src[self.cursor..].is_empty() {
                break;
            }
            if self.src[self.cursor..].starts_with('<') {
                rhs.push(self.parse_nonterm()?);
            } else {
                let tokenset_ty = self.tokenset_ty;
                let ident = self.parse_ident()?.parse::<TokenStream>().unwrap();
                rhs.push(quote! { RuleElem::new_term(#tokenset_ty :: #ident) });
            }
        }
        if rhs.is_empty() {
            rhs.push(quote! { RuleElem::Epsilon });
        }
        Ok(rhs)
    }

    // <nonterm> ::= '<' <nonterm> '>'
    fn parse_nonterm(&mut self) -> Result<TokenStream, String> {
        self.consume("<")?;
        let ident = self.parse_ident()?;
        let lhs = quote! { RuleElem::new_nonterm(#ident) };
        self.consume(">")?;
        Ok(lhs)
    }

    // <ident> ::= [a-zA-Z_][a-zA-Z0-9_]*
    fn parse_ident(&mut self) -> Result<&str, String> {
        self.skip_spaces();

        let end_idx = self.src[self.cursor..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(self.src[self.cursor..].len());
        if end_idx == 0 {
            self.error("Expected an identifier")?;
        }

        let ident = &self.src[self.cursor..self.cursor+end_idx];
        self.cursor += end_idx;
        self.col += end_idx;

        Ok(ident)
    }

    fn consume(&mut self, expected: &str) -> Result<(), String> {
        self.skip_spaces();
        if self.src[self.cursor..].starts_with(expected) {
            self.cursor += expected.len();
            self.col += expected.len();
            Ok(())
        } else {
            self.error(&format!("Expected '{}'", expected))
        }
    }

    fn skip_spaces(&mut self) {
        while let Some(c) = self.src[self.cursor..].chars().next() {
            if c.is_whitespace() {
                self.cursor += c.len_utf8();
                if c == '\n' {
                    self.row += 1;
                    self.col = 1;
                } else {
                    self.col += c.len_utf8();
                }
            } else {
                break;
            }
        }
    }

    fn error(&self, msg: &str) -> Result<(), String> {
        Err(format!(
            "Error: {}\n{}\n{}^ here\n",
            msg,
            self.src,
            " ".repeat(self.col-1),
        ))
    }
}
