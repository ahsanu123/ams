use proc_macro::TokenStream;

mod generate_crud_macro;
mod generate_diesel_table_macro;
mod generate_no_id_struct_macro;
mod generate_table_enum_macro;

#[proc_macro_derive(GenerateTableEnum)]
pub fn generate_table_enum(input: TokenStream) -> TokenStream {
    generate_table_enum_macro::derive(input)
}

#[proc_macro_derive(GenerateNoIdStruct)]
pub fn generate_no_id_struct(input: TokenStream) -> TokenStream {
    generate_no_id_struct_macro::derive(input)
}

#[proc_macro_derive(GenerateDieselTable)]
pub fn diesel_table_derive(input: TokenStream) -> TokenStream {
    generate_diesel_table_macro::derive(input)
}
