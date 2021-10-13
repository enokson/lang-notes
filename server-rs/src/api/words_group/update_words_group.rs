use actix_web::{
    web::{Data, Json},
    HttpResponse
};
use crate::{
    AppData,
    error_msg,
    schema::{
        word_groups::{ Row, table },
        ParamIndexer
    }
};

pub fn update_words_group(data: Data<AppData>, body: Json<Row>) -> Result<(), String> {
    let mut db = error_msg!(data.db.try_lock())?;
    let mut indexer = ParamIndexer::new();
    let updates = vec![
        "set", table::NAME, "=", &indexer.next()
    ].join(",");
    let sql = vec![
        "update", table::TABLE_NAME,
        "where", table::ID, "=", &indexer.next(),
        &updates        
    ].join(" ");
    match error_msg!(db.query(sql.as_str(), &[ &body.name, &body.id])) {
        Ok(_rows) => Ok(()),
        Err(error) => Err(error)
    }
}

pub fn update(data: Data<AppData>, body: Json<Row>) -> HttpResponse {
    match error_msg!(update_words_group(data, body)) {
        Ok(_) => {
            return HttpResponse::Ok().finish();
        },
        Err(error) => {
            println!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
