use super::{
    Int,
    Deserialize, Serialize,
    examples
};

pub mod table {
    pub const TABLE_NAME: &'static str = "translations";
    pub const ID: &'static str = "id";
    pub const LANG_ID: &'static str = "lang_id";
    pub const DEFINITION: &'static str = "definition";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub id: Int,
    pub lang_id: Int,
    pub definition: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RowI {
    pub lang_id: Int,
    pub definition: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Translation {
    pub value: Row,
    pub examples: Vec<examples::Row>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TranslationI {
    pub value: RowI,
    pub examples: Vec<examples::RowI>
}