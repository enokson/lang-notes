use actix_web::{
    web::{Data, Json},
    HttpResponse
};
use postgres::types::ToSql;
use serde::{ Serialize };
use crate::{
    AppData,
    schema::{
        ParamIndexer,
        definitions::{table, RowI}
    },
    error_msg
};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Reply {
    Ok { id: i32 },
    Err { error: String }
}

pub fn post_definition(data: Data<AppData>, body: Json<RowI>) -> Result<i32, String> {
    let mut db = error_msg!(data.db.try_lock())?;
    let mut indexer = ParamIndexer::new();
    let sql = vec![
        "insert", "into", table::TABLE_NAME,
        &format!("({})", vec![
            table::WORDS_GROUP_ID,
            table::CLUSTER_ID,
            table::PRONOUNCIATION,
            table::WORD,
            table::PREFIXES,
            table::SUFFIXES,
            table::DEFINITION
        ].join(",")),
        "values", &format!("({})", indexer.params(&7))
    ].join(" ");
    let words_group_id = 1;
    let params: [&(dyn ToSql + Sync); 7] = [
        &words_group_id, 
        &body.cluster_id, 
        &body.pronounciation,
        &body.word,
        &body.prefixes,
        &body.suffixes,
        &body.definition
    ];
    let id = match error_msg!(db.query(sql.as_str(), &params)) {
        Ok(rows) => match rows.get(0) {
            Some(row) => match error_msg!(row.try_get("id")) {
                Ok(id) => id,
                Err(error) => {
                    return Err(error);
                }
            },
            None => {
                return Err(format!("{}::{} Could not find row.", file!(), line!()));
            }
        },
        Err(error) => {
            return Err(error);
        }
    };
    return Ok(id);
}

pub fn post(data: Data<AppData>, body: Json<RowI>) -> HttpResponse {
    match error_msg!(post_definition(data, body)) {
        Ok(id) => {
            return HttpResponse::Ok().json(Reply::Ok{ id });
        },
        Err(error) => {
            println!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
