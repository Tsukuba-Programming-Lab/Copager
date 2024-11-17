use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Index, Type};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_struct = if let Data::Struct(data_struct) = ast.data {
        data_struct
    } else {
        panic!("\"CFLTokens\" proc-macro is only implemented for struct.")
    };

    let struct_name = &ast.ident;
    let (tokens_field, rules_field) = parse_fields(&data_struct.fields);
    let (tokens_ident, tokens_type) = (&tokens_field.ident, tokens_field.ty);
    let (rules_ident, rules_type) = (&rules_field.ident, rules_field.ty);

    quote!{
        impl CFL for #struct_name {
            type TokenTag = #tokens_type;
            type Tokens = #tokens_type;
            type RuleTag = #rules_type;
            type Rules = #rules_type;

            fn instantiate_tokens(&self) -> Self::Tokens {
                self.#tokens_ident
            }

            fn instantiate_rules(&self) -> Self::Rules {
                self.#rules_ident
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
    let mut tokens_field = None;
    let mut rules_field = None;
    for (idx, field) in fields.iter().enumerate() {
        for attr in &field.attrs {
            let attr = attr.path();
            if attr.is_ident("tokens") {
                if tokens_field.is_some() {
                    panic!("Multiple #[tokens] attributes are not allowed.");
                }
                tokens_field = Some(FieldInfo::from((idx, field)));
            } else if attr.is_ident("rules") {
                if rules_field.is_some() {
                    panic!("Multiple #[rules] attributes are not allowed.");
                }
                rules_field = Some(FieldInfo::from((idx, field)));
            }
        }
    }
    if tokens_field.is_none() {
        panic!("No #[tokens] attribute found.");
    }
    if rules_field.is_none() {
        panic!("No #[rules] attribute found.");
    }

    (tokens_field.unwrap(), rules_field.unwrap())
}
