use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let vis = &ast.vis;
    let ident = &ast.ident;
    let ident_builder = Ident::new(&format!("{}Builder", ident), ident.span());

    quote! {
        #vis struct #ident_builder<'input, Lang: CFL> {
            stack: Vec<RawIR<'input, Lang>>,
        }

        impl <'input, Lang: CFL> IRBuilder<'input, Lang> for #ident_builder<'input, Lang> {
            type Output = #ident<'input, Lang>;

            fn new() -> #ident_builder<'input, Lang> {
                #ident_builder {
                    stack: Vec::new(),
                }
            }

            fn on_read(&mut self, token: Token<'input, Lang::TokenTag>) -> anyhow::Result<()> {
                self.stack.push(RawIR::Atom(token));
                Ok(())
            }

            fn on_parse(&mut self, rule: Lang::RuleTag, len: usize) -> anyhow::Result<()> {
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
                Self::Output: From<RawIR<'input, Lang>>,
            {
                assert!(self.stack.len() == 1);
                Ok(Self::Output::from(self.stack.pop().unwrap()))
            }
        };
    }
}
