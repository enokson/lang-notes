use crate::schema::Int;
use serde::{Deserialize, Serialize};

pub mod table {
    pub const TABLE_NAME: &'static str = "clusters";
    pub const ID: &'static str = "id";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub id: Int
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cluster {
    pub cluster: Row,
    pub definitions: Vec<Int>
}

pub fn get_create_table_sql() -> String {
    vec![
        "create", "table", "if not exists", table::TABLE_NAME,
        &format!("({})",
            vec![
                format!("{} serial not null unique", table::ID)
            ].join(",")
        )
    ].join(" ")
}
