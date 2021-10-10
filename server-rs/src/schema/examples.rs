use super::{ Db, get_id, Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub enum ParentType {
    Definition,
    Translation
}
impl ParentType {
    pub fn to_int(&self) -> u32 {
        match self {
            ParentType::Definition => 1,
            ParentType::Translation => 2
        }
    }
    pub fn from_int(int: &u32) -> Result<ParentType, String> {
        match int {
            1 => Ok(ParentType::Definition),
            2 => Ok(ParentType::Translation),
            _ => Err("Could not parse int into example parent type.".to_string())
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub id: u32,
    pub parent_type: u32,
    pub parent_id: u32,
    pub example: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewRow {
    pub parent_type: ParentType,
    pub parent_id: u32,
    pub example: String
}

pub fn insert_row(db: &mut Db, data: &NewRow) -> Result<u32, String> {
    let sql = include_str!("sql/examples/insert.sql");
    match db.query(sql, &[&data.parent_type.to_int(), &data.parent_id, &data.example]) {
        Ok(rows) => get_id(&rows),
        Err(error) => Err(error.to_string())
    }
}
