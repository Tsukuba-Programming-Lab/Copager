use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_struct = if let Data::Struct(data_struct) = ast.data {
        data_struct
    } else {
        panic!("\"CFL\" proc-macro is only implemented for struct.")
    };

    let struct_fields = FieldsWrapper::from(&data_struct.fields);
    let tokenset_ty = struct_fields.pickup("tokenset");
    let ruleset_ty = struct_fields.pickup("ruleset");

    let struct_name = &ast.ident;

    quote!{
        impl CFL for #struct_name {
            type TokenTag = #tokenset_ty;
            type TokenSet = #tokenset_ty;
            type RuleTag = #ruleset_ty;
            type RuleSet = #ruleset_ty;
        }
    }
}

struct FieldsWrapper<'a> {
    fields: &'a Fields,
}

impl<'a> From<&'a Fields> for FieldsWrapper<'a> {
    fn from(fields: &'a Fields) -> Self {
        FieldsWrapper { fields }
    }
}

impl<'a> FieldsWrapper<'a> {
    fn pickup(&'a self, name: &str) -> &'a Type {
        // 指定された名前の attribute を持つフィールドを探す (複数 -> ×)
        let mut found_field = None;
        for field in self.fields {
            for attr in &field.attrs {
                let attr = attr.path();
                if attr.is_ident(name) {
                    if found_field.is_some() {
                        panic!("Multiple #[{}] attributes are not allowed.", name);
                    }
                    found_field = Some(field);
                }
            }
        }

        &found_field.unwrap().ty
    }
}
