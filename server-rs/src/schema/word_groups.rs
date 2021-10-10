

use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub id: u32,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewRow {
    pub name: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NewDefinition {
    New { name: String },
    Existing { id: u32 }
}

#[derive(Debug, Clone, Copy)]
pub enum K {
    Id,
    Name
}
impl Columnist<V> for K {
    fn get_key(&self) -> String {
        match &self {
            Self::Id => format!("id"),
            Self::Name => format!("name")
        }
    }
    fn get_value_from_row(&self, row: &postgres::Row) -> Result<V, String> {
        match &self {
            Self::Id => Ok(V::Id(get_value_from_row(row, &self.get_key())?)),
            Self::Name => Ok(V::Name(get_value_from_row(row, &self.get_key())?))
        }
        
    }
}

#[derive(Debug, Clone)]
pub enum V {
    Id(u32),
    Name(String)
}
impl Valuable for V {
    fn get_value(&self) -> String {
        match &self {
            Self::Id(id) => id.to_string(),
            Self::Name(name) => escape(name)
        }
    }
}
