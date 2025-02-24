use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use sea_query::Iden;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Field, Ident, Type};

#[proc_macro_derive(GenerateTableEnum)]
pub fn generate_table_enum(input: TokenStream) -> TokenStream {
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

// TODO: Complete this macro to generate sea-query table creation
//
// use proc_macro::TokenStream;
// use quote::{quote, format_ident};
// use syn::{parse_macro_input, DeriveInput, Data, Fields};
//
// #[proc_macro_derive(SeaQueryTable)]
// pub fn seaquery_table_macro(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_name = input.ident;
//
//     let mut columns = vec![];
//
//     if let Data::Struct(data_struct) = input.data {
//         if let Fields::Named(fields) = data_struct.fields {
//             for field in fields.named {
//                 let field_name = field.ident.unwrap().to_string();
//                 let field_type = field.ty;
//
//                 // Convert Rust type to SeaQuery column definition
//                 let col_def = quote! {
//                     .col(sea_query::ColumnDef::new(#field_name)
//                         #{
//                             if stringify!(#field_type) == "i32" {
//                                 quote!(.integer())
//                             } else if stringify!(#field_type) == "String" {
//                                 quote!(.string().not_null())
//                             } else if stringify!(#field_type) == "bool" {
//                                 quote!(.boolean().not_null())
//                             } else {
//                                 quote!()
//                             }
//                         }
//                     )
//                 };
//
//                 columns.push(col_def);
//             }
//         }
//     }
//
//     let expanded = quote! {
//         impl #struct_name {
//             pub fn create_table() -> sea_query::TableCreateStatement {
//                 use sea_query::{Table, ColumnDef, SqliteQueryBuilder};
//
//                 Table::create()
//                     .table(stringify!(#struct_name))
//                     #(#columns)*
//                     .build(SqliteQueryBuilder)
//                     .unwrap()
//             }
//         }
//     };
//
//     TokenStream::from(expanded)
// }
