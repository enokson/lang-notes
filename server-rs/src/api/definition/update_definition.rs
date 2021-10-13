use actix_web::{
    web::{Data, Json},
    HttpResponse
};
use postgres::types::ToSql;
use crate::error_msg;

use super::super::super::{
    AppData, schema,
};
use schema::{
    ParamIndexer,
    definitions::{table, Row}
};

pub fn update_definition(data: Data<AppData>, body: Json<Row>) -> Result<(), String> {
    let mut db = error_msg!(data.db.try_lock())?;
    let mut indx = ParamIndexer::new();
    let updates = vec![
        "set", table::WORDS_GROUP_ID, "=", &indx.next(),
        "set", table::CLUSTER_ID, "=", &indx.next(),
        "set", table::PRONOUNCIATION, "=", &indx.next(),
        "set", table::WORD, "=", &indx.next(),
        "set", table::PREFIXES, "=", &indx.next(),
        "set", table::SUFFIXES, "=", &indx.next(),
        "set", table::DEFINITION, "=", &indx.next()
    ].join(",");
    let sql = vec![
        "update", table::TABLE_NAME,
        "where", table::ID, "=", &indx.next(),
        &updates        
    ].join(" ");
    let params: [&(dyn ToSql + Sync); 7] = [
        &body.words_group_id,
        &body.cluster_id,
        &body.pronounciation,
        &body.word,
        &body.prefixes,
        &body.suffixes,
        &body.definition
    ];
    match error_msg!(db.query(sql.as_str(), &params)) {
        Ok(_rows) => Ok(()),
        Err(error) => Err(error)
    }
}

pub fn update(data: Data<AppData>, body: Json<Row>) -> HttpResponse {
    match error_msg!(update_definition(data, body)) {
        Ok(_) => {
            return HttpResponse::Ok().finish();
        },
        Err(error) => {
            println!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
