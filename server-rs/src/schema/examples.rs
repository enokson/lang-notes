use serde::{Deserialize, Serialize};
use crate::schema::Int;

pub mod table {
    pub const TABLE_NAME: &'static str = "examples";
    pub const ID: &'static str = "id";
    pub const PARENT_ID: &'static str = "parent_id";
    pub const PARENT_TYPE: &'static str = "parent_type";
    pub const EXAMPLE: &'static str = "example";
    pub const PARENT_TYPE_DEFINITION: &'static i32 = &1;
    pub const PARENT_TYPE_TRANSLATION: &'static i32 = &2;
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

pub fn get_create_table_sql() -> String {
    vec![
        "create", "table", "if not exists", table::TABLE_NAME,
        &format!("({})",
            vec![
                format!("{} serial not null unique",    table::ID),
                format!("{} int not null",              table::PARENT_TYPE),
                format!("{} int not null",              table::PARENT_ID),
                format!("{} varchar(256)",              table::EXAMPLE)
            ].join(",")
        )
    ].join(" ")
}
