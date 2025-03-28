use ams_macro::GenerateNoIdStruct;

#[derive(GenerateNoIdStruct)]
pub struct WithId {
    pub id: i32,
    pub other_field: String,
}
