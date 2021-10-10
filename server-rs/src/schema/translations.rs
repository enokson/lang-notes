use super::{
    Db, get_id,
    Deserialize, Serialize,
    examples
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub id: u32,
    pub lang_id: u32,
    pub definition: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewRow {
    pub lang_id: u32,
    pub definition: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Translation {
    pub value: Row,
    pub examples: Vec<examples::Row>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTranslation {
    pub value: NewRow,
    pub examples: Vec<examples::NewRow>
}

// TODO: write Column Enum

pub fn insert_translation_row(db: &mut Db, data: &NewRow) -> Result<u32, String> {
    let sql = include_str!("sql/translations/insert.sql");
    match db.query(sql, &[ &data.lang_id, &data.definition ]) {
        Ok(rows) => get_id(&rows),
        Err(error) => Err(error.to_string())
    }
}

pub fn insert_translation(db: &mut Db, data: &NewTranslation) -> Result<u32, String> {
    let id = insert_translation_row(db, &data.value)?;
    data
        .examples
        .iter()
        .map(|new_example| examples::insert_row(db, new_example))
        .collect::<Result<Vec<u32>, String>>()?;
    Ok(id)
}
