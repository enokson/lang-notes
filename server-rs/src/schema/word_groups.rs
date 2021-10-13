use super::*;

pub mod table {
    pub const TABLE_NAME: &'static str = "word_groups";
    pub const ID: &'static str = "id";
    pub const NAME: &'static str = "name";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub id: u32,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RowI {
    pub name: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NewDefinition {
    New { name: String },
    Existing { id: u32 }
}
