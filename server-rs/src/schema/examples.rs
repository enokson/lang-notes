use serde::{Deserialize, Serialize};

pub mod table {
    pub const TABLE_NAME: &'static str = "examples";
    pub const ID: &'static str = "id";
    pub const PARENT_ID: &'static str = "parent_id";
    pub const PARENT_TYPE: &'static str = "parent_type";
    pub const EXAMPLE: &'static str = "example";
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub id: i32,
    pub parent_type: i32,
    pub parent_id: i32,
    pub example: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RowI {
    pub parent_type: i32,
    pub parent_id: i32,
    pub example: String
}
