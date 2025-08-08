use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

use crate::utils::to_generics_without_where;

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let vis = &ast.vis;
    let ident = &ast.ident;
    let ident_builder = Ident::new(&format!("{}Builder", ident), ident.span());
    let generics = to_generics_without_where(&ast.generics);

    quote! {
        #vis struct #ident_builder<'input, L: Lang> {
            stack: Vec<RawIR<'input, L>>,
        }

        impl <'input, L: Lang> IRBuilder<'input, L> for #ident_builder<'input, L> {
            type Output = #ident #generics;

            fn new() -> #ident_builder<'input, L> {
                #ident_builder {
                    stack: Vec::new(),
                }
            }

            fn on_read(&mut self, token: Token<'input, L::TokenTag>) -> anyhow::Result<()> {
                self.stack.push(RawIR::Atom(token));
                Ok(())
            }

            fn on_parse(&mut self, rule: L::RuleTag, len: usize) -> anyhow::Result<()> {
                let elems = self.stack.split_off(self.stack.len() - len);
                let elems = elems
                    .into_iter()
                    .filter(|elem| match elem {
                        RawIR::Atom(token) => !token.kind.as_option_list().contains(&"ir_omit"),
                        _ => true,
                    })
                    .collect();
                self.stack.push(RawIR::List { rule, elems });
                Ok(())
            }

            fn build(mut self) -> anyhow::Result<Self::Output>
            where
                Self::Output: From<RawIR<'input, L>>,
            {
                assert!(self.stack.len() == 1);
                Ok(Self::Output::from(self.stack.pop().unwrap()))
            }
        };
    }
}
