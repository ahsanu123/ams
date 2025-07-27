use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Convert struct name to lowercase to use as the table name
    let table_struct_name = struct_name.to_string().to_lowercase() + "_table";
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
