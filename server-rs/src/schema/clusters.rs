use super::*;

use definitions::{
    Definition,
    NewDefinition
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    id: u32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cluster {
    cluster: Row,
    definition: Vec<Definition>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewCluster {
    definitions: Option<Vec<NewDefinition>>
}

#[derive(Debug, Clone, Copy)]
pub enum K {
    Id
}
impl Columnist<V> for K {
    fn get_key(&self) -> String {
        match &self {
            Self::Id => format!("id")
        }
    }
    fn get_value_from_row(&self, row: &postgres::Row) -> Result<V, String> {
        match &self {
            Self::Id => Ok(V::Id(get_value_from_row(row, &self.get_key())?))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum V {
    Id(u32)
}

impl Valuable for V {
    fn get_value(&self) -> String {
        match &self {
            Self::Id(id) => format!("{}", id)
        }
    }
}

pub struct Filter {
    pub search_translations: bool, // will search for word prop in the translation table
    pub language_ids: Option<Vec<u32>>,
    pub word: String
}

pub fn get_ids(db: &mut Db, filter: &Filter) -> Result<Vec<u32>, String> {
    let mut def_ids: Vec<u32> = vec![];
    let get_definition_ids_by_word = include_str!("sql/clusters/search/get_def_ids_by_word.sql");
    match db.query(get_definition_ids_by_word, &[&filter.word]) {
        Ok(rows) => {
            def_ids.append(
                &mut rows.iter().map(|row| -> Result<u32, String> {
                    match row.try_get("id") {
                        Ok(id) => Ok(id),
                        Err(error) => Err(error.to_string())
                    }
                }).collect::<Result<Vec<u32>, String>>()?
            );
            Ok(())
        },
        Err(error) => Err(error.to_string())
    }?;
    if filter.search_translations {
        let mut get_def_ids_by_translation_def = include_str!("sql/clusters/search/get_def_ids_by_translation.sql").to_string();
        if let Some(languages) = &filter.language_ids {
            get_def_ids_by_translation_def
                .push_str(&format!(" and in ({})", languages.iter().map(|id| format!("{}", id)).collect::<Vec<String>>().join(",")));
            languages.iter().for_each(|id| {
                get_def_ids_by_translation_def.push_str(&format!(" and id = {}", id));
            });
            match db.query(get_def_ids_by_translation_def.as_str(), &[&filter.word]) {
                Ok(rows) => {
                    def_ids.append(
                        &mut rows.iter().map(|row| -> Result<u32, String> {
                            match row.try_get("id") {
                                Ok(id) => Ok(id),
                                Err(error) => Err(error.to_string())
                            }
                        }).collect::<Result<Vec<u32>, String>>()?
                    );
                    Ok(())
                },
                Err(error) => Err(error.to_string())
            }?;
        }
    }
    let mut spotted_ids: Vec<u32> = vec![];
    let mut distinct_def_ids: Vec<u32> = vec![];
    for id in def_ids.iter() {
        let mut unique = true;
        for spotted_id in spotted_ids.iter() {
            if id == spotted_id {
                unique = false;
                break;
            }
        }
        if unique {
            distinct_def_ids.push(id.to_owned());
            spotted_ids.push(id.to_owned());
        }
    }
    let sql = include_str!("sql/clusters/search/get_ids_by_def_ids.sql")
        .replace("{ids}", &format!("{}", 
            distinct_def_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",")));
    match db.query(sql.as_str(), &[]) {
        Ok(rows) => {
            Ok(
                rows.iter().map(|row| -> Result<u32, String> {
                    match row.try_get("id") {
                        Ok(id) => Ok(id),
                        Err(error) => Err(error.to_string())
                    }
                }).collect::<Result<Vec<u32>, String>>()?
            )
        },
        Err(error) => Err(error.to_string())
    }
}
