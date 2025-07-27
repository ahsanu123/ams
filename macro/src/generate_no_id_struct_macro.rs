use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let no_id_struct_name = Ident::new(&format!("{}NoId", struct_name), struct_name.span());

    match input.data {
        Data::Struct(data_struct) => {
            let fields: Vec<_> = data_struct.fields.into_iter().collect();

            let no_id_fields: Vec<_> = fields
                .iter()
                .filter(|field| field.ident.as_ref().unwrap().to_string().to_lowercase() != "id")
                .collect();

            println!("{:#?}", no_id_fields);

            let quoted_fields = no_id_fields.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;

                let pascal_name = syn::Ident::new(
                    &field_name.to_string().to_case(Case::Snake),
                    field_name.span(),
                );
                quote! {
                    pub #pascal_name: #field_type
                }
            });

            let no_id_struct = quote! {
                #[derive(Debug, Clone)]
                pub struct #no_id_struct_name {
                    #(#quoted_fields),*
                }
            };

            TokenStream::from(no_id_struct)
        }
        _ => {
            panic!("GenerateNoIdStruct Only with struct")
        }
    }
}
