use crate::schema::Int;
use serde::{Deserialize, Serialize};

pub mod table {
    pub const TABLE_NAME: &'static str = "word_groups";
    pub const ID: &'static str = "id";
    pub const NAME: &'static str = "name";
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub id: Int,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RowI {
    pub name: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum NewDefinition {
    New { name: String },
    Existing { id: Int }
}

pub fn get_create_table_sql() -> String {
    vec![
        "create", "table", "if not exists", table::TABLE_NAME,
        &format!("({})",
            vec![
                format!("{} serial not null unique",    table::ID),
                format!("{} varchar(256) not null",     table::NAME)
            ].join(",")
        )
    ].join(" ")
}
