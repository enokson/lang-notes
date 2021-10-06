use postgres::{ Client, Row };
use serde::{Deserialize, Serialize};
use std::{ 
    sync::{ Mutex, MutexGuard },
};
type Db<'a> = MutexGuard<'a, Client>;

pub fn get_db<'a>(db: &'a Mutex<Client>) -> Result<Db<'a>, String> {
    match db.try_lock() {
        Ok(db) => Ok(db),
        Err(error) => Err(error.to_string())
    }
}

pub fn get_id(rows: Vec<Row>) -> Result<u32, String> {
    match rows.get(0) {
        Some(row) => match row.try_get(0) {
            Ok(id) => Ok(id),
            Err(error) => Err(error.to_string())
        },
        None => Err("Could not get id. Could not find row.".to_string())
    }
}

pub mod words_group {
    use super::{Db, get_id, Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Row {
        pub id: u32,
        pub name: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct NewRow {
        pub name: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub enum NewDefinition {
        New { name: String },
        Existing { id: u32 }
    }

    pub fn insert_row(db: &mut Db, new_row: &NewRow) -> Result<u32, String> {
        let sql = include_str!("sql/word_groups/insert.sql");
        match db.query(sql, &[&new_row.name]) {
            Ok(rows) => get_id(rows),
            Err(error) => Err(error.to_string())
        }
    }

}

pub mod languages {
    use super::{Db, get_id, Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Row {
        pub id: u32,
        pub name: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct NewRow {
        pub name: String
    }

    pub fn insert_row(db: &mut Db, new_row: &NewRow) -> Result<u32, String> {
        let sql = include_str!("sql/languages/insert.sql");
        match db.query(sql, &[&new_row.name]) {
            Ok(rows) => get_id(rows),
            Err(error) => Err(error.to_string())
        }
    }

}

pub mod translations {

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

    pub fn insert_translation_row(db: &mut Db, data: &NewRow) -> Result<u32, String> {
        let sql = include_str!("sql/translations/insert.sql");
        match db.query(sql, &[ &data.lang_id, &data.definition ]) {
            Ok(rows) => get_id(rows),
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

}

pub mod examples {
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
            Ok(rows) => get_id(rows),
            Err(error) => Err(error.to_string())
        }
    }

}

pub mod definitions {
    use super::{
        Db, get_id,
        Deserialize, Serialize,
        examples,
        translations,
        words_group
    };

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Row {
        pub id: u32,
        pub word_group_id: u32,
        pub cluster_id: u32,
        pub pronounciation: Option<String>,
        pub word: String,
        pub prefixes: Option<String>,
        pub uffixes: Option<String>,
        pub definition: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct NewRow {
        pub words_group_id: words_group::NewDefinition,
        pub cluster_id: u32,
        pub pronounciation: Option<String>,
        pub word: String,
        pub prefixes: Option<String>,
        pub suffixes: Option<String>,
        pub definition: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Definition {
        pub value: Row,
        pub word_group: words_group::Row,
        pub translations: Vec<translations::Translation>,
        pub examples: Vec<examples::Row>
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct NewDefinition {
        pub value: NewRow,
        pub translations: Option<Vec<translations::NewTranslation>>,
        pub examples: Option<Vec<examples::NewRow>>
    }

    pub fn insert_row(db: &mut Db, data: &NewRow) -> Result<u32, String> {
        let sql = include_str!("sql/definitions/insert.sql");
        let word_group_id = {
            match &data.words_group_id {
                words_group::NewDefinition::New { name } => words_group::insert_row(db, &words_group::NewRow { name: name.to_string() }),
                words_group::NewDefinition::Existing { id } => Ok(id.to_owned())
            }
        }?;

        match db.query(
            sql, 
            &[
                &word_group_id,
                &data.cluster_id,
                &data.pronounciation,
                &data.word,
                &data.prefixes,
                &data.suffixes,
                &data.definition
            ]) {
                Ok(rows) => get_id(rows),
                Err(error) => Err(error.to_string())
            }
    }

    pub fn insert_definition(db: &mut Db, data: &NewDefinition) -> Result<u32, String> {
        let def_id = insert_row(db, &data.value)?;
        if let Some(translation_vec) = &data.translations {
            translation_vec
                .iter()
                .map(|new_translation| translations::insert_translation(db, new_translation))
                .collect::<Result<Vec<u32>, String>>()?;
        }
        if let Some(example_vec) = &data.examples {
            example_vec
                .iter()
                .map(|new_example| examples::insert_row(db, new_example))
                .collect::<Result<Vec<u32>, String>>()?;
        }
        Ok(def_id)
    }

}

pub mod clusters {
    use super::{
        Db,
        Deserialize, Serialize,
        definitions::{Definition, NewDefinition}
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

    pub fn insert_cluster_row(db: &mut Db) -> Result<u32, String> {
        let sql = include_str!("./sql/cluster/insert.sql");
        match db.query(sql, &[]) {
            Ok(rows) => match rows.get(0) { // get the first row
                Some(row) => match row.try_get(0) { // get the first value, which should be the id
                    Ok(id) => Ok(id),
                    Err(error) => Err(error.to_string())
                },
                None => Err("Could not return id after insert. No rows returned.".to_string())
            },
            Err(error) => Err(error.to_string())
        }
    }
}
