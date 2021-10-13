use actix_web::web::{Data, HttpResponse, Query};
use serde::{Deserialize, Serialize};
use crate::{
    AppData,
    error_msg, 
    schema::{
        translations::{
            Row,
            table
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
        example: Option<Row>
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

pub fn get_translation(data: &Data<AppData>, id: &i32) -> Result<Option<Row>, String> {
    let mut db = error_msg!(data.db.try_lock())?;
    let mut indexer = ParamIndexer::new();
    let sql = vec![
        "select", "*",
        "from", table::TABLE_NAME,
        "where", table::ID, "=", &indexer.next(),
        "limit", "1"
    ].join(" ");
    let example: Option<Row> = match error_msg!(db.query(sql.as_str(), &[id])) {
        Ok(rows) => match rows.get(0) {
            Some(row) => {
                Some(error_msg!(get_translation_from_row(row))?)
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
    match error_msg!(get_translation(&data, &info.id)) {
        Ok(example) => {
            return HttpResponse::Ok().json(Reply::Ok{ example });
        },
        Err(error) => {
            println!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
    }  
}