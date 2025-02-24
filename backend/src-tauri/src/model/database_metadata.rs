use custom_macro::GenerateTableEnum;
use sea_query::Iden;

#[derive(GenerateTableEnum)]
pub struct DatabaseMetadata {
    pub id: u32,
    pub description: String,
    pub version: u64,
}
