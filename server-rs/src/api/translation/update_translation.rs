use actix_web::{
    web::{Data, Json},
    HttpResponse
};
use crate::{
    AppData,
    error_msg,
    schema::{
        Db,
        translations::{ Row, table },
        ParamIndexer
    }
};

pub fn update_translation(db: &mut Db, body: &Row) -> Result<(), String> {
    let mut indexer = ParamIndexer::new();
    let updates = vec![
        "set", table::LANG_ID, "=", &indexer.next(),
        "set", table::DEFINITION, "=", &indexer.next()
    ].join(",");
    let sql = vec![
        "update", table::TABLE_NAME,
        "where", table::ID, "=", &indexer.next(),
        &updates        
    ].join(" ");
    match error_msg!(db.query(sql.as_str(), &[ &body.lang_id, &body.definition, &body.id ])) {
        Ok(_rows) => Ok(()),
        Err(error) => Err(error)
    }
}

pub fn update(data: Data<AppData>, body: Json<Row>) -> HttpResponse {
    match error_msg!(data.db.try_lock()) {
        Ok(mut db) => match error_msg!(update_translation(&mut db, &body.into_inner())) {
            Ok(_) => {
                return HttpResponse::Ok().finish();
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
