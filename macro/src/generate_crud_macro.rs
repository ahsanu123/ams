use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // let post = posts
    //     .find(post_id)
    //     .select(Post::as_select())
    //     .first(connection)
    //     .optional();

    let output = quote! {
        diesel::table! {
            // #table_name (id) {
            //     #(
            //         #field_mappings
            //     )*
            // }
        }
    };

    output.into()
}
