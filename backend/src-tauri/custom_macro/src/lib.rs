use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Field, Fields, Ident, Type};

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

// TODO: Make Macro to automate create struct without an id
//
// #[proc_macro_derive(GenerateNoIdStruct)]
// pub fn generate_no_id_struct(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_name = input.ident;
//
//     let no_id_struct_name = String::from(struct_name.to_string() + "NoId");
//     let new_struct_name = syn::Ident::new(&no_id_struct_name, struct_name.span());
//
//     let mut field_mappings = Vec::new();
//
//     if let Data::Struct(data_struct) = input.data {
//
//     }
//
// }

// TODO:    Make macro to convert struct into diesel table! macro
//          and make good understanding on how this macro work

#[proc_macro_derive(GenerateDieselTable)]
pub fn diesel_table_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Convert struct name to lowercase to use as the table name
    let table_struct_name = String::from(struct_name.to_string().to_lowercase() + "_table");
    let table_name = syn::Ident::new(&table_struct_name, struct_name.span());

    let mut field_mappings = Vec::new();

    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields) = data_struct.fields {
            for field in fields.named {
                let field_name = field.ident.unwrap();
                let field_type = field.ty;

                // Convert Rust types to Diesel SQL types
                let diesel_type = match field_type {
                    Type::Path(type_path) => {
                        let type_name = type_path.path.segments.last().unwrap().ident.to_string();
                        match type_name.as_str() {
                            "i32" => quote! { Integer },
                            "u32" => quote! { Integer },
                            "i64" => quote! { BigInt },
                            "String" => quote! { Text },
                            "bool" => quote! { Bool },
                            "NaiveDate" => quote! { Date},
                            // "NaiveDateTime" | "DateTime" => quote! { Timestamp },
                            "f32" => quote! { Float },
                            "f64" => quote! { Double },
                            _ => quote! { Text }, // Default to Text if unknown
                        }
                    }
                    _ => quote! { Text }, // Default to Text for unsupported types
                };

                field_mappings.push(quote! {
                    #field_name -> #diesel_type,
                });
            }
        }
    }

    let output = quote! {
        diesel::table! {
            #table_name (id) {
                #(
                    #field_mappings
                )*
            }
        }
    };

    // remove comment to debug
    // println!("{}", output.to_string());
    output.into()
}

// TODO: Complete this test to testing macro
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_diesel_table_macro() {
        let input = quote! {
            struct Product {
                pub id: u32,
                pub user_id: u32,
                pub paid: bool,
                // pub production_date: NaiveDate,
                // pub taken_date: NaiveDate,
                pub price: f64,
                pub amount: u32,
                pub description: String,
            }
        };

        let token_stream = diesel_table_derive(input.into());
        let output = token_stream.to_string();

        println!("{}", output.to_string());
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
