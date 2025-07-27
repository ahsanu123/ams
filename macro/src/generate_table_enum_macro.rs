use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Field, Ident, Type};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let enum_name = Ident::new(&format!("{}Table", struct_name), struct_name.span());

    match input.data {
        Data::Struct(data) => {
            let mut mut_data: Vec<_> = data.fields.into_iter().collect();

            mut_data.push(Field {
                attrs: Vec::new(),
                vis: syn::Visibility::Inherited,
                mutability: syn::FieldMutability::None,
                ident: Some(Ident::new("table", "table".span())),
                colon_token: Default::default(),
                ty: syn::parse_str::<Type>("String").unwrap(),
            });

            let fields = mut_data.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();

                let pascal_name = syn::Ident::new(
                    &field_name.to_string().to_case(Case::Pascal),
                    field_name.span(),
                );

                quote! {#pascal_name}
            });

            let expanded = quote! {
                use sea_query::Iden;
                #[derive(Debug, Clone, Copy, Iden)]
                pub enum #enum_name {
                    #(#fields),*
                }
            };

            TokenStream::from(expanded)
        }
        _ => {
            panic!("GenerateTableEnum only works on structs");
        }
    }
}
