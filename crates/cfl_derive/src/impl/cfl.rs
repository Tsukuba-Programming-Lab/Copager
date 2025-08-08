use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Index, Type};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_struct = if let Data::Struct(data_struct) = ast.data {
        data_struct
    } else {
        panic!("\"CFL\" proc-macro is only implemented for struct.")
    };

    let struct_name = &ast.ident;
    let (tokenset_field, ruleset_field) = parse_fields(&data_struct.fields);
    let (tokenset_ident, tokenset_type) = (&tokenset_field.ident, tokenset_field.ty);
    let (ruleset_ident, ruleset_type) = (&ruleset_field.ident, ruleset_field.ty);

    quote!{
        impl CFL for #struct_name {
            type TokenTag = #tokenset_type;
            type Tokens = #tokenset_type;
            type RuleTag = #ruleset_type;
            type Rules = #ruleset_type;

            fn instantiate_tokens(&self) -> Self::Tokens {
                self.#tokenset_ident
            }

            fn instantiate_rules(&self) -> Self::Rules {
                self.#ruleset_ident
            }
        }
    }
}

struct FieldInfo<'a> {
    ident: TokenStream,
    ty: &'a Type,
}

impl<'a> From<(usize, &'a Field)> for  FieldInfo<'a> {
    fn from((idx, field): (usize, &'a Field)) -> Self {
        let ident = match &field.ident {
            Some(ident) => quote!{ #ident },
            None => {
                let idx = Index::from(idx);
                quote! { #idx }
            }
        };
        FieldInfo {
            ident,
            ty: &field.ty,
        }
    }
}

fn parse_fields(fields: &Fields) -> (FieldInfo, FieldInfo) {
    let mut tokenset_field = None;
    let mut ruleset_field = None;
    for (idx, field) in fields.iter().enumerate() {
        for attr in &field.attrs {
            let attr = attr.path();
            if attr.is_ident("tokenset") {
                if tokenset_field.is_some() {
                    panic!("Multiple #[tokenset] attributes are not allowed.");
                }
                tokenset_field = Some(FieldInfo::from((idx, field)));
            } else if attr.is_ident("ruleset") {
                if ruleset_field.is_some() {
                    panic!("Multiple #[ruleset] attributes are not allowed.");
                }
                ruleset_field = Some(FieldInfo::from((idx, field)));
            }
        }
    }
    if tokenset_field.is_none() {
        panic!("No #[tokenset] attribute found.");
    }
    if ruleset_field.is_none() {
        panic!("No #[ruleset] attribute found.");
    }

    (tokenset_field.unwrap(), ruleset_field.unwrap())
}
