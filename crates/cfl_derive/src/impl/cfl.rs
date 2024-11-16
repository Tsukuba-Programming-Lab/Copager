use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_struct = if let Data::Struct(data_struct) = ast.data {
        data_struct
    } else {
        panic!("\"CFLTokens\" proc-macro is only implemented for struct.")
    };

    let struct_name = &ast.ident;
    let (tokens_type, rules_type) = parse_fields(&data_struct.fields);

    quote!{
        impl CFL for #struct_name {
            type Tokens = #tokens_type;
            type Rules = #rules_type;
        }
    }
}

fn parse_fields(fields: &Fields) -> (&Type, &Type) {
    let mut tokens_type = None;
    let mut rules_type = None;
    for field in fields {
        for attr in &field.attrs {
            let attr = attr.path();
            if attr.is_ident("tokens") {
                if tokens_type.is_some() {
                    panic!("Multiple #[tokens] attributes are not allowed.");
                }
                tokens_type = Some(&field.ty);
            } else if attr.is_ident("rules") {
                if rules_type.is_some() {
                    panic!("Multiple #[rules] attributes are not allowed.");
                }
                rules_type = Some(&field.ty);
            }
        }
    }
    if tokens_type.is_none() {
        panic!("No #[tokens] attribute found.");
    }
    if rules_type.is_none() {
        panic!("No #[rules] attribute found.");
    }

    (tokens_type.unwrap(), rules_type.unwrap())
}
