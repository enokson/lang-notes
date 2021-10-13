use crate::{
    schema::{
        Int,
        definitions::{
            Row as DefinitionRow,
            RowI as DefinitionRowI
        }
    }
};
use serde::{Deserialize, Serialize};

pub mod table {
    pub const TABLE_NAME: &'static str = "clusters";
    pub const ID: &'static str = "id";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    id: Int
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cluster {
    cluster: Row,
    definition: Vec<DefinitionRow>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterI {
    definitions: Vec<DefinitionRowI>
}
