use actix_web::web::{Data, HttpResponse, Query};
use serde::{Deserialize, Serialize};
use crate::{
    AppData,
    error_msg, 
    schema::{
        ParamIndexer,
        definitions::{
            Row,
            table
        }
    },
};

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

pub fn get_defition(data: Data<AppData>, id: &i32) -> Result<Option<Row>, String> {
    let mut db = error_msg!(data.db.try_lock())?;
    let mut indexer = ParamIndexer::new();
    let sql = vec![
        "select", "*", "from", table::TABLE_NAME,
        "where", table::ID, "=", &indexer.next(),
        "limit", "1"
    ].join(" ");
    let example: Option<Row> = match error_msg!(db.query(sql.as_str(), &[id])) {
        Ok(rows) => match rows.get(0) {
            Some(row) => {
                Some(Row {
                    id: error_msg!(row.try_get(table::ID))?,
                    words_group_id: error_msg!(row.try_get(table::WORDS_GROUP_ID))?,
                    cluster_id: error_msg!(row.try_get(table::CLUSTER_ID))?,
                    pronounciation: error_msg!(row.try_get(table::PRONOUNCIATION))?,
                    word: error_msg!(row.try_get(table::WORD))?,
                    prefixes: error_msg!(row.try_get(table::PREFIXES))?,
                    suffixes: error_msg!(row.try_get(table::SUFFIXES))?,
                    definition: error_msg!(row.try_get(table::DEFINITION))?
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
    match error_msg!(get_defition(data, &info.id)) {
        Ok(example) => {
            return HttpResponse::Ok().json(Reply::Ok{ example });
        },
        Err(error) => {
            println!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
    }  
}