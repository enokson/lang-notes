use serde::{Deserialize, Serialize};
use crate::schema::Int;

pub mod table {
    pub const TABLE_NAME: &'static str = "translations";
    pub const ID: &'static str = "id";
    pub const LANG_ID: &'static str = "lang_id";
    pub const DEFINITION_ID: &'static str = "definition_id";
    pub const DEFINITION: &'static str = "definition";
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub id: Int,
    pub lang_id: Int,
    pub definition_id: Int,
    pub definition: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RowI {
    pub lang_id: Int,
    pub definition_id: Int,
    pub definition: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Translation {
    pub value: Row,
    pub examples: Vec<Int>
}

pub fn get_create_table_sql() -> String {
    vec![
        "create", "table", "if not exists", table::TABLE_NAME,
        &format!("({})",
            vec![
                format!("{} serial not null unique",    table::ID),
                format!("{} int not null",              table::LANG_ID),
                format!("{} int not null",              table::DEFINITION_ID),
                format!("{} varchar(256) not null",     table::DEFINITION)
            ].join(",")
        )
    ].join(" ")
}
