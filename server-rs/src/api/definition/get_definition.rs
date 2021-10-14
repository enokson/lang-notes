use actix_web::web::{Data, HttpResponse, Query};
use serde::{Deserialize, Serialize};
use crate::{
    AppData, 
    error_msg, 
    schema::{
        Db,
        Int, 
        ParamIndexer, 
        definitions::{
            Definition,
            Row,
            table
        }, 
        examples::{
            table as examples_table
        }, 
        translations::{
            table as translations_table
        }
    }
};
use postgres::Row as PostgresRow;

#[derive(Debug, Deserialize)]
pub struct Info {
    id: Int
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Reply {
    Ok {        
        data: Option<Definition>
    },
    Error {
        error: String
    }
}

pub fn get_definition_from_row(row: &PostgresRow) -> Result<Row, String> {
    Ok(Row {
        id: error_msg!(row.try_get(table::ID))?,
        words_group_id: error_msg!(row.try_get(table::WORDS_GROUP_ID))?,
        cluster_id: error_msg!(row.try_get(table::CLUSTER_ID))?,
        pronounciation: error_msg!(row.try_get(table::PRONOUNCIATION))?,
        word: error_msg!(row.try_get(table::WORD))?,
        prefixes: error_msg!(row.try_get(table::PREFIXES))?,
        suffixes: error_msg!(row.try_get(table::SUFFIXES))?,
        definition: error_msg!(row.try_get(table::DEFINITION))?
    })
}

pub fn get_defition(db: &mut Db, id: &Int) -> Result<Option<Definition>, String> {
    let mut indexer = ParamIndexer::new();
    let sql = vec![
        "select", "*", "from", table::TABLE_NAME,
        "where", table::ID, "=", &indexer.next(),
        "limit", "1"
    ].join(" ");
    let example: Option<Definition> = match error_msg!(db.query(sql.as_str(), &[id])) {
        Ok(rows) => match rows.get(0) {
            Some(row) => {
                let definition = error_msg!(get_definition_from_row(&row))?;
                let mut indexer = ParamIndexer::new();
                let examples_query = vec![
                    "select", examples_table::ID,
                    "from", examples_table::TABLE_NAME,
                    "where", table::ID, "=", &indexer.next()
                ].join(" ");
                let examples = match error_msg!(db.query(examples_query.as_str(), &[ &definition.id ])) {
                    Ok(rows) => rows.iter().map(|row| {
                        Ok(error_msg!(row.try_get(examples_table::ID))?)
                    }).collect::<Result<Vec<Int>, String>>(),
                    Err(error) => Err(error)
                }?;
                let translations_query = vec![
                    "select", translations_table::ID,
                    "from", translations_table::TABLE_NAME,
                    "where", table::ID, "=", &indexer.last()
                ].join(" ");
                let translations = match error_msg!(db.query(translations_query.as_str(), &[ &definition.id ])) {
                    Ok(rows) => rows.iter().map(|row| {
                        Ok(error_msg!(row.try_get(translations_table::ID))?)
                    }).collect::<Result<Vec<Int>, String>>(),
                    Err(error) => Err(error)
                }?;
                Some(Definition {
                    value: definition,
                    examples,
                    translations
                })
            },
            None => {
                return Err(format!("{}::{} Could not find row", file!(), line!()));
            }
        },
        Err(error) => {
            return Err(error);
        }
    };
    return Ok(example); 
}

pub fn get(data: Data<AppData>, info: Query<Info>) -> HttpResponse {
    match error_msg!(data.db.try_lock()) {
        Ok(mut db) => match error_msg!(get_defition(&mut db, &info.id)) {
            Ok(data) => {
                return HttpResponse::Ok().json(Reply::Ok{ data });
            },
            Err(error) => {
                println!("{}", error);
                return HttpResponse::InternalServerError().finish();
            }
        },
        Err(error) => {
            println!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
    };
}