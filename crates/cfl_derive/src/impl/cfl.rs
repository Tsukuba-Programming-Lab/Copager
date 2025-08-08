use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Index, Type};

pub fn proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_struct = if let Data::Struct(data_struct) = ast.data {
        data_struct
    } else {
        panic!("\"CFL\" proc-macro is only implemented for struct.")
    };

    let struct_fields = FieldsWrapper::from(&data_struct.fields);
    let (tokenset_ident, tokenset_ty) = struct_fields.pickup("tokenset");
    let (ruleset_ident, ruleset_ty) = struct_fields.pickup("ruleset");

    let struct_name = &ast.ident;

    quote!{
        impl CFL for #struct_name {
            type TokenTag = #tokenset_ty;
            type Tokens = #tokenset_ty;
            type RuleTag = #ruleset_ty;
            type Rules = #ruleset_ty;

            fn instantiate_tokens(&self) -> Self::Tokens {
                self.#tokenset_ident
            }

            fn instantiate_rules(&self) -> Self::Rules {
                self.#ruleset_ident
            }
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
    fn pickup(&'a self, name: &str) -> (TokenStream, &'a Type) {
        // 指定された名前の attribute を持つフィールドを探す (複数 -> ×)
        let (mut found_at, mut found_field) = (None, None);
        for (idx, field) in self.fields.iter().enumerate() {
            for attr in &field.attrs {
                let attr = attr.path();
                if attr.is_ident(name) {
                    if found_field.is_some() {
                        panic!("Multiple #[{}] attributes are not allowed.", name);
                    }
                    found_at = Some(idx);
                    found_field = Some(field);
                }
            }
        }

        // フィールド名，型を取得
        let (ident, ty) = match &found_field {
            // 名前付きフィールド
            Some(Field { ident: Some(ident), .. }) => {
                (quote! { #ident }, &found_field.unwrap().ty)
            }
            // 無名フィールド (タプル構造体)
            Some(Field { ident: None, .. }) => {
                let idx = Index::from(found_at.unwrap());
                (quote! { #idx }, &found_field.unwrap().ty)
            }
            None => panic!("Field with #[{}] attribute must have an identifier.", name),
        };

        (ident, ty)
    }
}
