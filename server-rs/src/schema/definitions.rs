use serde::{Deserialize, Serialize};
use crate::schema::{
    Int,
    word_groups::{
        NewDefinition
    }
};

pub mod table {
    pub const TABLE_NAME: &'static str = "definitions";
    pub const ID: &'static str = "id";
    pub const WORDS_GROUP_ID: &'static str = "words_group_id";
    pub const CLUSTER_ID: &'static str = "cluster_id";
    pub const PRONOUNCIATION: &'static str = "pronounciation";
    pub const WORD: &'static str = "word";
    pub const PREFIXES: &'static str = "prefixes";
    pub const SUFFIXES: &'static str = "suffixes";
    pub const DEFINITION: &'static str = "definition";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub id: Int,
    pub words_group_id: Int,
    pub cluster_id: Int,
    pub pronounciation: Option<String>,
    pub word: String,
    pub prefixes: Option<String>,
    pub suffixes: Option<String>,
    pub definition: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RowI {
    pub words_group: NewDefinition,
    pub cluster_id: Int,
    pub pronounciation: Option<String>,
    pub word: String,
    pub prefixes: Option<String>,
    pub suffixes: Option<String>,
    pub definition: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Definition {
    pub value: Row,
    pub translations: Vec<Int>,
    pub examples: Vec<Int>
}

pub fn get_create_table_sql() -> String {
    vec![
        "create", "table", "if not exists", table::TABLE_NAME,
        &format!("({})",
            vec![
                format!("{} serial not null unique",    table::ID),
                format!("{} int not null",              table::WORDS_GROUP_ID),
                format!("{} int not null",              table::CLUSTER_ID),
                format!("{} varchar(256)",              table::PRONOUNCIATION),
                format!("{} varchar(256) not null",     table::WORD),
                format!("{} varchar(256)",              table::PREFIXES),
                format!("{} varchar(256)",              table::SUFFIXES),
                format!("{} varchar(256) not null",     table::DEFINITION)
            ].join(",")
        )
    ].join(" ")
}
