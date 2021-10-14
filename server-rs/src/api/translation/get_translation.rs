use actix_web::web::{Data, HttpResponse, Query};
use serde::{Deserialize, Serialize};
use crate::{
    AppData,
    error_msg, 
    schema::{
        Db,
        Int,
        examples::{
            table as examples_table
        },
        translations::{
            Row,
            table,
            Translation
        },
        ParamIndexer
    },
};
use postgres::Row as PostgresRow;

#[derive(Debug, Deserialize)]
pub struct Info {
    id: i32
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Reply {
    Ok {        
        data: Option<Translation>
    },
    Error {
        error: String
    }
}

pub fn get_translation_from_row(row: &PostgresRow) -> Result<Row, String> {
    Ok(Row {
        id: error_msg!(row.try_get(table::ID))?,
        lang_id: error_msg!(row.try_get(table::LANG_ID))?,
        definition_id: error_msg!(row.try_get(table::DEFINITION_ID))?,
        definition: error_msg!(row.try_get(table::DEFINITION))?
    })
}

pub fn get_translation(db: &mut Db, id: &i32) -> Result<Option<Translation>, String> {
    let mut indexer = ParamIndexer::new();
    let sql = vec![
        "select", "*",
        "from", table::TABLE_NAME,
        "where", table::ID, "=", &indexer.next(),
        "limit", "1"
    ].join(" ");
    let translation: Option<Translation> = match error_msg!(db.query(sql.as_str(), &[id])) {
        Ok(rows) => match rows.get(0) {
            Some(row) => {
                let translation = error_msg!(get_translation_from_row(row))?;
                let mut indexer = ParamIndexer::new();
                let examples_query = vec![
                    "select", examples_table::ID,
                    "from", examples_table::TABLE_NAME,
                    "where", table::ID, "=", &indexer.next()
                ].join(" ");
                let examples = match error_msg!(db.query(examples_query.as_str(), &[ &translation.id ])) {
                    Ok(rows) => rows.iter().map(|row| {
                        Ok(error_msg!(row.try_get(examples_table::ID))?)
                    }).collect::<Result<Vec<Int>, String>>(),
                    Err(error) => Err(error)
                }?;
                Some(Translation {
                    value: translation,
                    examples
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
    return Ok(translation); 
}

pub fn get(data: Data<AppData>, info: Query<Info>) -> HttpResponse {
    match error_msg!(data.db.try_lock()) {
        Ok(mut db) => match error_msg!(get_translation(&mut db, &info.id)) {
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