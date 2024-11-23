use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let vis = &ast.vis;
    let ident = &ast.ident;
    let ident_builder = Ident::new(&format!("{}Builder", ident), ident.span());

    quote! {
        #vis struct #ident_builder<'input, Ts, Rs>
        where
            Ts: CFLTokens + 'input,
            Rs: CFLRules<Ts::Tag>,
        {
            stack: Vec<RawIR<'input, Ts, Rs>>,
        }

        impl <'input, Ts, Rs> IRBuilder<'input, Ts, Rs> for #ident_builder<'input, Ts, Rs>
        where
            Ts: CFLTokens + 'input,
            Rs: CFLRules<Ts::Tag>,
        {
            type Output = #ident<'input, Ts, Rs>;

            fn new() -> #ident_builder<'input, Ts, Rs> {
                #ident_builder {
                    stack: Vec::new(),
                }
            }

            fn on_read(&mut self, token: Token<'input, Ts::Tag>) -> anyhow::Result<()> {
                self.stack.push(RawIR::Atom(token));
                Ok(())
            }

            fn on_parse(&mut self, rule: Rs::Tag, len: usize) -> anyhow::Result<()> {
                let elems = self.stack.split_off(self.stack.len() - len);
                self.stack.push(RawIR::List { rule, elems });
                Ok(())
            }

            fn build(mut self) -> anyhow::Result<Self::Output>
            where
                Self::Output: From<RawIR<'input, Ts, Rs>>,
            {
                assert!(self.stack.len() == 1);
                Ok(Self::Output::from(self.stack.pop().unwrap()))
            }
        };
    }
}
