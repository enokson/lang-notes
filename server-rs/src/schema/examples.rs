use serde::{Deserialize, Serialize};
use crate::schema::Int;

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
    pub id: Int,
    pub parent_type: Int,
    pub parent_id: Int,
    pub example: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RowI {
    pub parent_type: Int,
    pub parent_id: Int,
    pub example: String
}
